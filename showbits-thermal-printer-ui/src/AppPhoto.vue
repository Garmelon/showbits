<script setup lang="ts">
import { onMounted } from "vue";

onMounted(async () => {
  const video = document.getElementById("video") as HTMLVideoElement;
  const button = document.getElementById("button") as HTMLButtonElement;
  const flip = document.getElementById("flip") as HTMLAnchorElement;
  const cover = document.getElementById("cover") as HTMLDivElement;

  const facing =
    new URLSearchParams(window.location.search).get("facing") ?? undefined;

  function getStreamFacingMode(stream: MediaStream): string | undefined {
    const videos = stream.getVideoTracks();
    if (videos.length === 0) return undefined;
    const video = videos[0];
    return video?.getSettings().facingMode;
  }

  async function initStream(facingMode?: string) {
    // Display video
    let stream = await navigator.mediaDevices.getUserMedia({
      video: { facingMode: { ideal: facingMode } },
    });
    video.srcObject = stream;

    // Flip video horizontally if it's facing the user
    const facing = getStreamFacingMode(stream);
    if (facing !== "environment") {
      video.classList.add("mirrored");
    }

    // Enable or disable flip button
    const canFlip = facing !== undefined;
    const facingOpposite = facing === "user" ? "environment" : "user";
    flip.hidden = !canFlip;
    flip.setAttribute("href", `?facing=${facingOpposite}`);
  }

  await initStream(facing);

  async function waitAtLeast(duration: number, since: number) {
    const now = Date.now();
    const wait = duration - (now - since);
    if (wait > 0) {
      await new Promise((resolve) => setTimeout(resolve, wait));
    }
  }

  button.addEventListener("click", () => {
    const canvas = document.createElement("canvas");
    const scale = 384 / video.videoWidth;
    canvas.width = video.videoWidth * scale;
    canvas.height = video.videoHeight * scale;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const ctx = canvas.getContext("2d")!;
    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

    // eslint-disable-next-line @typescript-eslint/no-misused-promises
    canvas.toBlob(async (blob) => {
      if (blob === null) return;

      const form = new FormData();
      form.append("image", blob);
      form.append("caption", new Date().toLocaleString());

      const start = Date.now();
      cover.classList.remove("hidden");
      try {
        await fetch("api/image", { method: "POST", body: form });
      } catch (e) {
        console.error("Error uploading image:", e);
      }
      await waitAtLeast(500, start);
      cover.classList.add("hidden");
    });
  });
});
</script>

<template>
  <video id="video" autoplay playsinline></video>
  <button id="button"><div class="circle"></div></button>
  <a id="flip" hidden>
    <svg viewBox="0 0 6 6">
      <path fill="#fff" stroke="none" d="M0,2h1v4h1v-4h1l-1.5,-2"></path>
      <path fill="#fff" stroke="none" d="M3,4h1v-4h1v4h1l-1.5,2"></path>
    </svg>
  </a>
  <div id="cover" class="hidden"></div>
</template>

<style>
body {
  margin: 0;
  background-color: black;
}

video {
  position: absolute;
  width: 100%;
  height: 100%;
}

video.mirrored {
  scale: -1 1;
}

#button {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);

  width: 100px;
  height: 100px;

  border: 10px solid #f00;
  border-radius: 100px;
  background-color: transparent;
}

#button:active {
  border-color: #fff;
}

#button .circle {
  width: 60px;
  height: 60px;
  border-radius: 60px;
  margin: auto;

  background-color: #f00;
}

#button:active .circle {
  background-color: #a00;
}

#flip {
  position: absolute;
  bottom: 20px;
  right: 20px;

  box-sizing: border-box;
  width: 60px;
  height: 60px;

  background-color: transparent;
  border: 5px solid #fff;
  border-radius: 100px;

  touch-action: manipulation;
}

#flip:active {
  background-color: #fff;
}

#flip svg {
  width: 60%;
  height: 60%;

  position: relative;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
}

#flip:active path {
  fill: #000;
}

#cover {
  position: absolute;
  width: 100%;
  height: 100%;
  background-color: white;
  transition: background-color 100ms linear;
}

#cover.hidden {
  background-color: transparent;
  pointer-events: none;
}
</style>
