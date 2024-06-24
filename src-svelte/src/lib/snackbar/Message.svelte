<script lang="ts">
  import IconClose from "~icons/ep/close-bold";

  export let dismiss: () => void;
  export let message: string;
  export let messageType: "error" | "info";
</script>

<div class={"snackbar " + messageType} role="alertdialog">
  {message}<button on:click={dismiss} title="Dismiss">
    <IconClose />
  </button>
</div>

<style>
  .snackbar {
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    color: white;
    border-radius: 4px;
    width: fit-content;
    margin: 0 auto;
  }

  .snackbar.error {
    background-color: var(--color-error-background);
    filter: drop-shadow(0px 1px 4px var(--color-error-shadow));
  }

  .snackbar.info {
    background-color: var(--color-info-background);
    filter: drop-shadow(0px 1px 4px var(--color-info-shadow));
  }

  button {
    background: none;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0.5rem;
    margin: -0.5rem;
    margin-top: -0.3rem;
    align-self: flex-center;
  }
</style>
