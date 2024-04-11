import { readable, writable } from "svelte/store";
import type { Subscriber } from "svelte/store";

interface Page {
  url: URL;
  params: Record<string, string>;
}

export const mockStores = {
  navigating: readable(null),
  page: writable({ url: new URL("http://localhost"), params: {} }),
  session: writable(null),
  updated: readable(false),
};

export const page = {
  subscribe(fn: Subscriber<Page>) {
    return mockStores.page.subscribe(fn);
  },
};
