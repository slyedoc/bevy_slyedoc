import { UserModule } from '~/types'

// Setup Vuetify
// export const install: UserModule = ({ app }) => {
//   const vuetify = createVuetify({
//     components,
//     directives,
//   })

//   app.use(vuetify)
// }

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
