import {dvi2html} from "dvi2html-wasm";

fetch('http://localhost:8080/two_page_tikz.dvi')
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




