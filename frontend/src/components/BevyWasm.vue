
<script setup lang="ts">
import { wasm_crates } from '~/wasm';
const props = defineProps<{ name: string }>()

const valid = wasm_crates.filter(item => item === props.name)
const router = useRouter()
const gpu = (navigator as any).gpu;

watch( () => props.name, async (selection, prevSelection) => { 
    console.log(
        "Watch props.name function called with args:",
        selection,
        prevSelection
      );
  await load_wasm()
})


onMounted(async () => {
  console.log("Component is mounted!");

  if (gpu && valid) {
    await load_wasm()
  }


});

async function load_wasm() {
  // using dynamic_imports
  const { default: init } = await import(`./../../wasm/${props.name}/index.js`);
  const wasm = await init();
  console.log('Init done');
  wasm.run();
}

const { t } = useI18n()
</script>

<template>
  <div>
    <!-- <p class="text-4xl">
      <carbon-pedestrian class="inline-block" />
    </p> -->
    <p>
      {{ props.name }}
    </p>

    <p class="text-sm opacity-50">
      <em>{{ t("intro.dynamic-route") }}</em>
    </p>

    <template v-if="!gpu">
      <p class="text-sm mt-4">
        WebGPU not supported! Please visit
        <a href="//webgpu.io">webgpu.io</a> to see the current implementation
        status.
      </p>
    </template>

    <template v-if="gpu">
      <canvas class="wasm" />
    </template>

    <template v-if="!valid">
      Sorry that wasm doesn't exist!
    </template>


    <div>
      <button class="btn m-3 text-sm mt-6" @click="router.back()">
        {{ t("button.back") }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.wasm {
  /* padding-left: 0;
  padding-right: 0;
  */
  margin-left: auto;
  margin-right: auto;
  display: block; 
  width: 100%;
  height: 100%;
  pointer-events: none;
}
</style>

<route lang="yaml">
meta:
  layout: wasm
</route>
