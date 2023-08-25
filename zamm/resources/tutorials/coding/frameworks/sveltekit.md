# Using SvelteKit

## Layout

Layout pages apply to all child routes, which can plug in their own components into `<slot />`. Observe `src-svelte/src/routes/+layout.svelte` in the sample SvelteKit app:

```svelte
<script>
  import Header from "./Header.svelte";
  import "./styles.css";
</script>

<div class="app">
  <Header />

  <main>
    <slot />
  </main>

  <footer>
    <p>
      visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit
    </p>
  </footer>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  ...
</style>

```

## Fonts

See [this page](https://khromov.se/adding-locally-hosted-google-fonts-to-your-sveltekit-project/).

```bash
$ yarn add @fontsource/teko
```

Then add it to `src-svelte/src/routes/+layout.svelte` so that it is available throughout the project. For example:

```
<script>
  ...
  import "@fontsource/teko";
</script>
```

Then reference it in the CSS of whichever component you'd like to use it in.

```css
  p {
    font-family: 'Teko', sans-serif;
    font-size: 20px;
    color: #000;
  }
```
