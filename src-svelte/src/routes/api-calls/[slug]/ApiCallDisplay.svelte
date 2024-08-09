<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import { type LlmCall } from "$lib/bindings";
  import Loading from "$lib/Loading.svelte";
  import IconLeftArrow from "~icons/mingcute/left-fill";
  import IconRightArrow from "~icons/mingcute/right-fill";
  import Prompt from "./Prompt.svelte";
  import ApiCallReference from "$lib/ApiCallReference.svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";

  export let dateTimeLocale: string | undefined = undefined;
  export let timeZone: string | undefined = undefined;
  export let apiCall: LlmCall | undefined = undefined;

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

  function updateDisplayStrings(apiCall: LlmCall | undefined) {
    if (!apiCall) {
      return;
    }

    const timestamp = apiCall.timestamp + "Z";
    const date = new Date(timestamp);
    humanTime = formatter.format(date);

    temperature = apiCall.request.temperature.toFixed(2);
  }

  function getThisAsRef(apiCall: LlmCall | undefined) {
    if (!apiCall) {
      return;
    }

    return {
      id: apiCall.id,
      snippet: apiCall.response.completion.text,
    };
  }

  function extractProvider(apiCall: LlmCall | undefined) {
    if (!apiCall) {
      return;
    }

    const provider = apiCall.llm.provider;
    if (typeof provider === "string") {
      return provider;
    } else {
      return provider["Unknown"];
    }
  }

  $: updateDisplayStrings(apiCall);
  $: previousCall = apiCall?.conversation?.previous_call;
  $: nextCalls = apiCall?.conversation?.next_calls ?? [];
  $: thisAsRef = getThisAsRef(apiCall);
  $: variants =
    apiCall?.variation?.variants ?? apiCall?.variation?.sibling_variants ?? [];
  $: provider = extractProvider(apiCall);
</script>

<InfoBox title="API Call">
  {#if apiCall}
    <table class="composite-reveal">
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
          {#if provider}
            (via {provider})
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

    {#if apiCall.request.prompt.type === "Chat"}
      <Prompt prompt={apiCall.request.prompt} />
    {:else}
      <EmptyPlaceholder
        >Prompt not shown because it is from an incompatible future version of
        ZAMM.</EmptyPlaceholder
      >
    {/if}

    <SubInfoBox subheading="Response">
      <pre class="response">{apiCall?.response.completion.text ??
          "Unknown"}</pre>
    </SubInfoBox>

    {#if apiCall?.variation !== undefined}
      <SubInfoBox subheading="Variants">
        <div class="variation-links composite-reveal">
          {#if apiCall.variation.canonical}
            <ApiCallReference
              selfContained
              apiCall={apiCall.variation.canonical}
            />
          {:else if thisAsRef}
            <ApiCallReference selfContained nolink apiCall={thisAsRef} />
          {/if}

          <ul>
            {#each variants as variant}
              <li>
                <ApiCallReference
                  selfContained
                  nolink={variant.id === apiCall.id}
                  apiCall={variant}
                />
              </li>
            {/each}
          </ul>
        </div>
      </SubInfoBox>
    {/if}

    {#if apiCall?.conversation !== undefined}
      <SubInfoBox subheading="Conversation">
        <div class="conversation-links composite-reveal">
          <div class="conversation previous-links composite-reveal">
            {#if previousCall !== undefined}
              <a
                class="conversation link previous atomic-reveal"
                href={`/api-calls/${previousCall?.id}`}
              >
                <div class="arrow-icon"><IconLeftArrow /></div>
                <div class="snippet">{previousCall?.snippet}</div>
              </a>
            {/if}
          </div>

          <div class="conversation next-links composite-reveal">
            {#each nextCalls as nextCall}
              <a
                class="conversation link next atomic-reveal"
                href={`/api-calls/${nextCall.id}`}
              >
                <div class="snippet">{nextCall.snippet}</div>
                <div class="arrow-icon"><IconRightArrow /></div>
              </a>
            {/each}
          </div>
        </div>
      </SubInfoBox>
    {/if}
  {:else}
    <Loading />
  {/if}
</InfoBox>

<style>
  td {
    text-align: left;
  }

  td:first-child {
    color: var(--color-faded);
    padding-right: 1rem;
  }

  pre {
    white-space: pre-wrap;
    word-wrap: break-word;
    word-break: break-word;
  }

  .variation-links ul {
    margin: 0;
    padding: 0;
    list-style-type: none;
  }

  .variation-links li {
    --indent: 2rem;
    --line-thickness: 2px;
    margin-left: var(--indent);
    width: calc(100% - var(--indent));
    position: relative;
  }

  .variation-links li:before {
    content: "";
    position: absolute;
    bottom: 50%;
    left: calc(-1 * var(--indent) + 0.5rem);
    width: 1rem;
    height: 150%;
    border-bottom: var(--line-thickness) solid var(--color-border);
    border-left: var(--line-thickness) solid var(--color-border);
    border-bottom-left-radius: var(--corner-roundness);
  }

  .variation-links li:first-child:before {
    height: 50%;
  }

  .variation-links li :global(span.nolink) {
    font-style: italic;
  }

  .conversation-links {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .conversation.previous-links,
  .conversation.next-links {
    flex: none;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  @media (min-width: 46rem) {
    .conversation-links {
      flex-direction: row;
    }

    .conversation.previous-links,
    .conversation.next-links {
      flex: 1;
    }
  }

  @media (min-width: 38.5rem) {
    :global(.high-dpi-adjust) .conversation-links {
      flex-direction: row;
    }

    :global(.high-dpi-adjust) .conversation.previous-links,
    :global(.high-dpi-adjust) .conversation.next-links {
      flex: 1;
    }
  }

  .conversation.link {
    border: 1px solid var(--color-border);
    padding: 0.75rem;
    border-radius: var(--corner-roundness);
    color: black;
    display: flex;
    flex-direction: row;
    gap: 0.75rem;
    align-items: center;
  }

  .conversation.link .arrow-icon {
    display: block;
    margin-top: 0.3rem;
  }

  .conversation.link .arrow-icon :global(svg) {
    transform: scale(1.5);
    color: var(--color-faded);
  }

  .conversation.link .snippet {
    flex: 1;
    text-align: start;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    line-clamp: 2;
    text-overflow: ellipsis;
    overflow: hidden;
  }
</style>
