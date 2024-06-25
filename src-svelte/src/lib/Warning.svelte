<script lang="ts">
  import IconWarning from "~icons/streamline/warning-octagon-solid";
</script>

<div class="warning atomic-reveal">
  <div class="icon">
    <IconWarning />
  </div>
  <div class="text">
    <slot />
  </div>
</div>

<style>
  .warning {
    background: var(--color-caution-background);
    padding: 0.5rem 1rem;
    margin-bottom: 1rem;
    border-radius: var(--corner-roundness);
    display: flex;
    flex-direction: row;
    gap: 1rem;
    align-items: center;
  }

  .icon :global(svg) {
    transform: scale(1.2);
    display: block;
  }

  .text {
    text-align: left;
  }
</style>
