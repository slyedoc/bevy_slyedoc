<script setup lang="ts">
// Generated File
import snakeReadme from '../../crates/snake/readme.md'
import init  from '~/wasm/snake'
const gpu = (navigator as any).gpu;

tryOnMounted(async () => {
    if (gpu) {
        const wasm =  await init();
        wasm.run();
    }
});

const router = useRouter()
const { t } = useI18n()
</script>

<template>
    <snakeReadme />
    <template v-if="!gpu">
        <p class="text-sm mt-4">
        WebGPU not supported! Please visit
        <a href="//webgpu.io">webgpu.io</a> to see the current implementation
        status.
        </p>
    </template>
    <template v-if="gpu">
        Make sure canvas has focus <br />
        Hit F12 for editor<br />
        <canvas class="wasm" />
    </template>

    <button class="btn m-3 text-sm mt-6" @click="router.back()">
    {{ t("button.back") }}
    </button>

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