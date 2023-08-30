# Refactoring a SvelteKit page

Let's say you have a SvelteKit page such as `src-svelte/src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import { getApiKeys } from "$lib/bindings";

  let api_keys = getApiKeys();
</script>

<section>
  <table>
    <tr>
      <th class="header-text" colspan="2">API Keys</th>
    </tr>
    <tr>
      <td>OpenAI</td>
      <td class="key">
        {#await api_keys}
          ...loading
        {:then keys}
          {#if keys.openai !== undefined && keys.openai !== null}
            {keys.openai.value}
          {:else}
            <span class="unset">not set</span>
          {/if}
        {:catch error}
          error: {error}
        {/await}
      </td>
    </tr>
  </table>
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    flex: 0.6;
  }

  table {
    width: 0.1%;
    white-space: nowrap;
  }

  th {
    color: var(--color-header);
  }

  th,
  td {
    padding: 0 0.5rem;
    text-align: left;
  }

  .key {
    font-weight: bold;
    text-transform: lowercase;
  }

  .unset {
    color: var(--color-faded);
  }
</style>

```

You can refactor this out into:

```svelte
<script lang="ts">
  import ApiKeysDisplay from "./api_keys_display.svelte";
</script>

<section>
  <ApiKeysDisplay />
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    flex: 0.6;
  }
</style>

```

where `src-svelte/src/routes/api_keys_display.svelte` looks like:

```svelte
<script lang="ts">
  import { getApiKeys } from "$lib/bindings";

  let api_keys = getApiKeys();
</script>

<table>
  <tr>
    <th class="header-text" colspan="2">API Keys</th>
  </tr>
  <tr>
    <td>OpenAI</td>
    <td class="key">
      {#await api_keys}
        ...loading
      {:then keys}
        {#if keys.openai !== undefined && keys.openai !== null}
          {keys.openai.value}
        {:else}
          <span class="unset">not set</span>
        {/if}
      {:catch error}
        error: {error}
      {/await}
    </td>
  </tr>
</table>

<style>
  table {
    width: 0.1%;
    white-space: nowrap;
  }

  th {
    color: var(--color-header);
  }

  th,
  td {
    padding: 0 0.5rem;
    text-align: left;
  }

  .key {
    font-weight: bold;
    text-transform: lowercase;
  }

  .unset {
    color: var(--color-faded);
  }
</style>

```

and `src-svelte/src/routes/homepage.test.ts` is renamed to `src-svelte/src/routes/api_keys_display.test.ts` with the corresponding import from

```ts
import ApiKeysDisplay from "./api_keys_display.svelte";
```

rather than the previous `+page.svelte`.
