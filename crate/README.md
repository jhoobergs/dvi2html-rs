<div align="center">

  <h1><code>dvi2html</code></h1>

  <strong>A dvi2html converter written in Rust.</strong>

</div>

## About
- Rust crate to convert dvi files to html
- Based on https://github.com/kisonecat/dvi2html

## Example
```rust
let mut input_owned = Vec::new();                                       
File::open("file.dvi")                               
     .unwrap()                                                           
     .read_to_end(&mut input_owned)                                      
     .unwrap();                                                          
println!("{}", dvi2html(&input_owned).unwrap());
```

## Development

### Commands

#### Build

```
cargo build
```

#### Test 

```
cargo test
```
