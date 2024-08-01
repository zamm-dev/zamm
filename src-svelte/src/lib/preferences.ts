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

const STANDARD_ROOT_EM = 18;

function getRootFontSize() {
  const rem = parseFloat(getComputedStyle(document.documentElement).fontSize);

  if (isNaN(rem)) {
    console.warn("Could not get root font size, assuming default of 18px");
    return 18;
  }
  return rem;
}

export const ROOT_EM = getRootFontSize();

export function getAdjustedFontSize(fontSize: number) {
  return Math.round(fontSize * (ROOT_EM / STANDARD_ROOT_EM));
}
