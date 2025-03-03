<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef } from "vue";
import CError from "./CError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const image = useTemplateRef<HTMLInputElement>("image");

const title = ref("");
const caption = ref("");
const algo = ref("stucki");
const bright = ref(true);
const seamless = ref(false);
const feed = ref(true);

function submit() {
  const data = new FormData();

  const file = image.value?.files?.[0];
  if (file !== undefined) data.append("image", file);

  if (title.value) data.append("title", title.value);
  if (caption.value) data.append("caption", caption.value);
  data.append("algo", algo.value);
  data.append("bright", String(bright.value));
  data.append("seamless", String(seamless.value));
  data.append("feed", String(feed.value));
  void makeRequest("api/image", data);
}
</script>

<template>
  <form @submit.prevent="submit">
    <h2>Image</h2>

    <input ref="image" type="file" accept="image/*" required :disabled />

    <label class="wide">
      Title:
      <input v-model="title" type="text" placeholder="none" :disabled />
    </label>

    <label class="wide">
      Caption:
      <input v-model="caption" type="text" placeholder="none" :disabled />
    </label>

    <label class="wide">
      Algorithm:
      <select v-model="algo" :disabled>
        <option value="stucki">Stucki</option>
        <option value="floyd-steinberg">Floyd-Steinberg</option>
      </select>
    </label>

    <div class="wide">
      <label><input v-model="bright" type="checkbox" :disabled /> Bright</label>
      <label>
        <input v-model="seamless" type="checkbox" :disabled />
        Seamless
      </label>
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
