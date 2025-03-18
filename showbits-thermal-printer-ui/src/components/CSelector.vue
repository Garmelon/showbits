<script setup lang="ts">
import { ref } from "vue";
import CDocumentCalendar from "./CDocumentCalendar.vue";
import CDocumentCells from "./CDocumentCells.vue";
import CDocumentChat from "./CDocumentChat.vue";
import CDocumentEgg from "./CDocumentEgg.vue";
import CDocumentImage from "./CDocumentImage.vue";
import CDocumentSunrise from "./CDocumentSunrise.vue";
import CDocumentText from "./CDocumentText.vue";
import CDocumentTictactoe from "./CDocumentTictactoe.vue";
import CDocumentXkcd from "./CDocumentXkcd.vue";

const mode = ref<
  | "calendar"
  | "cells"
  | "chat"
  | "egg"
  | "image"
  | "sunrise"
  | "text"
  | "tictactoe"
  | "xkcd"
>();
</script>

<template>
  <div class="outer">
    <button v-if="mode" class="close" @click="mode = undefined">X</button>
    <CDocumentCalendar v-if="mode === 'calendar'" />
    <CDocumentCells v-if="mode === 'cells'" />
    <CDocumentChat v-if="mode === 'chat'" />
    <CDocumentEgg v-if="mode === 'egg'" />
    <CDocumentImage v-if="mode === 'image'" />
    <CDocumentSunrise v-if="mode === 'sunrise'" />
    <CDocumentText v-if="mode === 'text'" />
    <CDocumentTictactoe v-if="mode === 'tictactoe'" />
    <CDocumentXkcd v-if="mode === 'xkcd'" />
    <hr v-if="mode !== undefined" />
    <section>
      <p>What do you want to print?</p>
      <div class="list">
        <button @click="mode = 'calendar'">Calendar</button>
        <button @click="mode = 'cells'">Cellular Automaton</button>
        <button @click="mode = 'chat'">Chat Message</button>
        <button @click="mode = 'egg'">Easter Egg</button>
        <button @click="mode = 'image'">Image</button>
        <button @click="mode = 'sunrise'">Sunrise and Sunset</button>
        <button @click="mode = 'text'">Text</button>
        <button @click="mode = 'tictactoe'">Tic Tac Toe</button>
        <button @click="mode = 'xkcd'">xkcd</button>
        <a href="photo.html">Take a photo</a>
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

  border: 2px solid black;
  border-radius: 4px;
  padding: 0 4px;

  color: white;
  background-color: black;
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
