<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { computed, ref, type StyleValue, useTemplateRef } from "vue";
import CSegmentError from "./CSegmentError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const form = useTemplateRef<HTMLFormElement>("form");

const text = ref("");
const forceWrap = ref(false);
const feed = ref(true);

// Emulate how typst is wrapping text.
const textareaStyle = computed<StyleValue>(() =>
  forceWrap.value ? { wordBreak: "break-all" } : { overflowWrap: "normal" },
);

// Ctrl+Enter in textarea should submit form.
function textareaKeypress(ev: KeyboardEvent): void {
  if (ev.ctrlKey && ev.key === "Enter") {
    form.value?.requestSubmit();
  }
}

function submit() {
  const data = new URLSearchParams();
  data.append("text", text.value);
  data.append("force_wrap", String(forceWrap.value));
  data.append("feed", String(feed.value));
  void makeRequest("/api/text", data);
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>Text</h2>
    <!-- For some reason one col = 2 characters. -->
    <textarea
      v-model="text"
      rows="10"
      cols="24"
      :style="textareaStyle"
      :disabled
      @keypress="textareaKeypress"
    ></textarea>
    <fieldset class="wide">
      <label>
        <input v-model="forceWrap" type="checkbox" :disabled />
        Force-Wrap
      </label>
      <label><input v-model="feed" type="checkbox" :disabled /> Feed</label>
    </fieldset>
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

textarea {
  align-self: center;
  width: fit-content;

  /* Prevent manual resizing from changing the width. */
  min-width: fit-content;
  max-width: fit-content;
}

.wide {
  display: flex;
  flex-direction: column;
}

fieldset {
  border: none;
}
</style>
