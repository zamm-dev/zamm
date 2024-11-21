import { mockStores } from "./stores";

// todo: remove because no longer useful in Svelte 5 with vitest
export function goto(url: string) {
  mockStores.page.set({
    url: new URL(url, window.location.href),
    params: {},
  });
}
