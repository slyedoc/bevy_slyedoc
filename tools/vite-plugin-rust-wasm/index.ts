

import chalk from 'chalk';
import fs from 'fs-extra';
import path from 'path';
import chokidar from 'chokidar';
import fg from 'fast-glob';
import mime from 'mime-types';
import toml from 'toml';
import child  from 'child_process';
import { Plugin } from 'vite';

interface Options {
    url_folder: string; // wasm
    crates: string[];
    out_dir: string;
}

interface Crate {
    name: string;
    description: string;
    version: string;
    path: string;
}

export function WasmPlugin(userOptions: Options): Plugin {
    const virtualWasmsModuleId = 'virtual:generated-wasms'
    const crates: Crate[] = [];
    let root = "";

    return {
        name: 'vite-plugin-rust-wasm',
        configResolved(config) {
            root = config.root;
        },
        async buildStart(_inputOptions) {
            // create complete crate list, expand on * pattern if needed
            const crate_list: string[] = [];
            userOptions.crates.forEach((crate_path) => {
                if (crate_path.endsWith('*')) {
                    // search path for folders
                    const parent_dir = `${path.resolve(root, crate_path.substring(0, crate_path.length - 1))}`
                    fs.readdirSync(parent_dir).forEach((file) => {
                        // Ignore hidden folders and engine
                        const child_dir = `${path.resolve(parent_dir, file)}`;
                        if (!file.startsWith('.') && fs.statSync(child_dir).isDirectory()) {
                            crate_list.push(child_dir);
                        }
                    });
                } else {
                    // add to list
                    crate_list.push(crate_path);
                }
            });

            // collect crate info and add to list
            crate_list.forEach((crate_path) => {
                // look for cargo.toml file
                const cargo_path = `${path.resolve(crate_path, 'Cargo.toml')}`;
                if (fs.existsSync(cargo_path)) {

                    // read cargo.toml file add info to crates
                    const tomlFile = fs.readFileSync(cargo_path, { encoding: 'utf-8' });
                    const cargo = toml.parse(tomlFile);

                    crates.push({
                        name: cargo.package.name,
                        description: cargo.package.description,
                        version: cargo.package.version,
                        path: crate_path,
                    });
                } else {
                    console.log(chalk.red(`Crate ${crate_path} does not have a Cargo.toml file`));
                }
            });

            // add watches
            crates.forEach(async (crate) => {
                const query = [
                    `${path.resolve(crate.path, 'Cargo.toml' )}`,
                    `${path.resolve(crate.path, 'src' )}/**/*`,
                    `${path.resolve(crate.path, 'assets' )}/**/*`
                ]
                const files = await fg(query);
                for(let file of files){
                    this.addWatchFile(file);
                    //this.emitFile(file.substring(crate.path.length + 1), file);
                }
            });
        },
        resolveId(id) {
            // virtual list of wasms
            if (id === virtualWasmsModuleId) {
                return id;
            }
            // } else if ( id.indexOf(userOptions.url_folder) === 0) {
            //     return id;
            // }
            return null;
        },
        async load(id) {
            // load wasm list as virtual module
            if (id === virtualWasmsModuleId) {
                // dont leak path to frontend
                let wasm_list_client = crates.map(c => {
                    return {
                        name: c.name,
                        description: c.description,
                        version: c.version
                    };
                });
                return 'export const wasm_crates = ' + JSON.stringify(wasm_list_client);
            }


            // if (id.indexOf(userOptions.url_folder) === 0) {
            //     console.log(id);
            //     id = id.replace(userOptions.url_folder, '');
            //     const file_path = path.join(
            //         root,
            //         userOptions.out_dir,
            //         id,
            //     );
            //     console.log(file_path)
            //     const code = await fs.promises.readFile(file_path, {
            //         encoding: 'utf-8'
            //     });
            //     return code;
            // }


            return null; // other ids should be handled as usually
        },
        configureServer({ middlewares }) {
            return () => {
                middlewares.use((req, res, next) => {
                    if (typeof (req.url) === 'string' && req.url.startsWith(userOptions.url_folder)) {
                        // load wasm, index.js and assets files
                        const url = req.url.substring(userOptions.url_folder.length + 1);
                        const dirs = path.dirname(url).split('/');
                        const crate_name = dirs[0];
                        const file_path = url.substring(crate_name.length + 1);
                        const file_name = path.basename(url);
                        const crate = crates.filter(c => c.name === crate_name )[0];
                        if (crate) {
                            if (dirs.length === 1 && file_name === 'index.js') {
                                res.writeHead(200, { 'Content-Type': 'application/javascript' });
                                fs.createReadStream(path.resolve(crate.path, userOptions.out_dir, file_name)).pipe(res);
                            } else if (dirs.length === 1 && file_name === 'index_bg.wasm') {
                                res.writeHead(200, { 'Content-Type': 'application/wasm' });
                                fs.createReadStream(path.resolve(crate.path, userOptions.out_dir, file_name)).pipe(res);
                            } else if (dirs.length > 1 && dirs[1] === 'assets') {
                                res.writeHead(200, { 'Content-Type': mime.contentType(file_name) as string });
                                fs.createReadStream(path.resolve(crate.path, file_path)).pipe(res);
                            } else {
                                next();
                            }
                        } else {
                            next();
                        }
                    } else {
                        next();
                    }
                });
            };
        },
    };
}



