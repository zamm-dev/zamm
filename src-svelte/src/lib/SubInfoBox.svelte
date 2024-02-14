<script lang="ts">
  import getComponentId from "./label-id";

  export let subheading: string;
  const subinfoboxId = getComponentId("subinfobox");
</script>

<section class="sub-info-box" aria-labelledby={subinfoboxId}>
  <div class="subheading">
    <h3 id={subinfoboxId}>{subheading}</h3>
  </div>
  <div class="content">
    <slot />
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
