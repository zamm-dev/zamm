<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import Scrollable from "$lib/Scrollable.svelte";
  import { standardDuration } from "$lib/preferences";
  import Creditor from "./Creditor.svelte";
  import Grid from "./Grid.svelte";

  let scrollDelay = 4.4 * $standardDuration + 1_000;
</script>

<div class="container full-height">
  <div class="credits-container full-height">
    <InfoBox title="Credits" fullHeight>
      <Scrollable autoscroll {scrollDelay}>
        <div class="subinfo-container">
          <SubInfoBox
            subheading="Zen and the Automation of Metaprogramming for the Masses"
          >
            <Grid>
              <Creditor
                isPerson
                name="Amos Jun-yeung Ng"
                logo="amos-ng.jpg"
                url="https://github.com/amosjyng/"
              />
            </Grid>
          </SubInfoBox>
          <SubInfoBox subheading="Frameworks">
            <Grid>
              <Creditor
                name="Tauri"
                logo="tauri.png"
                url="https://tauri.app/"
              />
              <Creditor
                name="Svelte"
                logo="svelte.svg"
                url="https://svelte.dev/"
              />
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
              <Creditor
                name="Vite"
                logo="vitest.svg"
                url="https://vitest.dev/"
              />
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
              <Creditor
                name="Diesel"
                logo="diesel.png"
                url="https://diesel.rs/"
              />
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
              <Creditor
                name="Pre-commit"
                logo="pre-commit.svg"
                url="https://pre-commit.com/"
              />
            </Grid>
          </SubInfoBox>
          <SubInfoBox subheading="Fonts">
            <Grid>
              <Creditor
                name="Nasalization"
                url="https://typodermicfonts.com/nasalization/"
              />
              <Creditor
                name="Good Timing"
                url="https://typodermicfonts.com/good-timing/"
              />
            </Grid>
          </SubInfoBox>
          <SubInfoBox subheading="Icons">
            <Grid>
              <Creditor
                name="Iconify"
                logo="iconify.png"
                url="https://iconify.design/"
              />
              <Creditor
                name="Unplugin"
                logo="unplugin.svg"
                url="https://unplugin.vercel.app/"
              />
              <Creditor
                name="Element+"
                logo="element-plus.png"
                url="https://github.com/element-plus/element-plus-icons"
              />
              <Creditor
                name="Phosphor Icons"
                logo="phosphor-icons.svg"
                url="https://phosphoricons.com/"
              />
              <Creditor
                name="Material Icons"
                logo="google.svg"
                url="https://github.com/google/material-design-icons"
              />
              <Creditor
                name="Tabler Icons"
                logo="tabler.svg"
                url="https://tabler.io/icons"
              />
              <Creditor
                name="Ionicons"
                url="https://github.com/ionic-team/ionicons"
              />
              <Creditor
                name="Font Awesome"
                logo="font-awesome.svg"
                url="https://fontawesome.com/"
              />
              <Creditor
                name="Vaadin"
                logo="vaadin.svg"
                url="https://vaadin.com/"
              />
            </Grid>
          </SubInfoBox>
        </div>
      </Scrollable>
    </InfoBox>
  </div>

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
    gap: 1.5rem;
  }

  .credits-container :global(.info-content),
  .credits-container :global(.growable) {
    height: 100%;
  }

  .credits-container
    :global(.sub-info-box:last-child .creditor:nth-last-child(1)) {
    padding-bottom: 0;
  }

  /* this takes sidebar width into account */
  @media (min-width: 46rem) {
    .credits-container
      :global(.sub-info-box:last-child .creditor:nth-last-child(2)) {
      padding-bottom: 0;
    }
  }

  /* this takes sidebar width into account */
  @media (min-width: 64rem) {
    .credits-container
      :global(.sub-info-box:last-child .creditor:nth-last-child(3)) {
      padding-bottom: 0;
    }
  }

  .subinfo-container {
    margin-top: 1rem;
  }
</style>
