<script setup lang="ts">
import { ref } from "vue";
import CSegmentText from "./CSegmentText.vue";

const mode = ref<"text">();
</script>

<template>
  <div class="outer">
    <button v-if="mode" class="close" @click="mode = undefined">X</button>
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
  </div>
</template>

<style scoped>
.outer {
  position: relative;
}

.close {
  position: absolute;
  top: 0;
  right: 0;

  width: 24px;
  height: 24px;

  color: white;
  background-color: black;

  border: 2px solid black;
  border-radius: 100px;

  display: flex;
  align-items: center;
  justify-content: center;
}

.close:hover {
  background-color: #444;
}

.close:active {
  background-color: #666;
}

p {
  margin-bottom: 1ch;
}

.list {
  display: flex;
  flex-wrap: wrap;
  gap: 1ch;
}
</style>
