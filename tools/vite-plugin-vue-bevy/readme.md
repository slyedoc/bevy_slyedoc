# Vite Plugin Rust Wasm

My first vite plugin, so take that into account before using.

## Exports

- virtual/vue-bevy/generated_wasms: List of generated crates
- virtual/vue-bevy/<crate_name>/index.js
- virtual/vue-bevy/<crate_name>/index_bg.wasm

## Notes

[Wasm Size](https://rustwasm.github.io/docs/book/reference/code-size.html)

There are 2 vite plugins I tried before starting my own.

- [vite-plugin-wasm-pack](https://github.com/nshen/vite-plugin-wasm-pack)
- [vite-plugin-rsw](https://github.com/lencx/vite-plugin-rsw)

I did get both working, rsw seems more complicated than was needed, the other didn't do as much, and neither used vite well, and both required manual steps, though both did work

- [Evan You interview](https://fullstackradio.com/140) - This interview was so helpful in understanding vite use of node modules resolution system, its dated but is making me rethink how important node modules are so vite can do more

Other references

- [rollup-plugin-rust](https://github.com/wasm-tool/rollup-plugin-rust/)

## Wasm-Pack

If you are new to rust and wasm, you kinda need to learn what each of the following do:

- wasm-pack
- wasm-bindgen
- wasm-bindgen-cli
- wasm-opt
  
For our use case, we are basically passing config options
though 3 abstractions layers before anything useful happens because I don't care about publishing wasms on npm.  Those options are:

--out-dir: where `cargo build` the file
--out-name: what you told cargo build call it
--profile: dev | release | profiling
--target web

I found this after learning most of this the hard way [Work With Wasm-bindgen](https://www.reddit.com/r/rust/comments/kd22u5/wasmpack_dissectionhow_to_work_with_wasmbindgen/) for an overview.  I disagree on author on one point though, I don't care about npm functionality
because we can just tell vite where the files.


## Setup

Requires [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)

```bash
cargo install -f wasm-bindgen-cli
```

> TODO: Feels bad having this step, though as you dive though wasm-bindgen stack
involved
