<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref } from "vue";
import CSegmentError from "./CSegmentError.vue";

const { disabled, error, makeRequest } = useApiRequest();

const feed = ref(true);

function submit() {
  const data = new URLSearchParams();
  data.append("feed", String(feed.value));
  void makeRequest("/api/tictactoe", data);
}
</script>

<template>
  <form @submit.prevent="submit">
    <h2>Tic Tac Toe</h2>

    <svg viewBox="0 0 384 384">
      <path
        stroke="#000"
        stroke-width="4"
        d="M128,0V384 M256,0V384 M0,128H384 M0,256H384"
      />
    </svg>

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
</style>
