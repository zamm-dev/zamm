import { get } from "svelte/store";
import { playSound, type Sound } from "./bindings";
import { soundOn, volume } from "./preferences";

export function playSoundEffect(sound: Sound, speed?: number) {
  if (get(soundOn)) {
    const soundEffectSpeed = speed || 1;
    // boost volume to compensate for lowered volume from lower speed
    const soundEffectVolume = get(volume) / soundEffectSpeed;
    try {
      playSound(sound, soundEffectVolume, soundEffectSpeed);
    } catch (e) {
      console.error(`Problem playing ${sound}: ${e}`);
    }
    if (window._testRecordSoundPlayed !== undefined) {
      window._testRecordSoundPlayed();
    }
  }
}
