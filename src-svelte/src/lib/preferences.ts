import { writable, derived, get } from "svelte/store";

export const STANDARD_ROOT_EM = 18;
export const SMALLER_ROOT_EM = 15;

export const animationsOn = writable(true);
export const transparencyOn = writable(false);
export const highDpiAdjust = writable(false);
export const backgroundAnimation = writable(false);
export const animationSpeed = writable(1);
export const soundOn = writable(true);
export const volume = writable(1);

export const standardDuration = derived(
  [animationsOn, animationSpeed],
  ([$animationsOn, $animationSpeed]) =>
    $animationsOn ? 100 / $animationSpeed : 0,
);

export const rootEm = derived(highDpiAdjust, ($highDpiAdjustOn) => {
  return $highDpiAdjustOn ? SMALLER_ROOT_EM : STANDARD_ROOT_EM;
});

export function getAdjustedFontSize(fontSize: number) {
  return Math.round(fontSize * (get(rootEm) / STANDARD_ROOT_EM));
}

export function newEmStore(initialValue: number) {
  return derived(rootEm, (_) => getAdjustedFontSize(initialValue));
}
