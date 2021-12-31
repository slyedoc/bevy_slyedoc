/* eslint-disable no-console */
import chalk from 'chalk';
import fs from 'fs-extra';
import path from 'path'
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
import { BevyWasm } from './tools/vite-plugin-vue-bevy';

const markdownWrapperClasses = 'prose prose-sm m-auto text-left'

export default defineConfig(async ({ command, mode }) => {

    let src_dir = `${path.resolve(__dirname, 'src')}`;

    return {
        //root: frontend,
        // build: {
        //     //outDir: '../dist',
        //     emptyOutDir: true,
        // },
        resolve: {
            alias: {
                '~/': `${src_dir}/`,
            },
        },
        plugins: [

            // our plugin
            BevyWasm({
                crates: ["./crates/**/Cargo.toml"],
                // out_dir is relative to each crate, wasm-pack issue
                out_dir: `${src_dir}/wasm`,
                out_dir_dist: `${src_dir}/wasm`,
                watch_debounce: 5000,
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
                dts: `${src_dir}/auto-imports.d.ts`,
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

                dts: `${src_dir}/components.d.ts`,
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
                    md.use(Prism)
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
                include: [`locales/**`],
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