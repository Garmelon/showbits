<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef } from "vue";
import CError from "./CError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const form = useTemplateRef<HTMLFormElement>("form");

const text = ref("");
const feed = ref(true);

function submit() {
  const data = new URLSearchParams();
  data.append("text", text.value);
  data.append("feed", String(feed.value));
  void makeRequest("api/banner", data);
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>Banner</h2>

    <input v-model="text" :disabled />

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
