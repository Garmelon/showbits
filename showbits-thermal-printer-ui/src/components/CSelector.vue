<script setup lang="ts">
import { ref } from "vue";
import CDocumentCalendar from "./CDocumentCalendar.vue";
import CDocumentCells from "./CDocumentCells.vue";
import CDocumentChat from "./CDocumentChat.vue";
import CDocumentEgg from "./CDocumentEgg.vue";
import CDocumentImage from "./CDocumentImage.vue";
import CDocumentText from "./CDocumentText.vue";
import CDocumentTictactoe from "./CDocumentTictactoe.vue";

const mode = ref<
  "calendar" | "chat" | "cells" | "egg" | "image" | "text" | "tictactoe"
>();
</script>

<template>
  <div class="outer">
    <button v-if="mode" class="close" @click="mode = undefined">X</button>
    <CDocumentCalendar v-if="mode === 'calendar'" />
    <CDocumentChat v-if="mode === 'chat'" />
    <CDocumentCells v-if="mode === 'cells'" />
    <CDocumentEgg v-if="mode === 'egg'" />
    <CDocumentImage v-if="mode === 'image'" />
    <CDocumentText v-if="mode === 'text'" />
    <CDocumentTictactoe v-if="mode === 'tictactoe'" />
    <hr v-if="mode !== undefined" />
    <section>
      <p>What do you want to print?</p>
      <div class="list">
        <button @click="mode = 'calendar'">Calendar</button>
        <button @click="mode = 'cells'">Cellular Automaton</button>
        <button @click="mode = 'chat'">Chat Message</button>
        <button @click="mode = 'egg'">Easter Egg</button>
        <button @click="mode = 'image'">Image</button>
        <button @click="mode = 'text'">Text</button>
        <button @click="mode = 'tictactoe'">Tic Tac Toe</button>
        <a href="/photo.html">Take a photo</a>
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
  align-items: baseline;
  gap: 1ch;
}
</style>
