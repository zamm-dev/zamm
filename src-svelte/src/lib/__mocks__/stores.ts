import type { StoryFn, Decorator, StoryContext } from "@storybook/svelte";
import {
  animationsOn,
  backgroundAnimation,
  animationSpeed,
} from "$lib/preferences";
import { systemInfo } from "$lib/system-info";
import type { SystemInfo } from "$lib/bindings";
import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";

interface Preferences {
  animationsOn?: boolean;
  backgroundAnimation?: boolean;
  animationSpeed?: number;
}

interface Stores {
  systemInfo?: SystemInfo;
}

interface StoreArgs {
  preferences?: Preferences;
  stores?: Stores;
  [key: string]: any;
}

const SvelteStoresDecorator: Decorator = (
  story: StoryFn,
  context: StoryContext,
) => {
  const { args, parameters } = context;
  const { preferences, stores } = parameters as StoreArgs;

  // set to their defaults on first load
  firstAppLoad.set(true);
  firstPageLoad.set(true);

  if (preferences?.animationsOn === undefined) {
    animationsOn.set(true);
  } else {
    animationsOn.set(preferences.animationsOn);
  }

  if (preferences?.backgroundAnimation !== undefined) {
    backgroundAnimation.set(preferences.backgroundAnimation);
  }

  if (preferences?.animationSpeed === undefined) {
    animationSpeed.set(1);
  } else {
    animationSpeed.set(preferences.animationSpeed);
  }

  systemInfo.set(stores?.systemInfo);

  return story(args, context);
};

export default SvelteStoresDecorator;
