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

  color: white;
  border: 5px solid white;
  border-radius: 100px;
  background-color: transparent;
}

button:enabled:active {
  color: black;
  background-color: white;
}

button:disabled {
  --disabled: #444;
  color: var(--disabled);
  border-color: var(--disabled);
}

svg {
  display: block;
}
</style>
