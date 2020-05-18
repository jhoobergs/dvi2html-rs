use flate2::write::ZlibDecoder;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

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

#[derive(Debug)]
pub struct FontDataHelper {
    data: HashMap<String, FontData>,
}

impl FontDataHelper {
    //TODO: change error type
    pub fn init_from_json() -> Result<FontDataHelper> {
        let json = fs::read_to_string("fontsfull.json").expect("Failed to read the font json file");
        //let json = get_json();
        let data = serde_json::from_str(&json)?;
        Ok(FontDataHelper { data })
    }
    pub fn init() -> Result<FontDataHelper> {
        let buffer = include_bytes!("tfmdata");
        let mut decoded = Vec::new();
        let mut z = ZlibDecoder::new(decoded);
        z.write_all(buffer).unwrap(); //TODO
        decoded = z.finish().unwrap();
        let data = bincode::deserialize(&decoded[..]).unwrap(); //TODO
        Ok(FontDataHelper { data })
    }
    pub fn get(&self, font_name: String) -> Option<&FontData> {
        self.data.get(&font_name)
    }
    pub fn get_data(&self) -> &HashMap<String, FontData> {
        &self.data
    }
}
