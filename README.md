# dvi2html-rs

Dvi to html converter written in Rust.
Based on https://github.com/kisonecat/dvi2html

Next steps: create a webassembly version with wasm-pack that uses this crate

## TODO
- Improve error handling: use Result type much more and don't use unwrap's
- Go through all TODO's in the code (mostly unsafe unwrap's)
- Better solution for the json file with the fonts? Use the ts code to generate a .rs file (simple function that returns json as string, currently this function is create manually)

