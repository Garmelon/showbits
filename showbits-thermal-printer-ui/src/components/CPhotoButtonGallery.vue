<script setup lang="ts">
import { RiMultiImageFill } from "@remixicon/vue";
import { useTemplateRef } from "vue";

const emit = defineEmits<{
  click: [file: File];
}>();

const image = useTemplateRef<HTMLInputElement>("image");

function onImageChange() {
  const theFile = image.value?.files?.[0];
  if (theFile === undefined) return;
  if (!theFile.type.startsWith("image/")) return;
  emit("click", theFile);
}
</script>

<template>
  <label>
    <RiMultiImageFill size="48px" />
    <input ref="image" type="file" accept="image/*" @change="onImageChange" />
  </label>
</template>

<style scoped>
label {
  padding: 8px;

  border: 5px solid var(--button-fg);
  border-radius: 100px;
  background-color: transparent;

  color: var(--button-fg);
}

label:active {
  color: var(--button-active-fg);
  background-color: var(--button-active-bg);
}

svg {
  display: block;
}

input {
  display: none;
}
</style>
