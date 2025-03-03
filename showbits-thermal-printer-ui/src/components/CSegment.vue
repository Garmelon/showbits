<script setup lang="ts">
import { ref } from "vue";
import CSegmentCalendar from "./CSegmentCalendar.vue";
import CSegmentText from "./CSegmentText.vue";

const mode = ref<"calendar" | "text">();
</script>

<template>
  <div class="outer">
    <button v-if="mode" class="close" @click="mode = undefined">X</button>
    <CSegmentCalendar v-if="mode === 'calendar'" />
    <CSegmentText v-if="mode === 'text'" />
    <hr v-if="mode !== undefined" />
    <section>
      <p>What do you want to print?</p>
      <div class="list">
        <button @click="mode = 'calendar'">Calendar</button>
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

hr {
  border: none;
  border-top: 2px groove white;
  margin: 32px -32px;
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
