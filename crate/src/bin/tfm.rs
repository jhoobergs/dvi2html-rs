use dvi2html::tfm;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let font_helper = tfm::FontDataHelper::init_from_json().unwrap();
    let bytes: Vec<u8> = bincode::serialize(font_helper.get_data()).unwrap();
    {
        let mut file = File::create("tfmdata").unwrap();
        file.write_all(bytes.as_slice());
    }
    println!("{}", "tfmdata file written");
}
