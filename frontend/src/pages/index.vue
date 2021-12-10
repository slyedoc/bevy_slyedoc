<script setup lang="ts">
</script>

<template>
  <v-card class="elevation-11">
    <v-card-title primary-title>
      Home
    </v-card-title>
    <v-card-text>
      Welcome to the start of Twin Games!

      <br />
      <router-link to="/cornell_box">
        <v-btn color="primary">
          Cornell Box
        </v-btn>
      </router-link>
      <router-link to="/disco">
        <v-btn color="primary">
          Disco
        </v-btn>
      </router-link>
    </v-card-text>
  </v-card>
</template>

<route lang="yaml">
meta:
  layout: default
</route>
