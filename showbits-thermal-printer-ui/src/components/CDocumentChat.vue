<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef } from "vue";
import CError from "./CError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const form = useTemplateRef<HTMLFormElement>("form");

const username = ref("");
const content = ref("");
const feed = ref(false);

// Ctrl+Enter in textarea should submit form.
function textareaKeypress(ev: KeyboardEvent): void {
  if (ev.ctrlKey && ev.key === "Enter") {
    form.value?.requestSubmit();
  }
}

function submit() {
  const data = new URLSearchParams();
  data.append("username", username.value);
  data.append("content", content.value);
  data.append("feed", String(feed.value));
  void makeRequest("api/chat", data);
}
</script>

<template>
  <form ref="form" @submit.prevent="submit">
    <h2>Chat Message</h2>

    <label class="wide">
      Name:
      <input
        v-model="username"
        type="text"
        minlength="1"
        maxlength="32"
        required
        :disabled
      />
    </label>

    <label for="content">Content:</label>
    <textarea
      id="content"
      v-model="content"
      rows="10"
      :disabled
      @keypress="textareaKeypress"
    ></textarea>

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

textarea {
  margin-top: -16px;

  align-self: center;
  width: 384px;

  /* Prevent manual resizing from changing the width. */
  min-width: 384px;
  max-width: 384px;

  /* Emulate how typst wraps text. */
  overflow-wrap: normal;
}

.wide {
  display: flex;
  flex-direction: column;
}
</style>
