use dvi2html::FontData;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let json = fs::read_to_string("fontsfull.json").expect("Failed to read the font json file");
    let data: HashMap<String, FontData> = serde_json::from_str(&json).unwrap();
    let bytes: Vec<u8> = bincode::serialize(&data).unwrap();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&bytes[..]).unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    {
        let mut file = File::create("src/tfmdata").unwrap();
        file.write_all(compressed_bytes.as_slice());
    }
    println!("tfmdata file written");
}
