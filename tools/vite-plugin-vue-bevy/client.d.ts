
interface Crate {
    name: string;
    description: string;
    version: string;
}

declare module 'virtual:vue-bevy/generated-wasms' {
    const wasm_crates: Crate[]
}