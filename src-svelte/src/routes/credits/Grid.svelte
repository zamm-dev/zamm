<div class="credits-grid">
  <slot />
</div>

<style>
  .credits-grid {
    margin: -0.5rem auto 0;
    display: grid;
    grid-template-columns: 1fr;
    justify-items: center;
  }

  /* this takes sidebar width into account */
  @media (min-width: 40rem) {
    .credits-grid {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
