<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef } from "vue";
import CSegmentError from "./CSegmentError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const form = useTemplateRef<HTMLFormElement>("form");

const today = new Date();
const year = ref(today.getFullYear());
const month = ref(today.getMonth() + 1);
const feed = ref(true);

function submit() {
  const data = new URLSearchParams();
  data.append("year", String(year.value));
  data.append("month", String(month.value));
  data.append("feed", String(feed.value));
  void makeRequest("/api/calendar", data);
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>Calendar</h2>
    <label class="wide">
      Year:
      <input v-model="year" type="number" />
    </label>
    <label class="wide">
      Month:
      <input v-model="month" type="number" min="1" max="12" />
    </label>
    <label><input v-model="feed" type="checkbox" :disabled /> Feed</label>
    <button :disabled>Print</button>
    <CSegmentError :message="error" />
  </form>
</template>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.wide {
  display: flex;
  flex-direction: column;
}
</style>
