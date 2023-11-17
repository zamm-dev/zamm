# Homepage

## Storybook

To test out the homepage display in Storybook, we move all the code from `src-svelte/src/routes/+page.svelte` to `src-svelte/src/routes/Homepage.svelte`, and then import the new homepage component in `src-svelte/src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import Homepage from "./Homepage.svelte";
</script>

<Homepage />

```

In the process of setting up a Storybook story, we find that we need yet another mock page transition element if we want `src-svelte/src/lib/__mocks__/MockPageTransitions.svelte`:

```ts
<script lang="ts">
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { animationSpeed} from "$lib/preferences";

  firstAppLoad.set(true);
  firstPageLoad.set(true);
  animationSpeed.set(0.1);
</script>

<div class="storybook-wrapper">
  <PageTransition currentRoute="/storybook-demo">
    <slot />
  </PageTransition>
</div>

<style>
  .storybook-wrapper {
    --base-animation-speed: 0.1;
  }
</style>

```

Now the page will finally animate as if we're navigating to it. We can now create a Storybook story for the homepage at `src-svelte/src/routes/Homepage.stories.ts`:

```ts
import HompageComponent from "./Homepage.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";

export default {
  component: HompageComponent,
  title: "Screens/Homepage",
  argTypes: {},
  decorators: [
    TauriInvokeDecorator,
    (story: StoryFn) => {
      return {
        Component: MockPageTransitions,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: HompageComponent,
  props: args,
});

export const Homepage: StoryObj = Template.bind({}) as any;
Homepage.parameters = {
  resolution: {
    openai: null,
  },
};
```

## Renaming to Dashboard

Upon realizing that this was already called the "Dashboard" for Storybook purposes, we rename both files and their imports.

## API keys display

We refactor `src-svelte/src/routes/ApiKeysDisplay.svelte` to contain just loading text, until the API call comes in:

```svelte
<script lang="ts">
  import { getApiKeys } from "$lib/bindings";
  import InfoBox from "$lib/InfoBox.svelte";
  import Service from "./Service.svelte";

  let api_keys = getApiKeys();
</script>

<InfoBox title="API Keys" {...$$restProps}>
  {#await api_keys}
    <span class="loading">...loading</span>
  {:then keys}
    <div class="api-keys">
      <Service name="OpenAI" apiKey={keys.openai?.value} />
    </div>
  {:catch error}
    error: {error}
  {/await}
</InfoBox>

<style>
  span.loading {
    color: var(--color-faded);
  }
</style>

```

We refactor the per-row display code to `src-svelte/src/routes/Service.svelte`:

```svelte
<script lang="ts">
  export let name: string;
  export let apiKey: string | undefined;

  $: active = apiKey !== undefined;
  $: label = active ? "Active" : "Inactive";
</script>

<div class="container">
  <div class="service">{name}</div>
  <div class="api-key" class:active>{label}</div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }

  .service {
    text-align: left;
    font-family: var(--font-body);
    flex: 1;
  }

  .api-key {
    text-align: center;
    text-transform: uppercase;
    font-family: var(--font-body);
    background-color: gray;
    color: white;
    flex: 1;
    border-radius: var(--corner-roundness);
  }

  .api-key.active {
    background-color: green;
  }
</style>

```

Next, we notice that the new table contents appear immediately instead of fading in slowly. This is because they are no longer subject to the same animations as before. We update `src-svelte/src/lib/InfoBox.svelte` to account for this, first by making `getNodeAnimations` free of side-effects to make it easier to reason about, and then by adding a new `MutationObserver` to the node for the duration of the animation effect:

```ts
    const getNodeAnimations = (currentNode: Element): RevealContent[] => {
      ...
      if (
        ...
      ) {
        return [
          new RevealContent({
            node: currentNode,
            timing: getChildKickoffFraction(currentNode),
          }),
        ];
      } else {
        const revealAnimations: RevealContent[] = [];
        for (const child of currentNode.children) {
          revealAnimations.push(...getNodeAnimations(child));
        }
        return revealAnimations;
      }
    };

    let revealAnimations = getNodeAnimations(node);

    const config = { childList: true, subtree: true };
    const mutationCallback: MutationCallback = () => {
      revealAnimations = getNodeAnimations(node);
      // hide all new nodes immediately
      revealAnimations.forEach((anim) => {
        anim.tickForGlobalTime(0);
      });
    };
    const observer = new MutationObserver(mutationCallback);
    observer.observe(node, config);

    return {
      ...,
      tick: (tGlobalFraction: number) => {
        ...

        if (tGlobalFraction === 1) {
          observer.disconnect();
        }
      },
    };
```

Finally, as a nit, we edit `src-svelte/src/routes/Dashboard.svelte` to change the spacing on the main page by removing `flex: 1;` from `.metadata-contanier` and adding

