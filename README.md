# Bevy Slyedoc

Personal Playground using bevy main branch, vite, and vue.

Been getting several community examples working, will try to link to their sources.

## Project

- Crates - Several Bevy example crates that can be run local or served
- Engine - Bevy Plugin for shared setup and tooling
  - Editor, simple EGUI toolbar and WorldInspector
- Tools
  - vite-plugin-rust-wasm: My first vite plugin, after having problems with 2 community plugins, I wanted to understand vite better, so started my own.

## Todo

- [ ] Kill wasm on route change, think is problem is due partly due to dynamic import hack
- [ ] Fine better solution to canvas and window sizes
  - thinking either web-sys or vue + wasmbinding, was hoping fullscreen in bevy would just default scale with element, no luck
  - Will likely use [ResizeObserver](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
- [X] Remove 'Pack' tool, and figure out how to use watch correctly in vite plugin
- [ ] Vite build asset bundles and dynamic import issues
- [ ] Bundle for bevy assets, sponza takes like 30 secs to load it's all HTTP requests
- [ ] Use crate readme to generate content
- [ ] Get [Vueify](https://next.vuetifyjs.com/en) working with vue3 (don't feel like learning another CSS framework)

## Browsers

Did some testing getting this running on firebox and chrome, and chrome wins hands on features and has 60% market share, for now just going to target it.

You will need canary or dev build of Chrome currently
- [Downloads](https://www.chromium.org/getting-involved/dev-channel)
- [Instructions](https://web.dev/gpu/#use) for enabling

## Setup
Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

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
