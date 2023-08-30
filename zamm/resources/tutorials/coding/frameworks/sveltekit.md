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

## SVGs

Follow [this answer](https://stackoverflow.com/a/67341665). If you have an SVG file `zamm.svg` that starts with:

```svg
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 20010904//EN"
              "http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd">

<svg xmlns="http://www.w3.org/2000/svg"
     width="0.306667in" height="0.0833333in"
     viewBox="0 0 92 25">
     ...
</svg>
```

then copy it to `src-svelte/src/lib/zamm.svelte` while stripping out the first two lines.

Then, in whichever file you want to include it:

```svelte
<script>
  ...
  import ZammSvg from "$lib/zamm.svelte";
</script>

...
<svelte:component this={ZammSvg} />
...
```

If you need the SVG to be larger, you can edit the `width` and leave out the `height`.
