use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct FontCharacterMetrics {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FontData {
    pub characters: HashMap<u32, FontCharacterMetrics>,
    pub smallest_character_code: u32,
    pub largest_character_code: u32,
    pub design_size: u32,
}

pub struct FontDataHelper {
    data: HashMap<String, FontData>,
}

impl FontDataHelper {
    pub fn init() -> Result<FontDataHelper> {
        let json = fs::read_to_string("fontsfull.json").expect("Failed to read the font json file");
        Ok(FontDataHelper {
            data: serde_json::from_str(&json)?,
        })
    }
    pub fn get(&self, font_name: String) -> Option<&FontData> {
        self.data.get(&font_name)
    }
}
