import { type ChatPrompt } from "./bindings";

export type ChatPromptVariant = { type: "Chat" } & ChatPrompt;
