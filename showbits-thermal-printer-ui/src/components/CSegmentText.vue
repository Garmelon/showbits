<script setup lang="ts"></script>

<template>
  <section>
    <h2>Text</h2>
    <textarea rows="10"></textarea>
    <fieldset>
      <label><input type="checkbox" /> Force-Wrap</label>
      <label><input type="checkbox" checked /> Feed</label>
    </fieldset>
    <button>Print</button>
  </section>
</template>

<style scoped>
section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

fieldset {
  display: flex;
  flex-direction: column;
  border: none;
}
</style>
