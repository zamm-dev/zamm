<div class="outer-container">
  <div class="button-container cut-corners outer">
    <slot />
  </div>
</div>

<style>
  .outer-container {
    width: fit-content;
    margin: 0 auto;
  }

  .button-container {
    display: flex;
  }

  .button-container :global(.left-end) {
    --cut-top-left: 8px;
  }

  .button-container :global(.right-end) {
    --cut-bottom-right: 8px;
  }

  @media (max-width: 35rem) {
    .button-container {
      flex-direction: column;
    }
  }
</style>
