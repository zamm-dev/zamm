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
  @media (min-width: 46rem) {
    .credits-grid {
      grid-template-columns: 1fr 1fr;
      justify-items: end;
    }
  }

  @media (min-width: 38.5rem) {
    :global(.high-dpi-adjust) .credits-grid {
      grid-template-columns: 1fr 1fr;
      justify-items: end;
    }
  }

  /* this takes sidebar width into account */
  @media (min-width: 55rem) {
    .credits-grid {
      justify-items: center;
    }
  }

  @media (min-width: 46rem) {
    :global(.high-dpi-adjust) .credits-grid {
      justify-items: center;
    }
  }

  /* this takes sidebar width into account */
  @media (min-width: 64rem) {
    .credits-grid {
      grid-template-columns: 1fr 1fr 1fr;
      justify-items: center;
    }
  }

  @media (min-width: 53.5rem) {
    :global(.high-dpi-adjust) .credits-grid {
      grid-template-columns: 1fr 1fr 1fr;
      justify-items: center;
    }
  }
</style>
