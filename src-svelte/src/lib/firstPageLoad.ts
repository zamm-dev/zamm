import { writable } from "svelte/store";

export const firstAppLoad = writable(true);
export const firstPageLoad = writable(true);
