import chalk from 'chalk';
import fs from 'fs-extra';
import path from 'path';
import chokidar from 'chokidar';
import fg from 'fast-glob';
//import mime from 'mime-types';
import toml from 'toml';
import child_process from 'child_process';
import { Plugin, ResolvedConfig } from 'vite';
import { useDebounceFn } from '@vueuse/core'
interface Options {

    crates: string[];
    out_dir: string;
    out_dir_dist: string;
    dts: string;
    wasm_opt: boolean;
}

interface Crate {
    name: string;
    description: string;
    version: string;
    path: string;
}

// Note: No support for watch on vite build
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
            crates = await getCrates();

            crates.forEach(async (crate) => {
                if (config.command === 'serve') {
                    // reusable function to build a crate
                    const crateGenerate = (crate: Crate) => {

                        // Step 1: Build crate
                        const cmd = `cargo build --lib --package ${crate.name} --target wasm32-unknown-unknown`;
                        const cmd_parts = cmd.split(' ');
                        console.log(chalk.gray("[vue-bevy]"), `Running 'cargo build' for `, chalk.bold.whiteBright(crate.name));
                        const cargo_build = child_process.spawnSync(cmd_parts.shift() as string, cmd_parts, {
                            stdio: 'inherit'
                        });

                        if (cargo_build.status !== 0) {
                            console.error(chalk.red("[vue-bevy]"), `Cargo build failed for ${crate.name}`);
                        }

                        // Step 2: generate wasm bindings
                        const bindgen_cmd = ['wasm-bindgen',
                            `./target/wasm32-unknown-unknown/debug/${crate.name}.wasm`,
                            '--out-dir', `${path.resolve(config.root, options.out_dir)}`,
                            '--out-name', `${crate.name}`,
                            '--target', 'web'];

                        console.log(chalk.gray("[vue-bevy]"), 'Wasm-bindgen building', chalk.bold.whiteBright(crate.name));
                        const wasm_bindgen = child_process.spawnSync(bindgen_cmd.shift() as string, bindgen_cmd, {
                            stdio: 'inherit'
                        });
                        if (wasm_bindgen.status !== 0) {
                            console.error(chalk.red("[vue-bevy]"), `Cargo build failed for ${crate.name}`);
                        }

                        // TODO: send hmr update
                    }
                    // called first time here on start
                    crateGenerate(crate);

                    const deboundBuild = useDebounceFn(() => {
                        crateGenerate(crate);
                    }, 5000);
                    // watch the crate and rebuild on change
                    // chokidar.watch(crate.path).on('all', (event, path) => {
                    //     // wrap crateGenerate in debounce so we can limit how often try to build
                    //    deboundBuild()
                    // });

                    // assets
                    // add alias for asset paths, so no coping is needed
                    // IMPORTANT: path needs to match bevy AssetServerSettings
                    config.resolve.alias.push({
                        find: `/assets/${crate.name}`,
                        replacement: `${crate.path}/assets/`
                    });
                } else {

                    // run cargo build
                    let cmd = ['cargo', 'build', '--package', crate.name, '--lib',  '--target',  'wasm32-unknown-unknown', '--release'];
                    console.log(chalk.gray("[vue-bevy]"), 'Cargo building ', chalk.bold.whiteBright(crate.name));
                    console.log(cmd.join(' '));
                    child_process.spawnSync(cmd.shift() as string, cmd, {
                        stdio: 'inherit'
                    });

                    // run wasm-bindings
                    let binding_cmd = ['wasm-bindgen',
                        `./target/wasm32-unknown-unknown/release/${crate.name}.wasm`,
                        '--out-dir', `${path.resolve(config.root, options.out_dir_dist)}`,
                        '--out-name', crate.name,
                        '--target', 'web'];
                    console.log(chalk.gray("[vue-bevy]"), 'Wasm-bindgen building', chalk.bold.whiteBright(crate.name));
                    console.log(binding_cmd.join(' '));
                    child_process.spawnSync(binding_cmd.shift() as string, binding_cmd, {
                        stdio: 'inherit'
                    });

                    // run wasm-opt if enabled
                    if (options.wasm_opt) {
                        let wasm_opt_cmd = [
                            'wasm-opt',
                            '-Os',
                            '--enable-simd',
                            '--output', `${options.out_dir_dist}/${crate.name}.wasm`,
                            `${options.out_dir_dist}/${crate.name}_bg.wasm`
                        ];

                        console.log(chalk.gray("[vue-bevy]"), 'Wasm-opt building', chalk.bold.whiteBright(crate.name));
                        console.log(wasm_opt_cmd.join(' '));
                        child_process.spawnSync(wasm_opt_cmd.shift() as string, wasm_opt_cmd, {
                            stdio: 'inherit'
                        });
                    }

                    // // emit wasm file and js file
                    // [ '.js', options.wasm_opt ? '.wasm': '_bg.wasm' ].forEach(ext => {
                    //     this.emitFile({
                    //         type: 'asset',
                    //         fileName: `assets/${crate.name}${ext}`,
                    //         source: fs.readFileSync(`${options.out_dir_dist}/${crate.name}${ext}`),
                    //     });
                    // });

                    // assets - alias for asset paths, no copy needed
                    // IMPORTANT: path needs to match bevy AssetServerSettings
                    let assets = await fg(path.resolve(crate.path, 'assets', '**/*'), {
                        onlyFiles: true,
                    });
                    assets.forEach((file) => {
                        let relative_path = path.relative(path.resolve(crate.path, 'assets'), file);
                        // emit asset files for build system
                        this.emitFile({
                            type: 'asset',
                            fileName: `assets/${crate.name}/${relative_path}`,
                            source: fs.readFileSync(file),
                        });
                    });
                    // for (let file of assets) {


                    // }
                }
            });

            // TODO: move this or find better way
            // doesn't really belong here, but vite.config.ts could fould need crates list
            createPages(crates);



            // FIXME: move this to vite.config.ts
            function createPages(crates: Crate[]) {
                crates.forEach(async (crate) => {
                    const readme_file = path.resolve(crate.path, 'readme.md');
                    if (!fs.existsSync(readme_file)) {
                        console.warn(chalk.gray("[vue-bevy]"), `No readme found in ${crate.name}`);
                    }
                    const file = path.resolve('src/pages', `${crate.name}.vue`);
                    fs.writeFile(file,
                        `<script setup lang="ts">
// Generated File
import ${crate.name}Readme from '../.${crate.path}/readme.md'
import init  from '~/${ path.relative( "./src/", config.command === 'serve' ? options.out_dir : options.out_dir)}/${crate.name}'
const gpu = (navigator as any).gpu;

tryOnMounted(async () => {
    if (gpu) {
        const wasm =  await init();
        wasm.run();
    }
});

const router = useRouter()
const { t } = useI18n()
</script>

<template>
    <${crate.name}Readme />
    <template v-if="!gpu">
        <p class="text-sm mt-4">
        WebGPU not supported! Please visit
        <a href="//webgpu.io">webgpu.io</a> to see the current implementation
        status.
        </p>
    </template>
    <template v-if="gpu">
        Make sure canvas has focus <br />
        Hit F12 for editor<br />
        <canvas class="wasm" />
    </template>

    <button class="btn m-3 text-sm mt-6" @click="router.back()">
    {{ t("button.back") }}
    </button>

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

            async function getCrates(): Promise<Crate[]> {
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
                if (id === '/generated-wasms') {
                    return 'export const wasm_crates = ' + JSON.stringify(crates.map(c => {
                        return {
                            name: c.name,
                            description: c.description,
                            version: c.version
                        };
                    }));
                }
                // crates.forEach(crate => {
                //     if (id.indexOf(`/${crate.name}`) === 0) {
                //         const file = path.resolve(options.out_dir, `${crate.name}.js`);
                //         return {
                //             //ast?: AcornNode;
                //             code: fs.readFileSync(file, { encoding: 'utf-8' }),
                //             //map?: SourceMapInput;
                //         };
                //     }
                // });
            }
        },
        async buildEnd() {
            console.log("build end")
        }

    };
}






export function checkMtime(
    dirs: string,
    cargoToml: string,
    benchmarkFile: string,
    runCallback: Function,
    optimCallback: Function,
) {
    try {
        // benchmark file modified time
        const pkgMtime = fs.statSync(benchmarkFile).mtimeMs;
        const cargoMtime = fs.statSync(cargoToml).mtimeMs;
        let isOptim = true;

        // run wasm-pack
        if (cargoMtime > pkgMtime) {
            isOptim = false;
            return runCallback();
        }

        (function dirsMtime(dir) {
            for (let f of fs.readdirSync(dir)) {
                const _f = fs.statSync(`${dir}/${f}`);

                if (_f.isDirectory()) {
                    if (_f.mtimeMs > pkgMtime) {
                        // run wasm-pack
                        isOptim = false;
                        runCallback();
                        break;
                    } else {
                        dirsMtime(`${dir}/${f}`)
                    }
                }

                if (_f.isFile()) {
                    if (_f.mtimeMs > pkgMtime) {
                        // run wasm-pack
                        isOptim = false;
                        runCallback();
                        break;
                    }
                }
            }
        })(dirs)

        isOptim && optimCallback();
    } catch (e) {
        // no such file or directory
        runCallback();
    }
}