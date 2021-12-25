<script setup lang="ts">
  const props = defineProps<{ name: string }>()
</script>

<template>
  <bevy-wasm :name="props.name" />
</template>

<route lang="yaml">
meta:
  layout: wasm
</route>
