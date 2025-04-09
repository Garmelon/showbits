<script setup lang="ts">
import { RiCameraSwitchFill } from "@remixicon/vue";
</script>

<template>
  <button>
    <RiCameraSwitchFill size="48px" />
  </button>
</template>

<style scoped>
button {
  padding: 8px;

  color: var(--button-fg);
  border: 5px solid var(--button-fg);
  border-radius: 100px;
  background-color: transparent;
}

button:enabled:active {
  color: var(--button-active-fg);
  background-color: var(--button-active-bg);
}

button:disabled {
  color: var(--button-disabled-fg);
  border-color: var(--button-disabled-fg);
}

svg {
  display: block;
}
</style>
