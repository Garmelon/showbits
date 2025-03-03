<script setup lang="ts">
import CSegment from "./components/CSegment.vue";
</script>

<template>
  <div class="scroll">
    <div class="strip">
      <CSegment />
    </div>
  </div>
</template>

<style scoped>
.scroll {
  height: 100dvh;
  overflow: auto;

  display: flex;
  flex-direction: column-reverse;
  align-items: center;
}

.strip {
  box-sizing: content-box;
  width: 384px;
  padding: 32px;

  background-color: white;
  box-shadow: inset 0 0 32px #0003;

  position: relative;
}

/* This construction makes the outside shadow/glow look like the strip of paper
 * extends below the screen. Otherwise, the glow at the bottom would look
 * thinner, which looks weird. */
.strip::after {
  content: " ";
  z-index: -1;

  position: absolute;
  top: 0;
  left: 0;

  width: 100%;
  height: calc(100% + 500px);

  box-shadow:
    0 0 16px #0006 /* for contrast */,
    0 0 500px -10px #fffa;
}
</style>

<style>
@font-face {
  font-family: "Unifont";
  src: url("/fonts/unifont-15.1.05.otf") format("opentype");
}
@font-face {
  font-family: "Unifont-JP";
  src: url("/fonts/unifont_jp-15.1.05.otf") format("opentype");
}
@font-face {
  font-family: "Unifont Upper";
  src: url("/fonts/unifont_upper-15.1.05.otf") format("opentype");
}

* {
  font-family: "Unifont", "Unifont-JP", "Unifont Upper", monospace;
  font-size: 16px;
  margin: 0;
  padding: 0;
}

h1,
h2 {
  font-size: 32px;
}

body {
  background-color: black;
}
</style>
