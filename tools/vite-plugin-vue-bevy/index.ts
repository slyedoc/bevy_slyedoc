

import chalk from 'chalk';
import fs from 'fs-extra';
import path from 'path';
//import chokidar from 'chokidar';
import fg from 'fast-glob';
//import mime from 'mime-types';
import toml from 'toml';
import child_process from 'child_process';
import { Plugin, ResolvedConfig } from 'vite';

interface Options {
    crates: string[];
    out_dir: string;
    out_dir_dist: string;
    watch_debounce?: number;
}

interface Crate {
    name: string;
    description: string;
    version: string;
    path: string;
}

export function BevyWasm(options: Options): Plugin {
    const virtualModuleId = 'virtual:@vue-bevy';
    const resolvedVirtualModuleId = '\0' + virtualModuleId;
    let crates: Crate[] = [];
    let config: ResolvedConfig;

    return {
        name: 'vite-plugin-vue-bevy',
        configResolved(viteConfig) {
            config = viteConfig;
        },
        async buildStart(_inputOptions) {
            // create crate list
            crates = await getCrates(config, options);

            // build crates
            await buildCrates(crates, config, options);
            
            // build wasm bindings
            await buildBindings(crates, config, options);

            //await copyReadmes(crates, config, options);

            createPages(crates);
            // add watches
            // crates.forEach((crate) => {
            //      this.addWatchFile(crate.path);
            // });
        },
        resolveId(id) {
            if (id.indexOf(virtualModuleId) === 0) {
                return id.replace(virtualModuleId, resolvedVirtualModuleId);
            }
            return null;
        },
        async load(id) {
            if (id.indexOf(resolvedVirtualModuleId) === 0) {
                id = id.replace(resolvedVirtualModuleId, '');
                console.log(chalk.gray("[vue-bevy]"), "Load: ", chalk.bold.whiteBright(id));
                if (id === '/generated-wasms') {
                    return 'export const wasm_crates = ' + JSON.stringify(crates.map(c => {
                        return {
                            name: c.name,
                            description: c.description,
                            version: c.version
                        };
                    }));
                }
            }
        },
    };
}

function createPages(crates: Crate[]) {
    crates.forEach(async (crate) => {
        const file = path.resolve('src/pages', `${crate.name}.vue`);
        fs.writeFile(file,
`<script setup lang="ts">
import ${crate.name}Readme from './../.${crate.path}/readme.md'
import init  from '~/wasm/${crate.name}'
const gpu = (navigator as any).gpu;

tryOnMounted(async () => {
    //const { default: init } = await import('~/wasm/disco')
    if (gpu) {
    const wasm =  await init();
    console.log('Init done');
    wasm.run();
    }
});

const router = useRouter()
const { t } = useI18n()
</script>

<template>
    <div>
    <template v-if="!gpu">
        <p class="text-sm mt-4">
        WebGPU not supported! Please visit
        <a href="//webgpu.io">webgpu.io</a> to see the current implementation
        status.
        </p>
    </template>
    <template v-if="gpu">
        <canvas class="wasm" />
    </template>
    <${crate.name}Readme />
    <div>
        <button class="btn m-3 text-sm mt-6" @click="router.back()">
        {{ t("button.back") }}
        </button>
    </div>
    </div>
</template>

<style scoped>
.wasm {
    margin-left: auto;
    margin-right: auto;
}
</style>

<route lang="yaml">
meta:
    layout: wasm
</route>`);
    });
}

async function getCrates(config: ResolvedConfig, options: Options): Promise<Crate[]> {
    const files = await fg(options.crates);
    return files.map((file) => {
        const cargo_dir = path.dirname(file);
        // read cargo.toml file add info to crates
        const cargo = toml.parse(fs.readFileSync(file, { encoding: 'utf-8' }));
        return {
            name: cargo.package.name,
            description: cargo.package.description,
            version: cargo.package.version,
            path: cargo_dir,
        };
    });
}

async function buildCrates(crates: Crate[], config: ResolvedConfig, options: Options) {
    crates.forEach(async (crate) => {
        let cmd_str = `cargo
                build
                --lib
                --package ${crate.name}
                --target wasm32-unknown-unknown
            `;
            if (config.command === "build") {
                cmd_str += "--release";
            }
        let cmd = cmd_str.split(' ').filter(a => a !== '').map(a => a.trim());
        console.log(cmd);
        console.log(chalk.gray("[vue-bevy]"), 'Cargo building ', chalk.bold.whiteBright(crate.name));
        child_process.spawnSync(cmd.shift() as string, cmd, {
            stdio: 'inherit'
        });
    });
}

async function buildBindings(crates: Crate[], config: ResolvedConfig, options: Options) {
    crates.forEach(async (crate) => {
        let outDir = config.command === "serve" ? path.resolve(config.root, "src/wasm") : path.resolve(config.root, "src/wasm_dist");
        let targetDir = config.command === "serve" ? "debug" : "release";
        let cmd_str = `wasm-bindgen
                ./target/wasm32-unknown-unknown/${targetDir}/${crate.name}.wasm
                --out-dir ${outDir}
                --out-name ${crate.name}
                --target web
            `;
        let cmd = cmd_str.split(' ').filter(a => a !== '').map(a => a.trim());
        console.log(chalk.gray("[vue-bevy]"), 'Wasm-bindgen building', chalk.bold.whiteBright(crate.name));
        //console.log(cmd);
        child_process.spawnSync(cmd.shift() as string, cmd, {
            stdio: 'inherit'
        });
    });
}

// TODO: delete this
async function copyReadmes(crates: Crate[], config: ResolvedConfig, options: Options) {
    crates.forEach(async (crate) => {
        const file = path.resolve(crate.path, 'readme.md');
        if (fs.existsSync(file)) {
            fs.writeFile(path.resolve(options.out_dir, `${crate.name}.md`), fs.readFileSync(file));
        } else {
            console.log(chalk.gray("[vue-bevy]"), 'No readme.md found for', chalk.bold.whiteBright(crate.name));
        }
    });
}


