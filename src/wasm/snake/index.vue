<script setup lang="ts">
    const wasm_name = "snake"
</script>
<template>
    <bevy-wasm :name="wasm_name" />
</template>
<route lang="yaml">
meta:
  layout: wasm
</route>