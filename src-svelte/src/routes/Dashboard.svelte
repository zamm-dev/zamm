<script lang="ts">
  import ApiKeysDisplay from "./components/api-keys/Display.svelte";
  import Metadata from "./components/Metadata.svelte";
</script>

<section class="homepage-banner">
  <img src="/logo.png" alt="Robot typing on keyboard" />
  <Metadata childNumber={0} />
</section>

<section>
  <ApiKeysDisplay childNumber={1} />
</section>

<style>
  .homepage-banner {
    display: flex;
    flex-direction: row;
    margin-bottom: 1rem;
    justify-content: space-evenly;
    flex-wrap: wrap;
    gap: 1rem;
  }

  img {
    height: 15rem;
    width: 19rem;
    object-fit: cover;
  }
</style>
