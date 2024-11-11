<script lang="ts">
  import getComponentId from "./label-id";

  interface Props {
    subheading: string;
    children?: import("svelte").Snippet;
  }

  let { subheading, children }: Props = $props();
  const subinfoboxId = getComponentId("subinfobox");

  const children_render = $derived(children);
</script>

<section class="sub-info-box composite-reveal" aria-labelledby={subinfoboxId}>
  <div class="subheading">
    <h3 id={subinfoboxId}>{subheading}</h3>
  </div>
  <div class="content composite-reveal">
    {@render children_render?.()}
  </div>
</section>

<style>
  .subheading {
    width: 100%;
    text-align: center;
    margin: 0.5rem 0;
  }

  .content :global(p:first-child) {
    margin-top: 0;
  }
</style>
