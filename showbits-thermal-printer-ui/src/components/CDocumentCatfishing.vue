<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef } from "vue";
import CError from "./CError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const form = useTemplateRef<HTMLFormElement>("form");

const day = ref<number>(448);
const feed = ref(true);

function submit() {
  const data = new URLSearchParams();
  if (typeof day.value === "number") data.append("day", day.value.toFixed());
  data.append("feed", String(feed.value));
  void makeRequest("api/catfishing", data);
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>catfishing</h2>

    <label class="wide">
      Day:
      <input v-model="day" type="number" min="1" :disabled />
    </label>

    <div class="wide">
      <label><input v-model="feed" type="checkbox" :disabled /> Feed</label>
    </div>

    <button :disabled>Print</button>
    <CError :message="error" />
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
