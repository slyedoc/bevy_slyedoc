
interface Crate {
    name: string;
    description: string;
    version: string;
}

declare module 'virtual:generated-wasms' {
// eslint-disable-next-line import/no-duplicates


const wasm_crates: Crate[]
//export default routes
}