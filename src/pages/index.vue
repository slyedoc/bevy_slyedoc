<script setup lang="ts">
import list from '~/wasm/list.json'

const { t } = useI18n()
</script>

<template>
  <div>
    <nav class="text-s" v-for="name in list">

      <router-link class="mx-2" :to="`/wasm/${name}`" :title="name">
        {{ name }}
      </router-link>
    </nav>

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


    
  </div>
</template>

<route lang="yaml">
meta:
  layout: wasm
</route>
