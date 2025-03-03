<script setup lang="ts">
import { ref } from "vue";
import CSegmentCalendar from "./CSegmentCalendar.vue";
import CSegmentEgg from "./CSegmentEgg.vue";
import CSegmentText from "./CSegmentText.vue";
import CSegmentTictactoe from "./CSegmentTictactoe.vue";
import CSegmentCells from "./CSegmentCells.vue";

const mode = ref<"calendar" | "cells" | "egg" | "text" | "tictactoe">();
</script>

<template>
  <div class="outer">
    <button v-if="mode" class="close" @click="mode = undefined">X</button>
    <CSegmentCalendar v-if="mode === 'calendar'" />
    <CSegmentCells v-if="mode === 'cells'" />
    <CSegmentEgg v-if="mode === 'egg'" />
    <CSegmentText v-if="mode === 'text'" />
    <CSegmentTictactoe v-if="mode === 'tictactoe'" />
    <hr v-if="mode !== undefined" />
    <section>
      <p>What do you want to print?</p>
      <div class="list">
        <button @click="mode = 'calendar'">Calendar</button>
        <button @click="mode = 'cells'">Cellular Automaton</button>
        <button @click="mode = 'text'">Chat Message</button>
        <button @click="mode = 'egg'">Easter Egg</button>
        <button @click="mode = 'text'">Text</button>
        <button @click="mode = 'tictactoe'">Tic Tac Toe</button>
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
  color: white;
  background-color: black;
  border: 2px solid black;
  padding: 0 4px;
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