```css
  .homepage-banner {
    ...
    justify-content: space-evenly;
  }
```

Next, we allow it to wrap on smaller screens, and remove `margin-left: 1rem;` from `.metadata-container` because the spacing should be 1rem even when wrapped vertically:

```css
  .homepage-banner {
    flex-wrap: wrap;
    gap: 1rem;
  }
```

Since there are now multiple Svelte files for this one component, we move `src-svelte/src/routes/ApiKeysDisplay.svelte` to `src-svelte/src/routes/components/api-keys/Display.svelte`, along with associated Storybook stories and Vitest tests. We move it inside `components` to distinguish complicated components that span multiple files from sub-paths in the app URLs. Storybook may need to be restarted due to indexing problems.

### Adding a form

We create a mock form at `src-svelte/src/routes/components/api-keys/Form.svelte`, with the requisite CSS transitions:

```svelte
<script lang="ts">
  import { cubicInOut } from "svelte/easing";
  import { animationSpeed, animationsOn } from "$lib/preferences";

  let saveKey = true;

  function growY(node: HTMLElement) {
    const height = node.offsetHeight;
    const duration = $animationsOn ? 100 / $animationSpeed : 0;
    return {
      duration,
      easing: cubicInOut,
      css: (t: number) => {
        const value = height * t;
        return `height: ${value}px;`;
      },
    }
  }
</script>

<div class="container" transition:growY>
  <form>
    <div class="form-row">
      <label for="apiKey">API key:</label>
      <input type="text" id="apiKey" name="apiKey">
    </div>

    <div class="form-row">
      <input type="checkbox" id="saveKey" name="saveKey" checked={saveKey}>
      <label for="saveKey">Save key to:</label>
      <input type="text" id="saveKeyInput" name="saveKeyInput">
    </div>

    <input type="submit" value="Save">
  </form>
</div>

<style>
  .container {
    --horizontal-overshoot: 1rem;
    overflow: hidden;
    box-sizing: border-box;
    margin: 0 calc(-1 * var(--horizontal-overshoot));
  }

  form {
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
    margin: 0.5rem 0;
    padding: 0.5rem var(--horizontal-overshoot);
    background-color: var(--color-background);
    margin-bottom: 0.5rem;

    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex-wrap: nowrap;
  }

  label {
    white-space: nowrap;
  }

  .form-row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  input[type=text] {
    flex: 1;
    min-width: 1rem;
    border: none;
    border-bottom: 1px solid var(--color-border);
    background-color: var(--color-background);
    font-family: var(--font-mono);
    font-weight: bold;
    font-size: 1rem;
    transition: border-bottom calc(0.05s / var(--base-animation-speed)) linear;
  }

  input[type=text]:focus {
    outline: none;
    border-bottom: 1px solid blue;
  }

  input[type=submit] {
    align-self: flex-start;
  }
</style>
```

We add this to `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
<script lang="ts">
  import Form from "./Form.svelte";

  ...
  let editing = false;

  function toggleEditing() {
    editing = !editing;
  }

  ...
</script>

<div class="container">
  <div class="row" on:click={toggleEditing} on:keypress={toggleEditing} role="row" tabindex="0">
    <div class="service">{name}</div>
    <div class="api-key" class:active>{label}</div>
  </div>

  {#if editing}
    <Form />
  {/if}
</div>

<style>
  .row {
    ...
    cursor: pointer;
  }

  ...
</style>
```

We should test this as part of our screenshot snapshots. To avoid having to interact with the component as part of our screenshot tests, we'll allow editing to be controlled externally in `src-svelte/src/routes/components/api-keys/Service.svelte`:

```ts
  export let editing = false;
```

Next, we add a demo prop to `src-svelte/src/routes/components/api-keys/Display.svelte` to control the editing:

```svelte
<script lang="ts">
  ...
  export let editDemo = false;
  ...
</script>

...
      <Service ... editing={editDemo} />
```

and a new story to `src-svelte/src/routes/components/api-keys/Display.stories.ts`:

```ts
export const Editing: StoryObj = Template.bind({}) as any;
Editing.args = {
  editDemo: true,
};
Editing.parameters = {
  resolution: knownKeys,
  viewport: {
    defaultViewport: "mobile2",
  },
};
```

and a test to `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["screens", "dashboard", "api-keys-display"],
    variants: [..., "editing"],
    ...
  },
  ...
];
```

We realize we should display the existing key if it is available, so we edit `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts">
  ...
  export let apiKey = "";
  ...
</script>

...
      <input ... value={apiKey}>
```

and pass it in through `src-svelte/src/routes/components/api-keys/Service.svelte`:

```svelte
    <Form {apiKey} />
```
