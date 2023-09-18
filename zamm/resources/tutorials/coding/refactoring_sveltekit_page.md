## SvelteKit refactoring

# Refactoring a SvelteKit page into components

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

## Refactoring a component into a library

Say you have a component at `src-svelte/src/routes/Metadata.svelte` such as:

```svelte
<div class="container">
  <div class="border-box"></div>
  <div class="background-box"></div>
  <svg style="visibility: hidden; position: absolute;" width="0" height="0" xmlns="http://www.w3.org/2000/svg" version="1.1">
    <defs>
        <filter id="round">
            <feGaussianBlur in="SourceGraphic" stdDeviation="3" result="blur" />    
            <feColorMatrix in="blur" mode="matrix" values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 19 -9" result="goo" />
            <feComposite in="SourceGraphic" in2="goo" operator="atop"/>
        </filter>
    </defs>
</svg>

  <div class="info-box">
    <h2>System Information</h2>
    <table>
      <tr>
        <th colspan="2">ZAMM</th>
      </tr>
      <tr>
        <td>Version</td>
        <td class="version-value">0.0.0</td>
      </tr>
      <tr>
        <td>Stability</td>
        <td class="stability-value">Unstable (Alpha)</td>
      </tr>
      <tr>
        <td>Fork</td>
        <td>Original</td>
      </tr>
    </table>

    <table>
      <tr>
        <th colspan="2">Computer</th>
      </tr>
      <tr>
        <td>OS</td>
        <td>Linux</td>
      </tr>
      <tr>
        <td>Release</td>
        <td>Ubuntu 18.04</td>
      </tr>
    </table>
  </div>
</div>

<style>
  .container {
    position: relative;
    flex: 1;
  }

  .border-box {
    width: 100%;
    height: 100%;
    position: absolute; /* absolute positioning inside the relative container */
    top: 0;
    left: 0;
    filter: url(#round);
    z-index: 0; /* to ensure it stays below the info-box */
  }

  .border-box::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--color-border);
    mask:
      linear-gradient(-45deg, transparent 0 calc(1rem + 1px), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 calc(1rem + 1px), #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }

  .background-box {
    width: calc(100% - 1px);
    height: calc(100% - 1px);
    position: absolute; /* absolute positioning inside the relative container */
    top: 1;
    left: 1;
    filter: url(#round);
    z-index: 1; /* to ensure it stays below the info-box */
  }

  .background-box::before {
    content: "";
    position: absolute;
    top: 1px;
    left: 1px;
    right: 0;
    bottom: 0;
    background: white;
    mask:
      linear-gradient(-45deg, transparent 0 1rem, #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 1rem, #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }

  .info-box {
    position: relative;
    z-index: 2;
    padding: 1rem;
  }

  .info-box h2 {
    margin: -0.25rem 0 0 1rem
  }

  table {
    margin-top: 0.5rem;
  }

  th,
  td {
    text-align: left;
    padding-left: 0;
  }

  td {
    vertical-align: text-top;
  }

  td:first-child {
    color: var(--color-faded);
    padding-right: 1rem;
  }

  .stability-value {
    color: var(--color-caution);
  }
</style>

```

and you want to refactor the border effect out for use in other components. Then create a `src-svelte/src/lib/InfoBox.svelte` which will hold the common component:

```svelte
<script lang="ts">
  export let title = "";
</script>

<div class="container">
  <div class="border-box"></div>
  <div class="background-box"></div>
  <svg style="visibility: hidden; position: absolute;" width="0" height="0" xmlns="http://www.w3.org/2000/svg" version="1.1">
    <defs>
        <filter id="round">
            <feGaussianBlur in="SourceGraphic" stdDeviation="3" result="blur" />    
            <feColorMatrix in="blur" mode="matrix" values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 19 -9" result="goo" />
            <feComposite in="SourceGraphic" in2="goo" operator="atop"/>
        </filter>
    </defs>
  </svg>

  <div class="info-box">
    <h2>{title}</h2>
    <slot />
  </div>
</div>

<style>
  .container {
    position: relative;
    flex: 1;
  }

  .border-box {
    width: 100%;
    height: 100%;
    position: absolute; /* absolute positioning inside the relative container */
    top: 0;
    left: 0;
    filter: url(#round);
    z-index: 0; /* to ensure it stays below the info-box */
  }

  .border-box::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--color-border);
    mask:
      linear-gradient(-45deg, transparent 0 calc(1rem + 1px), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 calc(1rem + 1px), #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }

  .background-box {
    width: calc(100% - 1px);
    height: calc(100% - 1px);
    position: absolute;
    top: 1;
    left: 1;
    filter: url(#round);
    z-index: 1;
  }

  .background-box::before {
    content: "";
    position: absolute;
    top: 1px;
    left: 1px;
    right: 0;
    bottom: 0;
    background: white;
    mask:
      linear-gradient(-45deg, transparent 0 1rem, #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 1rem, #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }

  .info-box {
    position: relative;
    z-index: 2;
    padding: 1rem;
  }

  .info-box h2 {
    margin: -0.25rem 0 0 1rem
  }
</style>

```

and then the original file becomes

```svelte
<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
</script>

<InfoBox title="System Information">
  <table>
    <tr>
      <th colspan="2">ZAMM</th>
    </tr>
    <tr>
      <td>Version</td>
      <td class="version-value">0.0.0</td>
    </tr>
    <tr>
      <td>Stability</td>
      <td class="stability-value">Unstable (Alpha)</td>
    </tr>
    <tr>
      <td>Fork</td>
      <td>Original</td>
    </tr>
  </table>

  <table>
    <tr>
      <th colspan="2">Computer</th>
    </tr>
    <tr>
      <td>OS</td>
      <td>Linux</td>
    </tr>
    <tr>
      <td>Release</td>
      <td>Ubuntu 18.04</td>
    </tr>
  </table>
</InfoBox>

<style>
  table {
    margin-top: 0.5rem;
  }

  th,
  td {
    text-align: left;
    padding-left: 0;
  }

  td {
    vertical-align: text-top;
  }

  td:first-child {
    color: var(--color-faded);
    padding-right: 1rem;
  }

  .stability-value {
    color: var(--color-caution);
  }
</style>

```
