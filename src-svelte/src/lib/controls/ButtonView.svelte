<script lang="ts">
  import Button from "./Button.svelte";

  export let text: string;
</script>

<Button {...$$restProps}>{text}</Button>
