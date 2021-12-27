# Bevy Slyedoc

Personal Playground using bevy main branch, vite, and vue.

Been getting several community examples working on main, will try to link to their sources.

## Project

- Crates - Serveral Bevy Example Crates that can be run local or compiled to wasm32
- Engine -
- Tools
  - vite-plugin-rust-wasm: My first vite plugin, after having problmes with 2 community plugins, I wanted to understand vite better, so started my own by writting my own.

## Todo

- [ ] Kill wasm on page change
- [ ] Fine better solution to window resize, really should be provided to bevy on element resize then use limit to prevent texuture size limit
  - Will likely use [ResizeObserver](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
  - Surprised I haven't found any decent examples
- [ ] Remove 'Pack' tool, and figure out how to use watch correctly in vite
- [ ] Figure out way to bundle assets into a bundle, sponza

## Browsers

Did some testing getting this running on firebox and chrome, and chrome wins hands on features and has 60% market share, for now just going to target it.

You will need canary or dev build of Chrome currently
- [Downloads](https://www.chromium.org/getting-involved/dev-channel)
- [Instructions](https://web.dev/gpu/#use) for enabling

## Setup

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```
pnpm install
cargo run -p pack // TODO: remove this once watch is working
vite
```
