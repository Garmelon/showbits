import { ref } from "vue";

async function waitAtLeast(duration: number, since: number): Promise<void> {
  const now = Date.now();
  const wait = duration - (now - since);
  if (wait > 0) {
    await new Promise((resolve) => setTimeout(resolve, wait));
  }
}

export function useApiRequest() {
  const disabled = ref(false);
  const error = ref<string>();

  async function makeRequest(url: string, data: URLSearchParams | FormData) {
    const start = Date.now();
    error.value = undefined;
    disabled.value = true;

    try {
      const response = await fetch(url, { method: "POST", body: data });
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

  return { disabled, error, makeRequest };
}
