import { mockStores } from "./stores";

export function goto(url: string) {
  mockStores.page.set({
    url: new URL(url, window.location.href),
    params: {},
  });
}
