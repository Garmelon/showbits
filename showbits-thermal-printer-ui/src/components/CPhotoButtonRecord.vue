<script setup lang="ts"></script>

<template>
  <button><div></div></button>
</template>

<style scoped>
button {
  width: 100px;
  height: 100px;

  --color: var(--record-button-fg);

  border: 10px solid var(--color);
  border-radius: 100px;
  background-color: transparent;
}

button:enabled:active {
  --color: var(--record-button-active-fg);
}

button:disabled {
  --color: var(--record-button-disabled-fg);
}

button div {
  width: 60px;
  height: 60px;
  border-radius: 60px;
  margin: auto;

  background-color: var(--color);
}
</style>
