<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import { type LlmCall, getApiCall } from "$lib/bindings";
  import Loading from "$lib/Loading.svelte";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Button from "$lib/controls/Button.svelte";
  import { conversation } from "../../chat/Chat.svelte";
  import { goto } from "$app/navigation";

  export let id: string;
  export let dateTimeLocale: string | undefined = undefined;
  export let timeZone: string | undefined = undefined;
  let apiCall: LlmCall | undefined = undefined;

  const formatter = new Intl.DateTimeFormat(dateTimeLocale, {
    year: "numeric",
    month: "long",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
    second: "numeric",
    hour12: true,
    timeZone,
  });

  let humanTime: string | undefined = undefined;
  let temperature: string | undefined = undefined;
  let apiCallPromise = getApiCall(id)
    .then((retrievedApiCall) => {
      apiCall = retrievedApiCall;
      const timestamp = apiCall.timestamp + "Z";
      const date = new Date(timestamp);
      humanTime = formatter.format(date);
      temperature = apiCall.request.temperature.toFixed(2);

      return apiCall;
    })
    .catch((error) => {
      snackbarError(error);
    });

  function restoreConversation() {
    if (!apiCall) {
      snackbarError("API call not yet loaded");
      return;
    }

    const restoredConversation = [
      ...apiCall.request.prompt.messages,
      apiCall.response.completion,
    ];
    conversation.set(restoredConversation);

    goto("/chat");
  }
</script>

<InfoBox title="API Call">
  {#await apiCallPromise}
    <Loading />
  {:then apiCall}
    <table>
      <tr>
        <td>ID</td>
        <td>{apiCall?.id ?? "Unknown"}</td>
      </tr>
      <tr>
        <td>Time</td>
        <td>{humanTime ?? "Unknown"}</td>
      </tr>
      <tr>
        <td>LLM</td>
        <td>
          {apiCall?.llm.requested ?? "Unknown"}
          {#if apiCall?.llm.requested !== apiCall?.llm.name}
            → {apiCall?.llm.name}
          {/if}
        </td>
      </tr>
      <tr>
        <td>Temperature</td>
        <td>
          {temperature ?? "Unknown"}
        </td>
      </tr>
      <tr>
        <td>Tokens</td>
        <td>
          {apiCall?.tokens.prompt ?? "Unknown"} prompt +
          {apiCall?.tokens.response ?? "Unknown"} response =
          {apiCall?.tokens.total ?? "Unknown"} total tokens
        </td>
      </tr>
    </table>

    <SubInfoBox subheading="Prompt">
      <div class="prompt">
        {#each apiCall?.request.prompt.messages ?? [] as message}
          <div class={"message " + message.role.toLowerCase()}>
            <span class="role">{message.role}</span>
            <pre>{message.text}</pre>
          </div>
        {/each}
      </div>
    </SubInfoBox>

    <SubInfoBox subheading="Response">
      <pre class="response">{apiCall?.response.completion.text ??
          "Unknown"}</pre>
    </SubInfoBox>
  {/await}
</InfoBox>

<div class="actions-container">
  <InfoBox title="Actions" childNumber={1}>
    <div class="action-buttons">
      <Button text="Restore conversation" on:click={restoreConversation} />
    </div>
  </InfoBox>
</div>

<style>
  td:first-child {
    color: var(--color-faded);
    padding-right: 1rem;
  }

  .prompt {
    margin-bottom: 1rem;
  }

  .message {
    margin: 0.5rem 0;
    padding: 0.5rem;
    border-radius: var(--corner-roundness);
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .message.system {
    background-color: var(--color-system);
  }

  .message.human {
    background-color: var(--color-human);
  }

  .message.ai {
    background-color: var(--color-ai);
  }

  .message .role {
    color: var(--color-faded);
    width: 4rem;
    min-width: 4rem;
    text-align: center;
  }

  pre {
    white-space: pre-wrap;
    font-family: var(--font-mono);
    margin: 0;
    text-align: left;
  }

  .actions-container {
    margin-top: 1rem;
  }

  .action-buttons {
    width: fit-content;
    margin: 0 auto;
  }
</style>
