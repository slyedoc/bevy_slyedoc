<script setup lang="ts">

</script>

<template>
  <v-card>
    <v-footer>
      <v-card
        flat
        tile
        width="100%"
        class="text-center"
      >
        <v-card-text>
          <v-btn
            class="mx-4"
            icon
          >
            <mdi-handshake-outline />
          </v-btn>
        </v-card-text>

        <v-divider></v-divider>

        <v-card-text class="">
          {{ new Date().getFullYear() }} — <strong>Twin Games</strong>
        </v-card-text>
      </v-card>
    </v-footer>
  </v-card>
</template>
