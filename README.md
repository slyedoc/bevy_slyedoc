# Bevy Slyedoc

Personal Playground using bevy main branch, vite, and vue.  Lots of new tech to play with.

Been getting several community examples working, will try to link to their sources.
## Project


- Crates - Several Bevy example crates that can be run local or served
- Engine - Bevy Plugin for shared setup and tooling
  - Editor, simple bevy_egui toolbar and WorldInspector
- Tools
  - vite-plugin-vue-bevy -  My first vite plugin, after having problems with 2 community plugins, I wanted to understand vite better, so started my own
    - trying to keep it general use, but will have to come back to that
    - not using hmr as well as I would like, still work in progress
- Broken - Serveral Bevy examples I have had working at one point or another, most require rapier, mold is the one I really want to get worked again, its works locally

## Hit list

- Wasm not getting killed, pretty sure there can be serveral copies of a wasm running at the same time
- Get [Vueify](https://next.vuetifyjs.com/en) working with vue3 (using enough beta libs, will wait for release in Feb)
- Fix camera controller cursor grab logic on wasm
- Fine better solution to canvas and window sizes
  - thinking either web-sys or vue + wasmbinding, maybe setup a bevy fullscreen option that would just default scale with element
  - Will likely use [ResizeObserver](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
- See about Bundling for bevy assets, sponza takes like 30 secs to load it's all HTTP requests

## Browsers

Did some testing getting this running on firebox and chrome, and chrome wins hands on features and has 60% market share, for now just going to target it.  Firefox worked last check though.

See [Implementation Status](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status) for wgpu support

You will most likely need Chrome Canary on Windows or a Dev build of Chrome
- [Chrome Dev Downloads](https://www.chromium.org/getting-involved/dev-channel)
  - [Enable Instructions](https://web.dev/gpu/#use) for chrome flags
- [Canary](https://www.google.com/chrome/canary/) - again Windows Only

## Setup

Requires [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html#the-wasm-bindgen-command-line-interface), make sure version matches use in crates

To run locally:

```bash
cargo run --release -p <name>
```

Name is any crate in ./crates/*

To run frontend:

```bash
pnpm install
vite
```

To build for deployment:

```bash
vite build
```
