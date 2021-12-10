<script setup lang="ts">
// https://github.com/vueuse/head
// you can use this to manipulate the document head in any components,
// they will be rendered correctly in the html results with vite-ssg
useHead({
  title: 'Twin Games',
  meta: [
    { name: 'description', content: 'Web Games' },
  ],
})
</script>

<template>
  <router-view />
</template>
