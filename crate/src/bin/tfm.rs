use dvi2html::tfm;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let font_helper = tfm::FontDataHelper::init_from_json().unwrap();
    let bytes: Vec<u8> = bincode::serialize(font_helper.get_data()).unwrap();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&bytes[..]).unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    {
        let mut file = File::create("src/tfmdata").unwrap();
        file.write_all(compressed_bytes.as_slice());
    }
    println!("{}", "tfmdata file written");
}
