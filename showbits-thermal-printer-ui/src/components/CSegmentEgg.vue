<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef } from "vue";
import CSegmentError from "./CSegmentError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const form = useTemplateRef<HTMLFormElement>("form");

const seed = ref<number | string>("");
const mode = ref("random");
const feed = ref(true);

function submit() {
  const data = new URLSearchParams();
  if (typeof seed.value === "number") data.append("seed", seed.value.toFixed());
  data.append("mode", mode.value);
  data.append("feed", String(feed.value));
  void makeRequest("/api/egg", data);
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>Easter Egg</h2>
    <svg viewBox="-2 0 4 6">
      <path
        fill="#000"
        d="M0,0 C1,0,2,2,2,3.5 S1,6,0,6 S-2,5,-2,3.5 S-1,0,0,0"
      />
    </svg>
    <label class="wide">
      Seed:
      <input
        v-model="seed"
        type="number"
        min="-9223372036854775808"
        max="9223372036854775807"
        step="1"
        placeholder="random"
      />
    </label>
    <label class="wide">
      Mode:
      <select v-model="mode">
        <option value="random">Random</option>
        <option value="good">Good</option>
        <option value="bad">Bad</option>
      </select>
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

svg {
  width: 50%;
  margin: auto;
}

.wide {
  display: flex;
  flex-direction: column;
}
</style>
