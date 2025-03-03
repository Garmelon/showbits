<script setup lang="ts">
import { computed, ref, type StyleValue, useTemplateRef } from "vue";

const form = useTemplateRef<HTMLFormElement>("form");
const disabled = ref(false);
const error = ref<string>();

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

async function waitAtLeast(duration: number, since: number): Promise<void> {
  const now = Date.now();
  const wait = duration - (now - since);
  if (wait > 0) {
    await new Promise((resolve) => setTimeout(resolve, wait));
  }
}

async function submit(): Promise<void> {
  const start = Date.now();
  disabled.value = true;

  try {
    const data = new URLSearchParams();
    data.append("text", text.value);
    data.append("force_wrap", String(forceWrap.value));
    data.append("feed", String(feed.value));
    const response = await fetch("/api/text", { method: "POST", body: data });
    if (!response.ok) {
      const status = `${response.status.toFixed()} ${response.statusText}`;
      const text = await response.text();
      error.value = text.length > 0 ? `${status}: ${text}` : status;
    }
  } catch (err) {
    error.value = String(err);
  }

  await waitAtLeast(500, start);
  disabled.value = false;
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
    <fieldset>
      <label>
        <input v-model="forceWrap" type="checkbox" :disabled />
        Force-Wrap
      </label>
      <label><input v-model="feed" type="checkbox" :disabled /> Feed</label>
    </fieldset>
    <button :disabled>Print</button>
    <p v-if="error" class="error">{{ error }}</p>
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

fieldset {
  display: flex;
  flex-direction: column;
  border: none;
}

.error {
  background-color: #fdd;
  color: #400;
  padding: 0 2px;
}
</style>
