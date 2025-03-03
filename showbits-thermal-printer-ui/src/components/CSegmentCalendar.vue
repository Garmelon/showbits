<script setup lang="ts">
import { ref, useTemplateRef } from "vue";

const form = useTemplateRef<HTMLFormElement>("form");
const disabled = ref(false);
const error = ref<string>();

const today = new Date();
const year = ref(today.getFullYear());
const month = ref(today.getMonth() + 1);
const feed = ref(true);

async function waitAtLeast(duration: number, since: number): Promise<void> {
  const now = Date.now();
  const wait = duration - (now - since);
  if (wait > 0) {
    await new Promise((resolve) => setTimeout(resolve, wait));
  }
}

async function submit(): Promise<void> {
  const start = Date.now();
  disabled.value = true;

  try {
    const data = new URLSearchParams();
    data.append("year", String(year.value));
    data.append("month", String(month.value));
    data.append("feed", String(feed.value));
    const response = await fetch("/api/calendar", {
      method: "POST",
      body: data,
    });
    if (!response.ok) {
      const status = `${response.status.toFixed()} ${response.statusText}`;
      const text = await response.text();
      error.value = text.length > 0 ? `${status}: ${text}` : status;
    }
  } catch (err) {
    error.value = String(err);
  }

  await waitAtLeast(500, start);
  disabled.value = false;
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>Calendar</h2>
    <label>Year: <input v-model="year" type="number" /></label>
    <label>
      Month:
      <input v-model="month" type="number" min="1" max="12" />
    </label>
    <label><input v-model="feed" type="checkbox" :disabled /> Feed</label>
    <button :disabled>Print</button>
    <p v-if="error" class="error">{{ error }}</p>
  </form>
</template>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

input[type="number"] {
  box-sizing: border-box;
  width: 100%;
}

fieldset {
  display: flex;
  flex-direction: column;
  border: none;
}

.error {
  background-color: #fdd;
  color: #400;
  padding: 0 2px;
}
</style>
