<script setup lang="ts">
import { RiLoader4Fill } from "@remixicon/vue";
import { computed, onMounted, ref, useTemplateRef } from "vue";
import CPhotoButtonFlip from "./components/CPhotoButtonFlip.vue";
import CPhotoButtonGallery from "./components/CPhotoButtonGallery.vue";
import CPhotoButtonRecord from "./components/CPhotoButtonRecord.vue";
import { assert } from "./lib/assert";

const endpoint = "api/image";
const video = useTemplateRef<HTMLVideoElement>("video");

const stream = ref<MediaStream>();
const facing = ref<string>();
const mirrored = computed(() => facing.value === "user");
const covered = ref(false);
const originals = ref<boolean>();

const originalsInfo = computed(() => {
  if (originals.value === true) {
    return "Uploaded images are saved.";
  } else if (originals.value === false) {
    return "Uploaded images are not saved.";
  } else {
    return "Uploaded images may be saved.";
  }
});

function getFacingModeFromStream(stream: MediaStream): string | undefined {
  const videos = stream.getVideoTracks();
  if (videos.length === 0) return undefined;
  const video = videos[0];
  return video?.getSettings().facingMode;
}

function deinitStream() {
  if (stream.value === undefined) return;
  for (const track of stream.value.getTracks()) {
    track.stop();
  }
  stream.value = undefined;
}

async function initStream(facingMode?: string) {
  assert(video.value !== null);
  const video_ = video.value;

  // If the tracks are not all stopped, getUserMedia throws an exception.
  deinitStream();

  stream.value = await navigator.mediaDevices.getUserMedia({
    video: { facingMode: { ideal: facingMode } },
  });

  facing.value = getFacingModeFromStream(stream.value);

  video_.srcObject = stream.value;
}

async function initOriginals() {
  const response = await fetch(endpoint);
  const info = (await response.json()) as { originals?: boolean };
  originals.value = info.originals ?? false;
}

async function waitAtLeast(duration: number, since: number) {
  const now = Date.now();
  const wait = duration - (now - since);
  if (wait > 0) {
    await new Promise((resolve) => setTimeout(resolve, wait));
  }
}

async function postImage(image: Blob | File) {
  const form = new FormData();
  form.append("image", image);
  form.append("caption", new Date().toLocaleString());

  const start = Date.now();
  covered.value = true;
  try {
    await fetch(endpoint, { method: "POST", body: form });
  } catch (e) {
    console.error("Error uploading image:", e);
  }
  await waitAtLeast(500, start);
  covered.value = false;
}

async function onGallery(file: File) {
  try {
    await postImage(file);
  } finally {
    await initStream(facing.value);
  }
}

async function onRecord() {
  assert(video.value !== null);
  const video_ = video.value;

  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  assert(ctx !== null);

  const scale = 384 / video_.videoWidth;
  canvas.width = video_.videoWidth * scale; // Yes, slightly redundant
  canvas.height = video_.videoHeight * scale;
  ctx.drawImage(video_, 0, 0, canvas.width, canvas.height);

  const blob = await new Promise<Blob | null>((resolve) => {
    canvas.toBlob(resolve);
  });
  assert(blob !== null);

  await postImage(blob);
}

async function onFlip() {
  const facingOpposite = facing.value === "user" ? "environment" : "user";
  await initStream(facingOpposite);
}

onMounted(() => {
  void initStream();
  void initOriginals();
});
</script>

<template>
  <video ref="video" :class="{ mirrored }" autoplay playsinline></video>
  <div class="originals">
    <p>{{ originalsInfo }}</p>
  </div>
  <div class="buttons">
    <CPhotoButtonGallery @click="onGallery" />
    <CPhotoButtonRecord :disabled="stream === undefined" @click="onRecord" />
    <CPhotoButtonFlip
      :disabled="stream === undefined || facing === undefined"
      @click="onFlip"
    />
  </div>
  <div class="cover" :class="{ hidden: !covered }">
    <RiLoader4Fill size="48px" />
  </div>
</template>

<style>
body {
  margin: 0;
  background-color: black;
}
</style>

<style scoped>
video {
  position: absolute;
  width: 100%;
  height: 100%;
}

video.mirrored {
  scale: -1 1;
}

.originals {
  position: absolute;
  top: 0;
  width: 100%;
  display: flex;
  justify-content: center;
}

.originals p {
  margin: 0;
  margin-top: 20px;
  padding: 0.2em 0.8em;
  border-radius: 10em;
  background-color: #fffa;
  text-align: center;
}

.buttons {
  position: absolute;
  bottom: 0;
  width: 100%;
  margin-bottom: 20px;

  display: flex;
  justify-content: space-evenly;
  align-items: center;
}

.cover {
  position: absolute;
  width: 100%;
  height: 100%;
  background-color: white;
  transition: opacity 100ms linear;

  display: flex;
  justify-content: center;
  align-items: center;
}

.cover.hidden {
  opacity: 0;
  pointer-events: none;
}

.cover svg {
  animation: spinner 2s linear infinite;
}

@keyframes spinner {
  to {
    transform: rotate(360deg);
  }
}
</style>
