<script setup lang="ts">
import { ref } from "vue";

const text = ref("");
const forceWrap = ref(false);
const feed = ref(true);

async function submit(): Promise<void> {
  try {
    const data = new URLSearchParams();
    data.append("text", text.value);
    data.append("force_wrap", String(forceWrap.value));
    data.append("feed", String(feed.value));
    const response = await fetch("/api/text", { method: "POST", body: data });
    console.log("POST succeeded:", await response.text());
  } catch (err) {
    console.log("POST failed:", err);
  }
}
</script>

<template>
  <section>
    <h2>Text</h2>
    <textarea v-model="text" rows="10"></textarea>
    <fieldset>
      <label><input v-model="forceWrap" type="checkbox" /> Force-Wrap</label>
      <label><input v-model="feed" type="checkbox" /> Feed</label>
    </fieldset>
    <button @click="submit">Print</button>
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
