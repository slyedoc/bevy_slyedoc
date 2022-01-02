
interface Crate {
    name: string;
    description: string;
    version: string;
}

declare module 'virtual:@vue-bevy/generated-wasms' {
    const wasm_crates: Crate[]
    
}

declare module 'virtual:@vue-bevy/*' {
    export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
}
