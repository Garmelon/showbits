<script setup lang="ts">
import { computed, onMounted, ref, useTemplateRef } from "vue";
import CPhotoButtonFlip from "./components/CPhotoButtonFlip.vue";
import CPhotoButtonGallery from "./components/CPhotoButtonGallery.vue";
import CPhotoButtonRecord from "./components/CPhotoButtonRecord.vue";
import { assert } from "./lib/assert";

const video = useTemplateRef<HTMLVideoElement>("video");

const stream = ref<MediaStream>();
const facing = ref<string>();
const mirrored = computed(() => facing.value === "user");
const covered = ref(false);

function getFacingModeFromStream(stream: MediaStream): string | undefined {
  const videos = stream.getVideoTracks();
  if (videos.length === 0) return undefined;
  const video = videos[0];
  return video?.getSettings().facingMode;
}

async function initStream(facingMode?: string) {
  assert(video.value !== null);
  const video_ = video.value;

  // If the tracks are not all stopped, getUserMedia throws an exception.
  if (stream.value !== undefined) {
    for (const track of stream.value.getTracks()) {
      track.stop();
    }
  }

  stream.value = undefined;
  facing.value = undefined;
  video_.srcObject = null;

  stream.value = await navigator.mediaDevices.getUserMedia({
    video: { facingMode: { ideal: facingMode } },
  });

  facing.value = getFacingModeFromStream(stream.value);

  video_.srcObject = stream.value;
}

async function waitAtLeast(duration: number, since: number) {
  const now = Date.now();
  const wait = duration - (now - since);
  if (wait > 0) {
    await new Promise((resolve) => setTimeout(resolve, wait));
  }
}

async function onRecord() {
  assert(video.value !== null);

  const canvas = document.createElement("canvas");

  const scale = 384 / video.value.videoWidth;
  canvas.width = video.value.videoWidth * scale;
  canvas.height = video.value.videoHeight * scale;

  const ctx = canvas.getContext("2d");
  assert(ctx !== null);

  ctx.drawImage(video.value, 0, 0, canvas.width, canvas.height);

  const blob = await new Promise<Blob | null>((resolve) => {
    canvas.toBlob(resolve);
  });
  assert(blob !== null);

  const form = new FormData();
  form.append("image", blob);
  form.append("caption", new Date().toLocaleString());

  const start = Date.now();
  covered.value = true;
  try {
    await fetch("api/image", { method: "POST", body: form });
  } catch (e) {
    console.error("Error uploading image:", e);
  }
  await waitAtLeast(500, start);
  covered.value = false;
}

async function onFlip() {
  const facingOpposite = facing.value === "user" ? "environment" : "user";
  await initStream(facingOpposite);
}

onMounted(async () => {
  await initStream();
});
</script>

<template>
  <video ref="video" :class="{ mirrored }" autoplay playsinline></video>
  <div class="buttons">
    <CPhotoButtonGallery style="visibility: hidden" />
    <CPhotoButtonRecord :disabled="stream === undefined" @click="onRecord" />
    <CPhotoButtonFlip
      :disabled="stream === undefined || facing === undefined"
      @click="onFlip"
    />
  </div>
  <div class="cover" :class="{ hidden: !covered }"></div>
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
  transition: background-color 100ms linear;
}

.cover.hidden {
  background-color: transparent;
  pointer-events: none;
}
</style>
