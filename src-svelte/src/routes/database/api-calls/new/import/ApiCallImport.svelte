<script lang="ts" context="module">
  import type { ChatMessage } from "$lib/bindings";

  function determineRole(role: string): "Human" | "System" | "AI" {
    switch (role.toLowerCase()) {
      case "human":
      case "user":
        return "Human";
      case "system":
        return "System";
      case "ai":
      case "assistant":
        return "AI";
      default:
        throw new Error(`Invalid role: ${role}`);
    }
  }

  function parseLmStudioExport(data: string): ChatMessage[] {
    const parsedImport = JSON.parse(data);
    if (!Array.isArray(parsedImport)) {
      throw new Error("Root element must be an array");
    }

    return parsedImport.map((message) => {
      if (typeof message !== "object") {
        throw new Error("Array element must be an object");
      }

      const role = determineRole(message.role);
      if (
        typeof message.content !== "string" &&
        typeof message.text !== "string"
      ) {
        throw new Error("No content found in message");
      }

      const parsedMessage: ChatMessage = {
        role,
        text: message.content ?? message.text,
      };

      return parsedMessage;
    });
  }

  function parseOllamaCliOutput(data: string): ChatMessage[] {
    const lines = data.trim().split("\n");
    if (!lines[0].startsWith(">>> ")) {
      throw new Error("First line must start with '>>> '");
    }

    let messages: ChatMessage[] = [];
    let currentRole: "Human" | "AI" = "Human";
    let currentMessage = "";
    for (const line of lines) {
      const nextRole =
        line.startsWith(">>> ") || line.startsWith("... ") ? "Human" : "AI";
      const nextMessage = nextRole === "Human" ? line.substring(4) : line;
      if (nextRole !== currentRole) {
        if (currentMessage) {
          messages.push({
            role: currentRole,
            text: currentMessage.trim(),
          });
        }

        currentRole = nextRole;
        currentMessage = nextMessage;
      } else {
        if (nextMessage === "") {
          currentMessage += "\n\n";
        } else {
          currentMessage += nextMessage;
        }
      }
    }

    if (currentMessage) {
      messages.push({
        role: currentRole,
        text: currentMessage.trim(),
      });
    }
    return messages;
  }

  export function parseImportData(data: string): ChatMessage[] {
    try {
      return parseOllamaCliOutput(data);
    } catch (error) {
      return parseLmStudioExport(data);
    }
  }
</script>

<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import Button from "$lib/controls/Button.svelte";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import { canonicalRef, prompt } from "../ApiCallEditor.svelte";
  import { goto } from "$app/navigation";

  let importData = "";

  function importConversation() {
    try {
      const messages = parseImportData(importData);

      canonicalRef.set(undefined);
      prompt.set({
        type: "Chat",
        messages,
      });
      goto("/database/api-calls/new");
    } catch (error) {
      snackbarError(error as string | Error);
    }
  }
</script>

<InfoBox title="Import API Call" fullHeight>
  <p>Import an existing conversation by:</p>
  <ul>
    <li class="atomic-reveal">
      Copy-pasting Ollama terminal output, starting with "
      <pre>&gt;&gt;&gt; </pre>
      "
    </li>
    <li>Clicking LM Studio's "Export" button &gt; "Copy as JSON"</li>
  </ul>

  <textarea
    rows="10"
    bind:value={importData}
    placeholder="Paste your conversation here..."
  ></textarea>

  <div class="action atomic-reveal">
    <Button on:click={importConversation}>Import</Button>
  </div>
</InfoBox>

<style>
  p {
    margin: 0;
  }

  ul {
    margin: 0 0 0.5rem 0;
  }

  p,
  ul {
    text-align: left;
  }

  pre {
    display: inline;
    font-family: var(--font-mono);
  }

  textarea {
    box-sizing: border-box;
    width: 100%;
    margin-bottom: 1rem;
    font-family: var(--font-mono);
    font-size: 1rem;
    background-color: hsla(0, 0%, 95%, 1);
    padding: 0.5rem;
    border-radius: var(--corner-roundness);
    resize: none;
    flex: 1;
  }

  .action {
    width: 100%;
    display: flex;
    justify-content: center;
  }

  .action :global(button) {
    width: 100%;
    max-width: 25rem;
  }
</style>
