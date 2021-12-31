<script setup lang="ts">
import Readme from '../../README.md'

const { t } = useI18n()
</script>

<template>
  <Readme />
  <p class="text-4xl">
    <carbon-campsite class="inline-block" />
  </p>
  <p>
    <a rel="noreferrer" href="https://github.com/antfu/vitesse" target="_blank">Vitesse</a>
  </p>
  <p>
    <em class="text-sm opacity-75">{{ t('intro.desc') }}</em>
  </p>
</template>

<route lang="yaml">
meta:
  layout: wasm
</route>
