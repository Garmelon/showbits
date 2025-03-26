<script setup lang="ts">
import { RiMultiImageFill } from "@remixicon/vue";
</script>

<template>
  <label>
    <RiMultiImageFill size="48px" />
  </label>
</template>

<style scoped>
label {
  padding: 8px;

  border: 5px solid white;
  border-radius: 100px;
  background-color: transparent;

  color: white;
}

label:active {
  background-color: white;
  color: black;
}

svg {
  display: block;
}
</style>
