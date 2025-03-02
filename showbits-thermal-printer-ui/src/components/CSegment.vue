<script setup lang="ts">
import { ref } from "vue";
import CSegmentText from "./CSegmentText.vue";

const mode = ref<"text">();
</script>

<template>
  <CSegmentText v-if="mode === 'text'" />
  <section v-else class="choose">
    <p>What do you want to print?</p>
    <div class="list">
      <button @click="mode = 'text'">Calendar</button>
      <button @click="mode = 'text'">Cellular Automaton</button>
      <button @click="mode = 'text'">Chat Message</button>
      <button @click="mode = 'text'">Easter Egg</button>
      <button @click="mode = 'text'">Text</button>
      <button @click="mode = 'text'">Tic Tac Toe</button>
    </div>
  </section>
</template>

<style scoped>
p {
  margin-bottom: 1ch;
}

.list {
  display: flex;
  flex-wrap: wrap;
  gap: 1ch;
}
</style>
