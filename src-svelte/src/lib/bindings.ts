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

export function chat(args: ChatArgs) {
  return invoke()<LightweightLlmCall>("chat", { args });
}

export function getApiCall(id: string) {
  return invoke()<LlmCall>("get_api_call", { id });
}

export function getApiCalls(offset: number) {
  return invoke()<LightweightLlmCall[]>("get_api_calls", { offset });
}

export function importDb(path: string) {
  return invoke()<DatabaseImportCounts>("import_db", { path });
}

export function exportDb(path: string) {
  return invoke()<DatabaseCounts>("export_db", { path });
}

export function runCommand(command: string) {
  return invoke()<RunCommandResponse>("run_command", { command });
}

export function sendCommandInput(sessionId: string, input: string) {
  return invoke()<string>("send_command_input", { sessionId, input });
}

export type TokenMetadata = {
  prompt: number | null;
  response: number | null;
  total: number | null;
};
export type Request = { prompt: Prompt; temperature: number };
export type ChatMessage =
  | { role: "System"; text: string }
  | { role: "Human"; text: string }
  | { role: "AI"; text: string };
export type Llm = { name: string; requested: string; provider: Service };
export type VariantMetadata = {
  canonical?: LlmCallReference | null;
  variants?: LlmCallReference[];
  sibling_variants?: LlmCallReference[];
};
export type ApiKeys = { openai: string | null };
export type Shell = "Bash" | "Zsh" | "PowerShell";
export type Prompt = ({ type: "Chat" } & ChatPrompt) | { type: "Unknown" };
export type ChatArgs = {
  provider: Service;
  llm: string;
  temperature?: number | null;
  prompt: ChatMessage[];
  previous_call_id?: string | null;
  canonical_id?: string | null;
};
export type DatabaseCounts = { num_api_keys: number; num_llm_calls: number };
export type Service = "OpenAI" | "Ollama" | { Unknown: string };
export type OS = "Mac" | "Linux" | "Windows";
export type SystemInfo = {
  zamm_version: string;
  os: OS | null;
  shell: Shell | null;
  shell_init_file: string | null;
};
export type Preferences = {
  version?: string | null;
  animations_on?: boolean | null;
  background_animation?: boolean | null;
  animation_speed?: number | null;
  transparency_on?: boolean | null;
  high_dpi_adjust?: boolean | null;
  sound_on?: boolean | null;
  volume?: number | null;
};
export type LightweightLlmCall = {
  id: EntityId;
  timestamp: string;
  response_message: ChatMessage;
};
export type EntityId = string;
export type ChatPrompt = { messages: ChatMessage[] };
export type Response = { completion: ChatMessage };
export type DatabaseImportCounts = {
  imported: DatabaseCounts;
  ignored: DatabaseCounts;
};
export type ConversationMetadata = {
  previous_call?: LlmCallReference | null;
  next_calls?: LlmCallReference[];
};
export type LlmCallReference = { id: EntityId; snippet: string };
export type Sound = "Switch" | "Whoosh";
export type RunCommandResponse = {
  id: EntityId;
  timestamp: string;
  output: string;
};
export type LlmCall = {
  id: EntityId;
  timestamp: string;
  llm: Llm;
  request: Request;
  response: Response;
  tokens: TokenMetadata;
  conversation?: ConversationMetadata;
  variation?: VariantMetadata;
};
