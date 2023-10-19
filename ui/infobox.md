# Information boxes

## Sub-information

After implementing the `SubInfoBox` as described in [`settings.md`](/ui/settings.md), we tweak it to center the h3 subheadings:

```svelte
<section ...>
  <div class="subheading">
    <h3 id={subinfoboxId}>{subheading}</h3>
  </div>
  ...
</section>

<style>
  .subheading {
    width: 100%;
    text-align: center;
  }
</style>
```

We can now remove this from `src-svelte/src/routes/settings/Settings.svelte`:

```css
  .container :global(h3) {
    margin-left: var(--side-padding);
  }
```
