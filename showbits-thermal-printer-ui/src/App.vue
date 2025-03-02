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
