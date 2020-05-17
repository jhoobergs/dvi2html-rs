<div align="center">

  <h1><code>dvi2html-wasm</code></h1>

  <strong>A dvi2html converter written in Rust and ported to WebAssembly using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

</div>

## About

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
* [tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
* [template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html
