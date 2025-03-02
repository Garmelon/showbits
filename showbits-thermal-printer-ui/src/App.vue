<script setup lang="ts">
import CSegment from "./components/CSegment.vue";
</script>

<template>
  <div class="strip">
    <CSegment />
  </div>
</template>

<style scoped>
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
  margin: 0;
  min-height: 100dvh;

  display: flex;
  flex-direction: column-reverse;
  align-items: center;

  background-color: black;
}
</style>
