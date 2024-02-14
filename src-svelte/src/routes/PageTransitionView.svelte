<script lang="ts">
  import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import PageTransition from "./PageTransition.svelte";

  let routeA = true;

  function toggleRoute() {
    routeA = !routeA;
  }

  $: currentRoute = routeA ? "/a.html" : "/b.html";
</script>

<MockAppLayout>
  <button class="route-toggle" on:click={toggleRoute}>Toggle route</button>
  <PageTransition {currentRoute} {...$$restProps}>
    {#if routeA}
      <InfoBox title="Simulation">
        <p>
          How do we know that even the realest of realities wouldn't be
          subjective, in the final analysis? Nobody can prove his existence, can
          he? &mdash; <em>Simulacron 3</em>
        </p>
      </InfoBox>
    {:else}
      <InfoBox title="Reality: Subjective or Objective?">
        <SubInfoBox subheading="Stuart Candy">
          <p>
            It is better to be surprised by a simulation, rather than blindsided
            by reality.
          </p>
        </SubInfoBox>

        <SubInfoBox subheading="Jean Baudrillard">
          <p>
            It is no longer a question of a false representation of reality
            (ideology) but of concealing the fact that the real is no longer
            real, and thus of saving the reality principle.
          </p>

          <p>
            And once freed from reality, we can produce the 'realer than real' -
            hyperrealism.
          </p>
        </SubInfoBox>
      </InfoBox>
    {/if}
  </PageTransition>
</MockAppLayout>

<style>
  .route-toggle {
    margin-bottom: 1rem;
  }
</style>
