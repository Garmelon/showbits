<script setup lang="ts">
const { message } = defineProps<{ message?: string }>();
</script>

<template>
  <p v-if="message">{{ message }}</p>
</template>

<style scoped>
p {
  background-color: #fdd;
  color: #400;
  padding: 0 2px;
}
</style>
