// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/

export const commands = {
  async getApiKeys(): Promise<Result<ApiKeys, Error>> {
    try {
      return { status: "ok", data: await TAURI_INVOKE("get_api_keys") };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async setApiKey(
    filename: string | null,
    service: Service,
    apiKey: string,
  ): Promise<Result<null, Error>> {
    try {
      return {
        status: "ok",
        data: await TAURI_INVOKE("set_api_key", { filename, service, apiKey }),
      };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async playSound(sound: Sound, volume: number, speed: number): Promise<void> {
    await TAURI_INVOKE("play_sound", { sound, volume, speed });
  },
  async getPreferences(): Promise<Preferences> {
    return await TAURI_INVOKE("get_preferences");
  },
  async setPreferences(preferences: Preferences): Promise<Result<null, Error>> {
    try {
      return {
        status: "ok",
        data: await TAURI_INVOKE("set_preferences", { preferences }),
      };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async getSystemInfo(): Promise<SystemInfo> {
    return await TAURI_INVOKE("get_system_info");
  },
  async chat(args: ChatArgs): Promise<Result<LightweightLlmCall, Error>> {
    try {
      return { status: "ok", data: await TAURI_INVOKE("chat", { args }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async getApiCall(id: string): Promise<Result<LlmCall, Error>> {
    try {
      return { status: "ok", data: await TAURI_INVOKE("get_api_call", { id }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async getApiCalls(
    offset: number,
  ): Promise<Result<LightweightLlmCall[], Error>> {
    try {
      return {
        status: "ok",
        data: await TAURI_INVOKE("get_api_calls", { offset }),
      };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async importDb(path: string): Promise<Result<DatabaseImportCounts, Error>> {
    try {
      return { status: "ok", data: await TAURI_INVOKE("import_db", { path }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async exportDb(path: string): Promise<Result<DatabaseCounts, Error>> {
    try {
      return { status: "ok", data: await TAURI_INVOKE("export_db", { path }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async runCommand(
    command: string,
  ): Promise<Result<RunCommandResponse, Error>> {
    try {
      return {
        status: "ok",
        data: await TAURI_INVOKE("run_command", { command }),
      };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
  async sendCommandInput(
    sessionId: string,
    input: string,
  ): Promise<Result<string, Error>> {
    try {
      return {
        status: "ok",
        data: await TAURI_INVOKE("send_command_input", { sessionId, input }),
      };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: "error", error: e as any };
    }
  },
};

/** user-defined events **/

/** user-defined constants **/

/** user-defined types **/

export type ApiKeys = { openai: string | null };
export type ChatArgs = {
  provider: Service;
  llm: string;
  temperature?: number | null;
  prompt: ChatMessage[];
  previous_call_id?: string | null;
  canonical_id?: string | null;
};
export type ChatMessage =
  | { role: "System"; text: string }
  | { role: "Human"; text: string }
  | { role: "AI"; text: string };
export type ChatPrompt = { messages: ChatMessage[] };
export type ConversationMetadata = {
  previous_call?: LlmCallReference | null;
  next_calls?: LlmCallReference[];
};
export type DatabaseCounts = { num_api_keys: number; num_llm_calls: number };
export type DatabaseImportCounts = {
  imported: DatabaseCounts;
  ignored: DatabaseCounts;
};
export type EntityId = string;
export type Error =
  | { UnexpectedOpenAiResponse: { reason: string } }
  | { MissingApiKey: { service: Service } }
  | { FutureZammImport: { version: string; import_error: ImportError } }
  | { GenericImport: { source: ImportError } }
  | { Poison: Record<string, never> }
  | { Serde: { source: SerdeError } }
  | { Rodio: { source: RodioError } }
  | { Uuid: string }
  | { Diesel: string }
  | { Reqwest: string }
  | { OpenAI: string }
  | { Ollama: string }
  | { Tauri: string }
  | { Io: string }
  | { Other: string };
export type ImportError = { UnknownPromptType: Record<string, never> };
export type LightweightLlmCall = {
  id: EntityId;
  timestamp: string;
  response_message: ChatMessage;
};
export type Llm = { name: string; requested: string; provider: Service };
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
export type LlmCallReference = { id: EntityId; snippet: string };
export type OS = "Mac" | "Linux" | "Windows";
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
export type Prompt = ({ type: "Chat" } & ChatPrompt) | { type: "Unknown" };
export type Request = { prompt: Prompt; temperature: number };
export type Response = { completion: ChatMessage };
export type RodioError =
  | { Stream: string }
  | { Decode: string }
  | { Play: string };
export type RunCommandResponse = {
  id: EntityId;
  timestamp: string;
  output: string;
};
export type SerdeError = { Json: string } | { Yaml: string } | { Toml: string };
export type Service = "OpenAI" | "Ollama" | { Unknown: string };
export type Shell = "Bash" | "Zsh" | "PowerShell";
export type Sound = "Switch" | "Whoosh";
export type SystemInfo = {
  zamm_version: string;
  os: OS | null;
  shell: Shell | null;
  shell_init_file: string | null;
};
export type TokenMetadata = {
  prompt: number | null;
  response: number | null;
  total: number | null;
};
export type VariantMetadata = {
  canonical?: LlmCallReference | null;
  variants?: LlmCallReference[];
  sibling_variants?: LlmCallReference[];
};

/** tauri-specta globals **/

import {
  invoke as TAURI_INVOKE,
  Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>,
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>,
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: null extends T
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>,
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindow__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindow__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    },
  );
}
