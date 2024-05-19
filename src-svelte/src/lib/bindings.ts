/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>,
    ): Promise<T>;
  }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function getApiKeys() {
  return invoke()<ApiKeys>("get_api_keys");
}

export function setApiKey(
  filename: string | null,
  service: Service,
  apiKey: string,
) {
  return invoke()<null>("set_api_key", { filename, service, apiKey });
}

export function playSound(sound: Sound, volume: number, speed: number) {
  return invoke()<null>("play_sound", { sound, volume, speed });
}

export function getPreferences() {
  return invoke()<Preferences>("get_preferences");
}

export function setPreferences(preferences: Preferences) {
  return invoke()<null>("set_preferences", { preferences });
}

export function getSystemInfo() {
  return invoke()<SystemInfo>("get_system_info");
}

export function chat(
  provider: Service,
  llm: string,
  temperature: number | null,
  prompt: ChatMessage[],
) {
  return invoke()<LightweightLlmCall>("chat", {
    provider,
    llm,
    temperature,
    prompt,
  });
}

export function getApiCall(id: string) {
  return invoke()<LlmCall>("get_api_call", { id });
}

export function getApiCalls(offset: number) {
  return invoke()<LightweightLlmCall[]>("get_api_calls", { offset });
}

export type ChatMessage =
  | { role: "System"; text: string }
  | { role: "Human"; text: string }
  | { role: "AI"; text: string };
export type Prompt = { type: "Chat" } & ChatPrompt;
export type Response = { completion: ChatMessage };
export type Service = "OpenAI";
export type Request = { prompt: Prompt; temperature: number };
export type Preferences = {
  animations_on?: boolean | null;
  background_animation?: boolean | null;
  animation_speed?: number | null;
  transparency_on?: boolean | null;
  sound_on?: boolean | null;
  volume?: number | null;
};
export type Llm = { name: string; requested: string; provider: Service };
export type LightweightLlmCall = {
  id: EntityId;
  timestamp: string;
  response_message: ChatMessage;
};
export type LlmCall = {
  id: EntityId;
  timestamp: string;
  llm: Llm;
  request: Request;
  response: Response;
  tokens: TokenMetadata;
};
export type ApiKeys = { openai: string | null };
export type OS = "Mac" | "Linux" | "Windows";
export type Shell = "Bash" | "Zsh" | "PowerShell";
export type SystemInfo = {
  zamm_version: string;
  os: OS | null;
  shell: Shell | null;
  shell_init_file: string | null;
};
export type ChatPrompt = { messages: ChatMessage[] };
export type TokenMetadata = {
  prompt: number | null;
  response: number | null;
  total: number | null;
};
export type Sound = "Switch" | "Whoosh";
export type EntityId = { uuid: string };
