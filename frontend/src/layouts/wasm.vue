<script setup lang="ts">
import { wasm_crates } from '~/wasm';

</script>


<template>

  <div class="sidenav px-4 py-10 text-center text-gray-700 dark:text-gray-200">
    <!-- <router-link class="icon-btn mx-2" :to="`/crates/${name}`" :title="name"> -->

    <!-- </router-link> -->
      <nav class="text-xl mt-6" v-for="name in wasm_crates">

        <router-link class="icon-btn mx-2" :to="`/crates/${name}`" :title="name">
          {{ name }}
        </router-link>
      </nav>
  </div>

  <main class="main px-4 py-10 text-center text-gray-700 dark:text-gray-200">
    <router-view />
    <Footer />
    <div class="mt-5 mx-auto text-center opacity-25 text-sm">
      [Wasm Layout]
    </div>
  </main>
</template>
