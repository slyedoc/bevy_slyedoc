
<script setup lang="ts">
const gpu = (navigator as any).gpu;

</script>
<template>
  <template v-if="!gpu">
    Web Gpu is not supported on your borswer.
    See
    <a href="https://github.com/gpuweb/gpuweb/wiki/Implementation-Status">Implementation Status</a> a for wgpu status.
    Broswers that do support WebGPU are:
    <ol>
      <li>
        <a href="https://www.chromium.org/getting-involved/dev-channel">Chrome Dev Channel</a>
        You will need to
        <a href="https://web.dev/gpu/#use">enable</a> it as well though chrome flags.
      </li>
      <li>
        <a href="https://www.google.com/chrome/canary/">Chrome Canary</a>(windows only)
      </li>
      <li>
        <a href="https://www.mozilla.org/en-US/firefox/developer/">Firefox Developer</a>
      </li>
      <li>Safari - pretty sure is in dev mode but don't have a mac to test on.</li>
    </ol>
  </template>
</template>
