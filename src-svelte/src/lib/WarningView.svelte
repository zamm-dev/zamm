<script lang="ts">
  import Warning from "./Warning.svelte";

  interface Props {
    text: string;
  }

  let { text }: Props = $props();
</script>

<Warning>{text}</Warning>
