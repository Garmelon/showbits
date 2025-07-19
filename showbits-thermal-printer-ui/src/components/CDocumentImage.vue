<script setup lang="ts">
import { useApiRequest } from "@/apiRequest";
import { ref, useTemplateRef, watchEffect } from "vue";
import CError from "./CError.vue";

const { disabled, error, makeRequest } = useApiRequest();
const image = useTemplateRef<HTMLInputElement>("image");

const file = ref<File>();
const title = ref("");
const caption = ref("");
const algo = ref("stucki");
const rotate = ref(false);
const bright = ref(true);
const seamless = ref(false);
const feed = ref(true);

const fileAsUrl = ref<string>();
watchEffect(() => {
  if (file.value === undefined) return;
  fileAsUrl.value = undefined;
  const reader = new FileReader();
  reader.addEventListener("loadend", () => {
    if (typeof reader.result !== "string") return;
    fileAsUrl.value = reader.result;
  });
  reader.readAsDataURL(file.value);
});

function onFormSubmit() {
  if (file.value === undefined) return;
  const data = new FormData();
  data.append("image", file.value);
  if (title.value) data.append("title", title.value);
  if (caption.value) data.append("caption", caption.value);
  data.append("algo", algo.value);
  data.append("rotate", String(rotate.value));
  data.append("bright", String(bright.value));
  data.append("seamless", String(seamless.value));
  data.append("feed", String(feed.value));
  void makeRequest("api/image", data);
}

function onFormPaste(e: ClipboardEvent) {
  const items = e.clipboardData?.items;
  if (items === undefined) return;
  for (const item of items) {
    const theFile = item.getAsFile();
    if (theFile === null) continue;
    if (!theFile.type.startsWith("image/")) continue;
    file.value = theFile;
    break;
  }
}

function onImageChange() {
  const theFile = image.value?.files?.[0];
  if (theFile === undefined) return;
  if (!theFile.type.startsWith("image/")) return;
  file.value = theFile;
}
</script>

<template>
  <form @submit.prevent="onFormSubmit" @paste="onFormPaste">
    <h2>Image</h2>

    <img v-if="fileAsUrl !== undefined" :src="fileAsUrl" />

    <label class="image">
      Select or paste an image.
      <input ref="image" type="file" accept="image/*" @change="onImageChange" />
    </label>

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
      <label><input v-model="rotate" type="checkbox" :disabled /> Rotate</label>
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

.image {
  cursor: pointer;
  text-decoration: underline;
}

.image:hover {
  text-shadow: 0px 0px 10px #aaa;
}

.image input {
  display: none;
}

.wide {
  display: flex;
  flex-direction: column;
}
</style>
