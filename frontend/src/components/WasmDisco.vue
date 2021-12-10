<template>
  <div>
    <div v-if="!gpu">
      WebGPU not supported! Please visit <a href="//webgpu.io">webgpu.io</a> to see the current implementation status.
    </div>
    <canvas class="wasm"></canvas>
  </div>
</template>
<style scoped>
  .wasm {
    padding-left: 0;
    padding-right: 0;
    margin-left: auto;
    margin-right: auto;
    display: block;
    width: 800px;
  }
</style>
<script lang="ts">
import init from 'disco'
import { defineComponent, onMounted } from 'vue'

export default defineComponent({
  name: 'WasmDisco',
  props: {
    gpu: Boolean
  },
  setup() {
    let gpu = (navigator as any).gpu;
    console.log('GPU is supported!')

    onMounted( async () => {
      console.log('Component is mounted!')
      
      if (gpu) {
        let wasm = await init();
        console.log('Init done')
        wasm.run();
      }

    })
    return {
      gpu
    }
  },
})
</script>
