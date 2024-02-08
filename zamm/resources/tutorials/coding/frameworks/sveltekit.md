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

## Proprietary fonts

If you want to use proprietary fonts in your app, but you wish to keep your app open source, you'll have to tell Git to ignore the proprietary fonts folder.

Download the font into `src-svelte/static/fonts`.

Then, follow the instructions [here](https://stackoverflow.com/a/70400854) to use the fonts inside the app. Remember that if you want to actually release this app, you will have to pay app licensing fees. See the different common licensing options [here](https://typodermicfonts.com/license/).

So for example, if your font is located at `src-svelte/static/fonts/nasalization-rg.otf`, do this to use it:

```css
@font-face {
  font-family: 'Nasalization';
  font-style: normal;
  font-weight: 400;
  src: url('/fonts/nasalization-rg.otf') format("opentype");
}
```

If you want to activate certain font features for a given CSS element:

```css
.element {
  font-family: 'Nasalization', sans-serif;
  font-feature-settings: "salt" on;
}
```

To see what features your font has, you can visit [wakamaifondue.com](https://wakamaifondue.com/).

If your font has multiple weights, you can add them in multiple entries, like so:

```css
@font-face {
  font-family: 'Nasalization';
  font-style: normal;
  font-weight: 300;
  src: url('/fonts/nasalization-lt.otf') format("opentype");
}
```

### Making proprietary fonts available to CI

Create a separate private repo for storing fonts. Put your font files in this repo. Commit them and upload.

Then in your main repo:

```bash
$ git rm -rf src-svelte/static/fonts
$ git submodule add git@github.com:amosjyng/zamm-fonts.git src-svelte/static/fonts
```

Make sure to add submodule init to your repo documentation.

Then in your workflows such as `.github/workflows/tests.yaml`, you have to change this:

```yaml
      - uses: actions/checkout@v3
```

to this:

```yaml
      - uses: actions/checkout@v3
        with:
          token: ${{ secrets.FONTS_PAT }}
          submodules: 'recursive'
```

Note that this requires a personal access token (PAT) to grant one repo access to another private repo. Go [here](https://github.com/settings/tokens/new). Select "repo" permissions. Click "Generate".

Now go [here](https://github.com/amosjyng/zamm/settings/secrets/actions) and add that under `FONTS_PAT`.

#### Fine-grained PAT

Go [here](https://github.com/settings/personal-access-tokens/new). Select `Only select repositories` and pick the repo `amosjyng/zamm-fonts` "Contents" to "Access: Read-only". Click "Generate".

Observe that you get the error

```
  remote: Write access to repository not granted.
  Error: fatal: unable to access 'https://github.com/amosjyng/zamm-ui/': The requested URL returned error: 403
  The process '/usr/bin/git' failed with exit code 128
```

on the CI run. It appeaers submodule cloning in CI environments is not yet supported for the beta feature of fine-grained PATs.

## Refactoring components

If you have built a new component in your library and wish to apply it to another one, you can follow this as an example for what to do. Say you have built an `InfoBox` component that is located at `src-svelte/src/lib/InfoBox.svelte`. Say you have an existing table like this:

```svelte
<table>
  <tr class="h2">
    <th class="header-text" colspan="2">API Keys</th>
  </tr>
  <tr>
    <td>OpenAI</td>
    <td class="key">
      {#await api_keys}
        ...loading
      {:then keys}
        {#if keys.openai !== undefined && keys.openai !== null}
          <span class="actual-key">{keys.openai.value}</span>
        {:else}
          unknown
        {/if}
      {:catch error}
        error: {error}
      {/await}
    </td>
  </tr>
</table>
```

Then import the new InfoBox:

```svelte
<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
</script>
```

and change the existing table implementation to use it instead:

```svelte
<InfoBox title="API Keys">
  <table>
    <tr>
      <td>OpenAI</td>
      <td class="key">
        {#await api_keys}
          ...loading
        {:then keys}
          {#if keys.openai !== undefined && keys.openai !== null}
            <span class="actual-key">{keys.openai.value}</span>
          {:else}
            unknown
          {/if}
        {:catch error}
          error: {error}
        {/await}
      </td>
    </tr>
  </table>
</InfoBox>
```

This requires understanding what the InfoBox is for, and what information the existing table is meant to convey.
