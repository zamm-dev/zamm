import { writable, derived } from "svelte/store";

export const animationsOn = writable(true);
export const transparencyOn = writable(false);
export const backgroundAnimation = writable(false);
export const animationSpeed = writable(1);
export const soundOn = writable(true);
export const volume = writable(1);

export const standardDuration = derived(
  [animationsOn, animationSpeed],
  ([$animationsOn, $animationSpeed]) =>
    $animationsOn ? 100 / $animationSpeed : 0,
);
