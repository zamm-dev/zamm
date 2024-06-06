<p class="empty atomic-reveal">
  <slot />
</p>

<style>
  .empty {
    color: var(--color-faded);
    font-size: 0.85rem;
    font-style: italic;
    text-align: center;
  }

  .empty :global(a) {
    color: var(--color-faded);
    text-decoration: underline;
  }
</style>
