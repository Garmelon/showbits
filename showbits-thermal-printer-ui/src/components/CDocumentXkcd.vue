<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef } from "vue";
import CError from "./CError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const form = useTemplateRef<HTMLFormElement>("form");

const number = ref<number>();
const dither = ref(true);
const bright = ref(true);
const feed = ref(true);

function submit() {
  const data = new URLSearchParams();
  if (typeof number.value === "number")
    data.append("number", number.value.toFixed());
  data.append("dither", String(dither.value));
  data.append("bright", String(bright.value));
  data.append("feed", String(feed.value));
  void makeRequest("api/xkcd", data);
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>xkcd</h2>

    <label class="wide">
      Number:
      <input
        v-model="number"
        type="number"
        min="1"
        placeholder="current"
        :disabled
      />
    </label>

    <div class="wide">
      <label><input v-model="dither" type="checkbox" :disabled /> Dither</label>
      <label
        ><input
          v-model="bright"
          type="checkbox"
          :disabled="disabled || !dither"
        />
        Bright</label
      >
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
