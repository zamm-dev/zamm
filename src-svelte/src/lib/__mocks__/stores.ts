import type { StoryFn, Decorator, StoryContext } from "@storybook/svelte";
import {
  animationsOn,
  backgroundAnimation,
  animationSpeed,
} from "$lib/preferences";
import { systemInfo } from "$lib/system-info";
import { conversation } from "../../routes/chat/Chat.svelte";
import {
  canonicalRef,
  getDefaultApiCall,
  prompt,
} from "../../routes/api-calls/new/ApiCallEditor.svelte";
import type { SystemInfo, ChatMessage, LlmCallReference } from "$lib/bindings";
import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
import type { ChatPromptVariant } from "$lib/additionalTypes";

interface Preferences {
  animationsOn?: boolean;
  backgroundAnimation?: boolean;
  animationSpeed?: number;
}

interface ApiCallEditing {
  canonicalRef: LlmCallReference;
  prompt: ChatPromptVariant;
}

interface Stores {
  systemInfo?: SystemInfo;
  conversation?: ChatMessage[];
  apiCallEditing?: ApiCallEditing;
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
  conversation.set(stores?.conversation || []);
  canonicalRef.set(stores?.apiCallEditing?.canonicalRef);
  prompt.set(stores?.apiCallEditing?.prompt || getDefaultApiCall());

  return story(args, context);
};

export default SvelteStoresDecorator;
