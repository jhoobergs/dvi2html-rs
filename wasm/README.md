<div align="center">

  <h1><code>dvi2html-wasm</code></h1>

  <strong>A dvi2html converter written in Rust and ported to WebAssembly using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

</div>

[![npm version](https://badge.fury.io/js/dvi2html-wasm.svg)](https://badge.fury.io/js/dvi2html-wasm)

## About
- NPM package to convet dvi files (as Uint8Array) to html
- It uses the dvi2html rust package (see [crate](https://github.com/jhoobergs/dvi2html-rs/tree/master/crate) folder)
- Install with `npm i dvi2html-wasm` or `yarn add dvi2html-wasm`

## Example
```js
import {dvi2html} from "dvi2html-wasm";

fetch('http://localhost:8080/main.dvi')
  .then(res => res.blob())
  .then(b => b.arrayBuffer())
  .then(b => {
    console.log(b);
    console.time("Generating html");
    let result = dvi2html(new Uint8Array(b))
    console.timeEnd("Generating html");
    console.time("Adding to body");
    let child = document.createElement('div');
    child.innerHTML= result;
    document.body.appendChild(child);
    console.timeEnd("Adding to body");
  });
```

## Development

### Commands

#### Build with `wasm-pack build`

```
wasm-pack build
```

#### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

#### Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

### Info

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
* [tutorials](https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html)
* [template-docs](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html)
