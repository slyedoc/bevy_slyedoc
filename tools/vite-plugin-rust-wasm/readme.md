# Vite Plugin Rust Wasm

My first vite plugin, so take that into account before using.

If you are new to rust and wasm, you kinda need to learn what each of the following do:

- wasm-pack
- wasm-bindgen
- wasm-bindgen-cli

For our use case, we are basically passing config options
though 3 abstractions layers before anything useful happens because I don't care about publishing wasms on npm.  Those options are:
--out-dir: where `cargo build` the file
--out-name: what you told cargo build call it
--profile: dev | release | profiling
--target web

I found this after learning most of this the hard way [Work With Wasm-bindgen](https://www.reddit.com/r/rust/comments/kd22u5/wasmpack_dissectionhow_to_work_with_wasmbindgen/) for an overview.  I disagree on author on one point though, I don't care about npm functionality
because we can just tell vite where the files.

There are 2 vite plugins I tried before starting my own.

- [vite-plugin-wasm-pack](https://github.com/nshen/vite-plugin-wasm-pack)
   The better of the 2
- [vite-plugin-rsw](https://github.com/lencx/vite-plugin-rsw)
   Supports multiple crates (but you have to run the cargo build yourself or use "rsw-node" tool which just does the cargo build)
   Has a custom cargo dependency watcher, that tries to detect cargo dependency changes as well setup though.

## Other rust plugins for refs

- [rollup-plugin-rust](https://github.com/wasm-tool/rollup-plugin-rust/)
   This one is pretty nice
