<script setup lang="ts"></script>

<template>
  <button><div></div></button>
</template>

<style scoped>
button {
  width: 100px;
  height: 100px;

  --color: red;

  border: 10px solid var(--color);
  border-radius: 100px;
  background-color: transparent;
}

button:enabled:active {
  --color: white;
}

button:disabled {
  --color: #444;
}

button div {
  width: 60px;
  height: 60px;
  border-radius: 60px;
  margin: auto;

  background-color: var(--color);
}
</style>
