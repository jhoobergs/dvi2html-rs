use crate::machine::Executor;
use crate::machine::Machine;
use dvi::Instruction;
use serde_json::Result;

mod htmlmachine;
mod machine;
mod tfm;
mod utils;

pub use crate::tfm::{FontCharacterMetrics, FontData};

// Copied from https://github.com/derekdreery/dvi-rs/blob/master/tests/lib.rs
// Adapted to new version of nom
fn parse_dvi(input: &[u8]) -> Vec<Instruction> {
    let mut input = input;
    let mut instructions = Vec::new();
    while !input.is_empty() {
        let instruction = match Instruction::parse(&input) {
            Ok((i, inst)) => {
                input = i;
                inst
            }
            _ => panic!("Parse error"),
        };
        instructions.push(instruction);
    }
    instructions
}

pub fn dvi2html(input: &[u8]) -> Result<String> {
    let font_helper = tfm::FontDataHelper::init()?;
    let mut machine = htmlmachine::HTMLMachine::new();
    let instructions = parse_dvi(input);
    let special_handlers: Vec<machine::SpecialHandler> = vec![
        Box::new(htmlmachine::special_html_svg),
        Box::new(htmlmachine::special_html_color),
        Box::new(htmlmachine::special_html_papersize),
    ];
    for ins in instructions.iter() {
        machine.execute(ins, &font_helper, &special_handlers);
    }

    Ok(machine.get_content())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    // test copied from https://github.com/derekdreery/dvi-rs/blob/master/tests/lib.rs
    #[test]
    fn dvi_works() {
        let mut input_owned = Vec::new();
        File::open("testfiles/main.dvi")
            .unwrap()
            .read_to_end(&mut input_owned)
            .unwrap();
        let instructions = parse_dvi(&input_owned);
        // e.g.
        assert!(
            instructions[instructions.len() - 1]
                == Instruction::PostPost {
                    post_pointer: 1826,
                    ident: 2,
                    two_two_three: 5
                }
        );
    }
    #[test]
    fn dvi2html_works() {
        let mut input_owned = Vec::new();
        File::open("testfiles/main.dvi")
            .unwrap()
            .read_to_end(&mut input_owned)
            .unwrap();
        println!("{}", dvi2html(&input_owned).unwrap());
        //assert!(true == false);
    }
    #[test]
    fn dvi2html_color_special_works() {
        let mut input_owned = Vec::new();
        File::open("testfiles/color.dvi")
            .unwrap()
            .read_to_end(&mut input_owned)
            .unwrap();
        println!("{}", dvi2html(&input_owned).unwrap());
    }
    #[test]
    fn dvi2html_color_pagesize_two_pages_works() {
        let mut input_owned = Vec::new();
        File::open("testfiles/two_page.dvi")
            .unwrap()
            .read_to_end(&mut input_owned)
            .unwrap();
        println!("{}", dvi2html(&input_owned).unwrap());
    }
    #[test]
    fn dvi2html_color_pagesize_two_pages_tikz_works() {
        let mut input_owned = Vec::new();
        File::open("testfiles/two_page_tikz.dvi")
            .unwrap()
            .read_to_end(&mut input_owned)
            .unwrap();
        println!("{}", dvi2html(&input_owned).unwrap());
    }
}
