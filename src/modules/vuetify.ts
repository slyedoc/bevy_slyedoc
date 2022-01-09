
import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/lib/iconsets/mdi'
import type { UserModule } from '~/types'

// Setup Vueify
// https://next.vuetifyjs.com/en/getting-started/installation#usage
export const install: UserModule = ({ isClient, initialState, app }) => {
    const vuetify = createVuetify({
        theme: {
            defaultTheme: 'dark'
        },
        icons: {
            defaultSet: 'mdi',
            aliases,
            sets: {
              mdi,
            }
          },
    });
    app.use(vuetify)
}
