<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";

  interface SnackbarMessage {
    id: number;
    msg: string;
  }

  export const snackbars: Writable<SnackbarMessage[]> = writable([]);
  export let messageDurationMs = 5_000;
  let baseAnimationDurationMs = 100;
  let animateDurationMs = baseAnimationDurationMs;

  function setBaseAnimationDurationMs(newDurationMs: number) {
    baseAnimationDurationMs = newDurationMs;
  }

  export function clearAllMessages() {
    snackbars.set([]);
  }

  let nextId = 0;

  // Function to show a new snackbar message
  export function snackbarError(msg: string) {
    animateDurationMs = baseAnimationDurationMs;
    const id = nextId++;
    snackbars.update((current) => [...current, { id, msg }]);

    // Auto-dismiss after 'duration'
    setTimeout(() => {
      dismiss(id);
    }, messageDurationMs);
  }

  // Function to manually dismiss a snackbar
  function dismiss(id: number) {
    animateDurationMs = 2 * baseAnimationDurationMs;
    snackbars.update((current) =>
      current.filter((snackbar) => snackbar.id !== id),
    );
  }
</script>

<script lang="ts">
  import { standardDuration } from "$lib/preferences";
  import { fly, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import Message from "./Message.svelte";

  $: setBaseAnimationDurationMs($standardDuration);
</script>

<div class="snackbars">
  {#each $snackbars as snackbar (snackbar.id)}
    <div
      in:fly|global={{ y: "1rem", duration: $standardDuration }}
      out:fade|global={{ duration: $standardDuration }}
      animate:flip={{ duration: animateDurationMs }}
    >
      <Message dismiss={() => dismiss(snackbar.id)} message={snackbar.msg} />
    </div>
  {/each}
</div>

<style>
  .snackbars {
    z-index: 100;
    width: 100%;
    position: fixed;
    bottom: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>
