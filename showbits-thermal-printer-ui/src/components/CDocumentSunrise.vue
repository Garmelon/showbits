<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref } from "vue";
import CError from "./CError.vue";

const { disabled, error, makeRequest } = useApiRequest();

const today = new Date();
const latitude = ref<number>();
const longitude = ref<number>();
const year = ref(today.getFullYear());
const month = ref(today.getMonth() + 1);
const feed = ref(true);

function submit() {
  if (typeof latitude.value !== "number") return;
  if (typeof longitude.value !== "number") return;
  const data = new URLSearchParams();
  data.append("latitude", latitude.value.toFixed(5));
  data.append("longitude", longitude.value.toFixed(5));
  data.append("year", year.value.toFixed());
  data.append("month", month.value.toFixed());
  data.append("feed", String(feed.value));
  void makeRequest("api/sunrise", data);
}
</script>

<template>
  <form @submit.prevent="submit">
    <h2>Sunrise and Sunset</h2>

    <label class="wide">
      Latitude:
      <input
        v-model="latitude"
        type="number"
        step="any"
        min="-90"
        max="90"
        required
        :disabled
      />
    </label>

    <label class="wide">
      Longitude:
      <input
        v-model="longitude"
        type="number"
        step="any"
        min="-180"
        max="180"
        required
        :disabled
      />
    </label>

    <label class="wide">
      Year:
      <input
        v-model="year"
        type="number"
        min="-9999"
        max="9999"
        required
        :disabled
      />
    </label>

    <label class="wide">
      Month:
      <input
        v-model="month"
        type="number"
        min="1"
        max="12"
        required
        :disabled
      />
    </label>

    <label><input v-model="feed" type="checkbox" :disabled /> Feed</label>

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
