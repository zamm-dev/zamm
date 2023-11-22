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

#### Styling the text input

We follow [this example](https://codepen.io/maheshambure21/pen/EozKKy) from [here](https://freefrontend.com/css-input-text/), and refactor and create `src-svelte/src/lib/controls/TextInput.svelte` to style the text input:

```svelte
<script lang="ts">
  export let name: string;
  export let value: string;
</script>

<div class="fancy-input">
  <input type="text" id={name} {name} {value} />
  <span class="focus-border"></span>
</div>

<style>
  .fancy-input {
    position: relative;
    flex: 1;
  }

  input[type="text"] {
    min-width: 1rem;
    width: 100%;
    border: none;
    border-bottom: 1px solid var(--color-border);
    background-color: var(--color-background);
    font-family: var(--font-mono);
    font-weight: bold;
    font-size: 1rem;
  }

  input[type="text"]:focus {
    outline: none;
  }

  input[type="text"] + .focus-border {
    position: absolute;
    bottom: -1px;
    left: 0;
    width: 0;
    height: 2px;
    background-color: blue;
    transition: width calc(0.05s / var(--base-animation-speed)) ease-out;
  }

  input[type="text"]:focus + .focus-border {
    width: 100%;
  }
</style>
```

Then, we use this new input in `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts">
  ...
  import TextInput from "$lib/controls/TextInput.svelte";

  ...
  export let saveKeyLocation = "";
  ...
</script>

...
  <form>
    <div ...>
      ...
      <TextInput name="apiKey" value={apiKey} />
    </div>

    <div ...>
      ...
      <label for="saveKeyLocation">Save key to:</label>
      <TextInput name="saveKeyLocation" value={saveKeyLocation} />
    </div>

    ...
  </form>
```

#### Styling the button

We create a button element at `src-svelte/src/lib/controls/Button.svelte`:

```svelte
<script lang="ts">
  export let text: string;
</script>

<button class="outer">
  <div class="inner">{text}</div>
</button>

<style>
  .outer, .inner {
    --cut: 7px;
    --background-color: var(--color-background);
    --border-color: #ccc;
    --border: 0.15rem;
    --diagonal-border: calc(var(--border) * 0.8);
    font-size: 0.9rem;
    font-family: var(--font-body);
    border: var(--border) solid var(--border-color);
    text-transform: uppercase;
    background:
      linear-gradient(-45deg, var(--border-color) 0 calc(var(--cut) + var(--diagonal-border)), var(--background-color) 0) bottom right / 50% 100%,
      linear-gradient(135deg, var(--border-color) 0 calc(var(--cut) + var(--diagonal-border)), var(--background-color) 0) top left / 50% 100%;
    background-origin:border-box;
    background-repeat: no-repeat;
    -webkit-mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    -webkit-mask-size: 51% 100%;
    -webkit-mask-repeat: no-repeat;
    mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
    transition: all calc(0.05s / var(--base-animation-speed)) ease-out;
  }

  .inner {
    padding: 5px 10px;
  }

  .inner:hover {
    filter: brightness(1.05);
  }

  .inner:active {
    transform: translateY(0.08rem) scale(0.98);
  }

  .outer {
    --background-color: #eee;
    --border: 2px;
    --diagonal-border: 2.5px;
    --cut: 8px;
    padding: 1px;
    display: inline-block;
  }
</style>

```

Note that we use a brightness filter on hover because changes to the background gradient colors don't get animated. The brightness filter lights up the entire element, including borders, but that is an acceptable look for our purposes.

We also create `src-svelte/src/lib/controls/Button.stories.ts`:

```ts
import Button from "./Button.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";

export default {
  component: Button,
  title: "Reusable/Button",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Button,
  props: args,
});

export const Regular: StoryObj = Template.bind({}) as any;
Regular.args = {
  text: "Simulate",
};

```

We also add this as a test to `src-svelte/src/routes/storybook.test.ts`:

```ts
const components: ComponentTestConfig[] = [
  ...
  {
    path: ["reusable", "button"],
    variants: ["regular"],
  },
  ...
];
```

Finally, we use this new element in `src-svelte/src/routes/components/api-keys/Form.svelte`:

```svelte
<script lang="ts">
  ...
  import Button from "$lib/controls/Button.svelte";
  ...
</script>

...
      <div class="save-button">
        <Button text="Save" />
      </div>
...

<style>
  ...
  .save-button {
    align-self: flex-start;
  }
</style>
```

#### Fixing corner overlap and bottom shadow

If we play the animation slowly, we see that 1) the corner of the form overlaps the supposed border of the div for a while until the animation finishes, and 2) there is no shadow on the bottom of the inset until the animation finishes playing. We fix this by moving the box shadow from the form to a containing element, and by first growing the padding before growing the container for the form:

```svelte
<script lang="ts">
  ...
  function growY(node: HTMLElement) {
    const rem = 18;
    const totalFinalPadding = 1 * rem;
    ...
    return {
      ...
      css: (t: number) => {
        const totalHeight = height * t;
        const totalCurrentPadding = Math.min(totalFinalPadding, totalHeight);
        const contentHeight = totalHeight - totalCurrentPadding;
        return `
          --vertical-padding: ${totalCurrentPadding / 2}px;
          --form-height: ${contentHeight}px;
        `;
      },
    };
  }
</script>

<div class="container" transition:growY>
  <div class="inset-container">
    <form>
      ...
    </form>
  </div>
</div>

<style>
  .container {
    --form-height: 100%;
    --vertical-padding: 0.5rem;
    --horizontal-overshoot: 1rem;
    overflow: hidden;
    margin: 0 calc(-1 * var(--horizontal-overshoot));
    padding: var(--vertical-padding) 0;
  }

  .inset-container {
    height: var(--form-height);
    overflow: hidden;
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
    background-color: var(--color-background);
  }

  form {
    padding: 0.5rem var(--horizontal-overshoot);
    ...
  }
  ...
</style>
```

#### Fixing animation on Firefox

We notice that the animation isn't running for some reason on Firefox. As such, we implement it by manually changing the style:

```ts
  function growY(node: HTMLElement) {
    ...
    const duration = $animationsOn ? 200 / $animationSpeed : 0;
    return {
      ...
      tick: (t: number) => {
        ...
        node.style.setProperty("--vertical-padding", `${totalCurrentPadding / 2}px`);
        node.style.setProperty("--form-height", `${contentHeight}px`);
      }
    };
  }
```

Note that we're now using the `tick` function instead of the `css` function, and we doubled the duration to make its speed feel consistent with the rest of the page.
