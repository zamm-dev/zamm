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
    flex-direction: column;
  }

  .button-container :global(.left-end) {
    --cut-top-left: 8px;
  }

  .button-container :global(.right-end) {
    --cut-bottom-right: 8px;
  }

  @media (min-width: 35rem),
    only screen and (-webkit-min-device-pixel-ratio: 2) and (min-width: 26rem) {
    .button-container {
      flex-direction: row;
    }
  }
</style>
