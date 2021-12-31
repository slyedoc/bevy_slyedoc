<script setup lang="ts">
import snakeReadme from './../../crates/snake/readme.md'
import init  from '~/wasm/snake'
const gpu = (navigator as any).gpu;

tryOnMounted(async () => {
    //const { default: init } = await import('~/wasm/disco')
    if (gpu) {
    const wasm =  await init();
    console.log('Init done');
    wasm.run();
    }
});

const router = useRouter()
const { t } = useI18n()
</script>

<template>
    <div>
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
    <snakeReadme />
    <div>
        <button class="btn m-3 text-sm mt-6" @click="router.back()">
        {{ t("button.back") }}
        </button>
    </div>
    </div>
</template>

<style scoped>
.wasm {
    margin-left: auto;
    margin-right: auto;
}
</style>

<route lang="yaml">
meta:
    layout: wasm
</route>