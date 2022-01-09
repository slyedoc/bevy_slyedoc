// register vue composition api globally
import { ViteSSG } from 'vite-ssg'
import generatedRoutes from 'virtual:generated-pages'
import { setupLayouts } from 'virtual:generated-layouts'
// your custom styles here
import './styles/main.css'
import '@mdi/font/css/materialdesignicons.css'

import App from './App.vue'



const routes = setupLayouts(generatedRoutes)

const webFontLoader = await import(/* webpackChunkName: "webfontloader" */'webfontloader')

webFontLoader.load({
  google: {
    families: ['Roboto:100,300,400,500,700,900&display=swap'],
  },
})

// https://github.com/antfu/vite-ssg
export const createApp = ViteSSG(
  App,
  { routes },
  (ctx) => {
    // install all modules under `modules/`
    Object.values(import.meta.globEager('./modules/*.ts')).map(i => i.install?.(ctx))
  },
)
