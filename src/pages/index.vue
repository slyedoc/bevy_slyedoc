<script setup lang="ts">
import HelloWorld from '../../README.md'
import { wasm_crates } from 'virtual:@vue-bevy/generated-wasms';

const { t } = useI18n()

// export default {
//   components: {
//     HelloWorld,
//   },
// }
</script>

<template>
    <nav class="text-s" v-for="crate in wasm_crates">
      <router-link class="mx-2" :to="`/crates/${crate.name}`" :title="crate.name">
        {{ crate.name }}
        {{ crate.description }}
      </router-link>
    </nav>

  <HelloWorld />




    <p class="text-4xl">
      <carbon-campsite class="inline-block" />
    </p>
    <p>
      <a rel="noreferrer" href="https://github.com/antfu/vitesse" target="_blank">
        Vitesse
      </a>
    </p>
    <p>
      <em class="text-sm opacity-75">{{ t('intro.desc') }}</em>
    </p>

    <div class="py-4" />
</template>

<route lang="yaml">
meta:
  layout: wasm
</route>
