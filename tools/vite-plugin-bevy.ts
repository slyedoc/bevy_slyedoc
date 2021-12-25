// After trying https://github.com/lencx/vite-plugin-rsw
// > not recommended

// I started to write my first plugin for vite

// If your new to rust and wasm, even if you have a understanding of js, you can get lost
// in needing to learn the different between:
//     
//     wasm-pack
//     wasm-bindgen
//     wasm-bindgen-cli

// For our use case, we are basiclly passing config options
// though 3 obstractions layers before anything useful happens:
// --out-dir: where `cargo build` the file
// --out-name: what you told cargo build call it
// --profile: dev | release | profiling
// --target web

// Found this after learning most of this the hardway https://www.reddit.com/r/rust/comments/kd22u5/wasmpack_dissectionhow_to_work_with_wasmbindgen/
// for more info, though I disagree on authors in that we I dont care about npm functionallity
// because we can just tell vite where the files are

// I found this later https://github.com/nshen/vite-plugin-wasm-pack
// its far better than rsw, though is basicly similar

import chalk from 'chalk';
import fs from 'fs-extra';
import path from 'path';
import { Plugin } from 'vite'

interface Crate {
    name: string;
    path: string;
}

interface UserConfig {
    crates: Crate[]
}

export function WasmPlugin(userConfig: UserConfig): Plugin {

    // TODO:  We dont really do anything anymore, just check the files
    // are where we think they should be currently, using public folder
    const prefix = '@vite-plugin-wasm@';
    let config_base: string;
    let config_assetsDir: string;

    return {
        name: 'vite-plugin-wasm',
        enforce: 'pre',
        config: () => ({
            // server: {
            //     host: '0.0.0.0',
            //     hmr: {
            //         host: devip()[0]
            //     }
            // }
        }),
        configResolved(resolvedConfig) {
            config_base = resolvedConfig.base;
            config_assetsDir = resolvedConfig.build.assetsDir;
        },

        resolveId(id: string) {
            for (let i = 0; i < userConfig.crates.length; i++) {
                if (path.basename(userConfig.crates[i].name) === id) return prefix + id;
            }
            return null;
        },
        async load(id: string) {
            if (id.indexOf(prefix) === 0) {
                id = id.replace(prefix, '');
                const modulejs = path.join(
                    './node_modules',
                    id,
                    id.replace(/\-/g, '_') + '.js'
                );
                const code = await fs.promises.readFile(modulejs, {
                    encoding: 'utf-8'
                });
                return code;
            }
        },
        async buildStart(_inputOptions) {

            userConfig.crates.forEach(crate => {
                if (!fs.existsSync(crate.path)) {
                    console.error(
                        chalk.bold.red('Error: ') +
                        `Can't find ${chalk.bold(crate.path)}, run ${chalk.bold.red(
                            `cargo run -p wasm`
                        )} first`
                    );
                }

                // replace default load path with '/assets/xxx.wasm'
                // const jsName = crate.name.replace(/\-/g, '_') + '.js';
                // const jsPath = path.join( crate.path, jsName);
                // const regex = /input = new URL\('(.+)'.+;/g;
                // let code = fs.readFileSync(path.resolve(jsPath), { encoding: 'utf-8' });
                // code = code.replace(regex, (_match: any, group1: string) => {
                //     return `input = "${path.posix.join(
                //         config_base,
                //         config_assetsDir,
                //         group1
                //     )}"`;
                // });
                // fs.writeFileSync(jsPath, code);
            })
        },

        configureServer({ middlewares }) {
            return () => {
                // send 'root/pkg/xxx.wasm' file to user
                middlewares.use((req, res, next) => {
                    if (typeof (req.url) == "string") {
                        console.log(req.url);
                        //const basename = path.basename(req.url);
                        // res.setHeader(
                        //     'Cache-Control',
                        //     'no-cache, no-store, must-revalidate'
                        // );
                        // const entry = wasmMap.get(basename);
                        // if (basename.endsWith('.wasm') && entry) {
                        //     res.writeHead(200, { 'Content-Type': 'application/wasm' });
                        //     fs.createReadStream(entry.path).pipe(res);
                        // } else {
                        next();
                        //}
                    }
                });
            };
        },

        buildEnd() {
            // copy xxx.wasm files to /assets/xxx.wasm
            // wasmMap.forEach((crate, fileName) => {
            //     this.emitFile({
            //         type: 'asset',
            //         fileName: `assets/${fileName}`,
            //         source: fs.readFileSync(crate.path)
            //     });
            // });
        },
        // generateBundle() {
        //     userConfig.crates.forEach((i: Crate) => {
        //       this.emitFile({
        //         fileName: i.name,
        //         type: 'asset',
        //         source: (i.source as Uint8Array),
        //       });
        //     })
        //   }
    };
}
