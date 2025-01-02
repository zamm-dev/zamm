import type { StoryFn, Decorator, StoryContext } from "@storybook/svelte";
import {
  animationsOn,
  transparencyOn,
  backgroundAnimation,
  animationSpeed,
} from "$lib/preferences";
import { systemInfo } from "$lib/system-info";
import {
  conversation,
  nextChatMessage,
  DEFAULT_SYSTEM_MESSAGE,
} from "../../routes/chat/Chat.svelte";
import {
  canonicalRef,
  getDefaultApiCall,
  prompt,
} from "../../routes/database/api-calls/new/ApiCallEditor.svelte";
import type { SystemInfo, ChatMessage, LlmCallReference } from "$lib/bindings";
import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
import type { ChatPromptVariant } from "$lib/additionalTypes";

interface Preferences {
  animationsOn?: boolean;
  transparencyOn?: boolean;
  backgroundAnimation?: boolean;
  animationSpeed?: number;
}

interface ApiCallEditing {
  canonicalRef: LlmCallReference;
  prompt: ChatPromptVariant;
}

interface Chat {
  conversation?: ChatMessage[];
  nextChatMessage?: string;
}

interface Stores {
  systemInfo?: SystemInfo;
  chat?: Chat;
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
    animationsOn.set(false);
  } else {
    animationsOn.set(preferences.animationsOn);
  }

  if (preferences?.transparencyOn === undefined) {
    transparencyOn.set(false);
  } else {
    transparencyOn.set(preferences.transparencyOn);
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
  nextChatMessage.set(stores?.chat?.nextChatMessage ?? "");
  conversation.set(stores?.chat?.conversation || [DEFAULT_SYSTEM_MESSAGE]);
  canonicalRef.set(stores?.apiCallEditing?.canonicalRef);
  prompt.set(stores?.apiCallEditing?.prompt || getDefaultApiCall());

  return story(args, context);
};

export default SvelteStoresDecorator;
