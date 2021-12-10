
/**
 * Resolver for Vuetify v3 - https://github.com/antfu/vitesse/issues/85
 *
 * @link https://github.com/vuetifyjs/vuetify
 */

// Vuetify
import { createVuetify } from 'vuetify'
import * as directives from 'vuetify/directives'
import * as components from 'vuetify/components'
import { UserModule } from '~/types'
// Styles
import 'vuetify/styles'

// Setup Vuetify
export const install: UserModule = ({ app }) => {
  const vuetify = createVuetify({
    components,
    directives,
  })

  app.use(vuetify)
}

// TODO: Couldn't get the VuetifyResolver to work so loads all components, fine for now,
// wait to vuetify is V3 is out before worrying about it.

/*
export function VuetifyResolver(): any {
  return {
    type: 'component',
    resolve: (name: string) => {
      if (name.match(/^V[A-Z]/))
        return { importName: name, path: 'vuetify/components' }
    },
  }
}
*/
