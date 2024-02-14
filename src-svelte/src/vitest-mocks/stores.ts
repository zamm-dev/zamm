import { readable, writable } from "svelte/store";
import type { Subscriber } from "svelte/store";

interface Page {
  url: URL;
  params: Record<string, string>;
}

const getStores = () => ({
  navigating: readable(null),
  page: readable({ url: new URL("http://localhost"), params: {} }),
  session: writable(null),
  updated: readable(false),
});

export const page = {
  subscribe(fn: Subscriber<Page>) {
    return getStores().page.subscribe(fn);
  },
};
