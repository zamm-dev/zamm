<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import { standardDuration } from "$lib/preferences";
  import { onMount } from "svelte";
  import Creditor from "./Creditor.svelte";
  import Grid from "./Grid.svelte";

  let scrollInterval: NodeJS.Timeout | undefined = undefined;
  let scrollElement: HTMLDivElement | undefined = undefined;

  function stopScrolling() {
    if (scrollInterval) {
      clearInterval(scrollInterval);
    }
  }

  onMount(() => {
    const mainContainer = document.getElementById("main-container");
    if (mainContainer) {
      scrollElement = mainContainer as HTMLDivElement;
      setTimeout(() => {
        scrollInterval = setInterval(function () {
          if (scrollElement) {
            scrollElement.scrollBy(0, 1);
          }
        }, 10);
      }, 4.4 * $standardDuration);
    }

    return function () {
      stopScrolling();
    };
  });
</script>

<div
  class="container"
  on:mousedown={stopScrolling}
  on:wheel={stopScrolling}
  role="none"
>
  <InfoBox title="Credits">
    <div class="subinfo-container">
      <SubInfoBox
        subheading="Zen and the Automation of Metaprogramming for the Masses"
      >
        <Creditor name="Amos Jun-yeung Ng" url="https://github.com/amosjyng/" />
      </SubInfoBox>
    </div>
    <div class="subinfo-container">
      <SubInfoBox subheading="Frameworks">
        <Grid>
          <Creditor name="Tauri" logo="tauri.png" url="https://tauri.app/" />
          <Creditor name="Svelte" logo="svelte.svg" url="https://svelte.dev/" />
        </Grid>
      </SubInfoBox>
      <SubInfoBox subheading="Frontend">
        <Grid>
          <Creditor
            name="Neodrag"
            logo="neodrag.svg"
            url="https://www.neodrag.dev/"
          />
          <Creditor
            name="Autosize"
            url="https://www.jacklmoore.com/autosize/"
          />
          <Creditor name="Nano ID" url="https://github.com/ai/nanoid" />
          <Creditor
            name="Storybook"
            logo="storybook.svg"
            url="https://storybook.js.org/"
          />
          <Creditor name="Vite" logo="vite.svg" url="https://vitejs.dev/" />
          <Creditor name="Vite" logo="vitest.svg" url="https://vitest.dev/" />
          <Creditor
            name="Playwright"
            logo="playwright.svg"
            url="https://playwright.dev/"
          />
          <Creditor
            name="WebdriverIO"
            logo="webdriverio.svg"
            url="https://webdriver.io/"
          />
          <Creditor
            name="Jest-Image-Snapshot"
            logo="jest-image-snapshot.png"
            url="https://github.com/americanexpress/jest-image-snapshot"
          />
        </Grid>
      </SubInfoBox>
      <SubInfoBox subheading="Backend">
        <Grid>
          <Creditor name="Serde" url="https://serde.rs/" />
          <Creditor name="Diesel" logo="diesel.png" url="https://diesel.rs/" />
          <Creditor
            name="async-openai"
            logo="async-openai.png"
            url="https://github.com/64bit/async-openai"
          />
          <Creditor
            name="Specta"
            logo="specta.png"
            url="https://github.com/oscartbeaumont/specta"
          />
          <Creditor name="rvcr" url="https://github.com/ChorusOne/rvcr" />
          <Creditor
            name="reqwest-middleware"
            url="https://github.com/TrueLayer/reqwest-middleware"
          />
        </Grid>
      </SubInfoBox>
      <SubInfoBox subheading="General Tooling">
        <Grid>
          <Creditor
            name="quicktype"
            logo="quicktype.png"
            url="https://quicktype.io/"
          />
          <Creditor
            name="Docker"
            logo="docker.svg"
            url="https://www.docker.com/"
          />
        </Grid>
      </SubInfoBox>
    </div>
  </InfoBox>

  <div class="dedication-container">
    <InfoBox title="Dedication" childNumber={1}>
      <p class="atomic-reveal">
        This project is made with ❤️, and dedicated to <strong
          >Eliza Lynn Kith</strong
        >.
      </p>
    </InfoBox>
  </div>
</div>

<style>
  .container {
    padding-bottom: 1rem;
  }

  .subinfo-container {
    margin-top: 1rem;
  }

  .dedication-container {
    margin-top: 2rem;
  }
</style>
