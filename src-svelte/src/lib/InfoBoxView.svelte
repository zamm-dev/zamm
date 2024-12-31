<script lang="ts">
  import InfoBox from "./InfoBox.svelte";
  interface Props {
    [key: string]: any;
  }

  let { ...rest }: Props = $props();
</script>

<InfoBox {...rest}>
  <p class="atomic-reveal">
    How do we know that even the realest of realities wouldn't be subjective, in
    the final analysis? Nobody can prove his existence, can he? &mdash; <em
      >Simulacron 3</em
    >
  </p>
</InfoBox>
