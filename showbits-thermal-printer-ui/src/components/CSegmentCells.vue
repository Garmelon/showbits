<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref } from "vue";
import CSegmentError from "./CSegmentError.vue";

const { disabled, error, makeRequest } = useApiRequest();

const showRule = ref(true);
const rule = ref<number | string>("");
const rows = ref(128);
const scale = ref(4);
const feed = ref(true);

function submit() {
  const data = new URLSearchParams();
  data.append("show_rule", String(showRule.value));
  if (typeof rule.value === "number") data.append("rule", rule.value.toFixed());
  data.append("rows", rows.value.toFixed());
  data.append("scale", scale.value.toFixed());
  data.append("feed", String(feed.value));
  void makeRequest("/api/cells", data);
}
</script>

<template>
  <form @submit.prevent="submit">
    <h2>Cellular Automaton</h2>

    <label class="wide">
      Rule:
      <input
        v-model="rule"
        type="number"
        min="0"
        max="127"
        placeholder="random"
        :disabled
      />
    </label>

    <label class="wide">
      Rows:
      <input v-model="rows" type="number" min="1" max="1024" :disabled />
    </label>

    <label class="wide">
      Scale:
      <input v-model="scale" type="number" min="1" max="16" :disabled />
    </label>

    <div class="wide">
      <label>
        <input v-model="showRule" type="checkbox" :disabled /> Show rule
      </label>
      <label><input v-model="feed" type="checkbox" :disabled /> Feed</label>
    </div>

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
