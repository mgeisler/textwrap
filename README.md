# Textwrap WebAssembly Demo

An example showing how Textwrap can be used to wrap text in a HTML
canvas. Both monospace and proportional fonts can be used since we use
the browser to measure the pixel width of each word as it is rendered
to the canvas.

## Links

* **Live demo:
  [mgeisler.github.io/textwrap/](https://mgeisler.github.io/textwrap/).**
  Here you can try the demo. It is automatically deployed on every
  merge to the `master` branch.

* **Source code:
  [`examples/wasm/`](https://github.com/mgeisler/textwrap/tree/master/examples/wasm).**
  This is the Rust code which make up the demo. You will also find
  some JavaScript and HTML glue code.

* **GitHub Action:
  [`build.yml`](https://github.com/mgeisler/textwrap/blob/master/.github/workflows/build.yml).**
  This is the script which compiles and deploys the code. We use
  [`wasm-pack`](https://github.com/rustwasm/wasm-pack) to easily
  compile Textwrap and its dependencies to Wasm.

* **Deployment branch:
  [`gh-pages`](https://github.com/mgeisler/textwrap/tree/gh-pages).**
  The compiled Wasm code is pushed to this branch. The branch might be
  squashed from time to time if it grows too big.
