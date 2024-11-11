<script lang="ts">
  import IconClose from "~icons/ep/close-bold";

  interface Props {
    dismiss: () => void;
    message: string;
    messageType: "error" | "info";
  }

  let { dismiss, message, messageType }: Props = $props();
</script>

<div class={"snackbar " + messageType} role="alertdialog">
  {message}<button onclick={dismiss} title="Dismiss">
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
