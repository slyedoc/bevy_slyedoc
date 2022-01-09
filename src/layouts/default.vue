<script setup lang="ts">
import { ref } from 'vue'
import { wasm_crates } from 'virtual:vue-bevy/generated-wasms'

const theme = ref('dark')
const drawer = ref(false)
const links = [
  { name: 'Home', path: '/', icon: 'mdi-home' },
  { name: 'About', path: '/about', icon: 'mdi-information' },

];

const toggleTheme = () => theme.value = theme.value === 'light' ? 'dark' : 'light'

</script>

<template>
  <v-app :theme="theme">
    <v-app-bar>
      <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>

      <v-app-bar-title>Bevy Slyedoc</v-app-bar-title>

      <v-spacer></v-spacer>

      <v-btn class="mr-3" @click="toggleTheme()">
        <v-icon icon="mdi-theme-light-dark" />
      </v-btn>

      <v-btn class="mr-4 ml-5" color="primary" plain>
        <v-icon left icon="mdi-handshake-outline"></v-icon>

        <span>John Leider</span>
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer app v-model="drawer">
      <v-list>
        <v-list-item v-for="(link, i) in links" :key="i" @click="$router.push({ path: link.path })" >
          <v-list-item-icon>
            <v-icon>{{ link.icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title>{{ link.name }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <!-- <v-list-item @click="$router.push({ path: '/' })">
          <v-list-item-icon>
            <v-icon>mdi-home</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title>Home</v-list-item-title>
          </v-list-item-content>
        </v-list-item> -->

        <v-list-item
          v-for="(crate, i) in wasm_crates"
          :key="i"
          @click="$router.push({ path: '/' + crate.name })"
        >
          <v-list-item-icon>
            <v-icon>mdi-home</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title>{{ crate.name }}</v-list-item-title>
            <v-list-item-subtitle>{{ crate.description }}</v-list-item-subtitle>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <v-container fluid>
        <router-view />
      </v-container>
    </v-main>
    <v-footer app></v-footer>
  </v-app>
</template>

