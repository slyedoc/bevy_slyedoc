/* eslint-disable no-console */
import path from 'path'
import fs from 'fs'
import { defineConfig } from 'vite'
import Vue from '@vitejs/plugin-vue'
import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'
import Components from 'unplugin-vue-components/vite'
import AutoImport from 'unplugin-auto-import/vite'
import Markdown from 'vite-plugin-md'
import WindiCSS from 'vite-plugin-windicss'
import { VitePWA } from 'vite-plugin-pwa'
import VueI18n from '@intlify/vite-plugin-vue-i18n'
import Inspect from 'vite-plugin-inspect'
import Prism from 'markdown-it-prism'
import LinkAttributes from 'markdown-it-link-attributes'
import ViteRsw from 'vite-plugin-rsw'

const markdownWrapperClasses = 'prose prose-sm m-auto text-left'

export default defineConfig(async ({ command, mode }) => {

    // Build the list of wasms for crates list and for json so we can use it client side
    // TODO: Learn to create my own vite plugin, and put this there
    const packages_path = `${path.resolve(__dirname, 'crates')}`
    const wasm_names = fs.readdirSync(packages_path).filter((file) => {
        if (!file.startsWith('.') && !file.startsWith('engine'))
        // Ignore hidden folders and engine
            return fs.statSync(`${packages_path}/${file}`).isDirectory()
        return false
    });
    let src_dir = `${path.resolve(__dirname, 'src')}`
    let wasm_dir = `${path.resolve(src_dir, 'wasm')}`
    if (!fs.existsSync(wasm_dir)) {
        fs.mkdirSync(wasm_dir);
    }
    fs.writeFileSync(`${path.resolve(wasm_dir, 'list.json')}`, JSON.stringify(wasm_names));
    // TODO: find better way, was using a [name].vue file but since assetFolderSettings not working
    // I need use folder stucture differently
    // create index.vue files for each wasm,
    wasm_names.map((name) => {
        let dir = `${path.resolve(packages_path, name)}`
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir);
        }
        fs.writeFileSync(`${path.resolve(dir, 'index.vue')}`, [
            
            `<script setup lang="ts">`,
            `    const wasm_name = "${name}"`,
            `</script>`,
            `<template>`,
            `    <bevy-wasm :name="wasm_name" />`,
            `</template>`,
            `<route lang="yaml">`,
            `meta:`,
            `  layout: wasm`,
            `</route>`,
        ].join('\n'));
   })

    return {
        resolve: {
            alias: {
                '~/': `${src_dir}/`,
            },
        },
        // assetsInclude: [
        //     `${packages_path}/**/assets/*.png`,
        //     `${packages_path}/**/assets/*.jpg`],
        plugins: [

            // https://github.com/lencx/vite-plugin-rsw#plugin-options
            // https://rustwasm.github.io/docs/wasm-pack/commands/build.html
            ViteRsw({
                cli: 'pnpm',// 'npm', 'pnpm'
                profile: mode === 'development' ? 'dev' : 'release', // 'dev' | 'release' | 'profiling'
                target: 'web', // 'bundler' | 'web' | 'nodejs' | 'no-modules'
                unwatch: [`${src_dir}`],
                crates: wasm_names.map((name) => {
                    return {
                        name: `crates/${name}`,
                        //outDir: `${wasm_dir}/${name}`,
                    }
                }),
            }),

            Vue({
                include: [/\.vue$/, /\.md$/],
            }),

            // https://github.com/hannoeru/vite-plugin-pages
            Pages({
                extensions: ['vue', 'md'],
            }),

            // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
            Layouts(),

            // https://github.com/antfu/unplugin-auto-import
            AutoImport({
                imports: [
                    'vue',
                    'vue-router',
                    'vue-i18n',
                    '@vueuse/head',
                    '@vueuse/core',
                ],
                dts: 'src/auto-imports.d.ts',
            }),

            // https://github.com/antfu/unplugin-vue-components
            Components({
                // allow auto load markdown components under `./src/components/`
                extensions: ['vue', 'md'],

                // allow auto import and register components used in markdown
                include: [/\.vue$/, /\.vue\?vue/, /\.md$/],

                // custom resolvers
                resolvers: [
                    // auto import icons
                    // https://github.com/antfu/unplugin-icons
                    IconsResolver({
                        componentPrefix: '',
                        // enabledCollections: ['carbon']
                    }),
                ],

                dts: 'src/components.d.ts',
            }),

            // https://github.com/antfu/unplugin-icons
            Icons({
                autoInstall: true,
            }),

            // https://github.com/antfu/vite-plugin-windicss
            WindiCSS({
                safelist: markdownWrapperClasses,
            }),

            // https://github.com/antfu/vite-plugin-md
            // Don't need this? Try vitesse-lite: https://github.com/antfu/vitesse-lite
            Markdown({
                wrapperClasses: markdownWrapperClasses,
                headEnabled: true,
                markdownItSetup(md) {
                    // https://prismjs.com/
                    // @ts-expect-error types mismatch
                    md.use(Prism)
                    // @ts-expect-error types mismatch
                    md.use(LinkAttributes, {
                        pattern: /^https?:\/\//,
                        attrs: {
                            target: '_blank',
                            rel: 'noopener',
                        },
                    })
                },
            }),

            // https://github.com/antfu/vite-plugin-pwa
            VitePWA({
                registerType: 'autoUpdate',
                includeAssets: ['favicon.svg', 'robots.txt', 'safari-pinned-tab.svg'],
                manifest: {
                    name: 'Vitesse',
                    short_name: 'Vitesse',
                    theme_color: '#ffffff',
                    icons: [
                        {
                            src: '/pwa-192x192.png',
                            sizes: '192x192',
                            type: 'image/png',
                        },
                        {
                            src: '/pwa-512x512.png',
                            sizes: '512x512',
                            type: 'image/png',
                        },
                        {
                            src: '/pwa-512x512.png',
                            sizes: '512x512',
                            type: 'image/png',
                            purpose: 'any maskable',
                        },
                    ],
                },
            }),

            // https://github.com/intlify/bundle-tools/tree/main/packages/vite-plugin-vue-i18n
            VueI18n({
                runtimeOnly: true,
                compositionOnly: true,
                include: [path.resolve(__dirname, 'locales/**')],
            }),

            // https://github.com/antfu/vite-plugin-inspect
            Inspect({
                // change this to enable inspect for debugging
                enabled: false,
            }),


        ],

        server: {
            fs: {
                strict: true,
            },
        },

        // https://github.com/antfu/vite-ssg
        ssgOptions: {
            script: 'async',
            formatting: 'minify',
        },

        optimizeDeps: {
            include: [
                'vue',
                'vue-router',
                '@vueuse/core',
                '@vueuse/head',
            ],
            exclude: [
                'vue-demi',
            ],
        },
    }
})
