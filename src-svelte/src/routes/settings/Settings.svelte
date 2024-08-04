<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import SettingsSwitch from "./SettingsSwitch.svelte";
  import SettingsSlider from "./SettingsSlider.svelte";
  import {
    animationsOn,
    animationSpeed,
    transparencyOn,
    highDpiAdjust,
    backgroundAnimation,
    soundOn,
    volume,
  } from "$lib/preferences";
  import { setPreferences } from "$lib/bindings";

  const onAnimationsToggle = (newValue: boolean) => {
    setPreferences({
      animations_on: newValue,
    });
  };

  const onbackgroundAnimationToggle = (newValue: boolean) => {
    setPreferences({
      background_animation: newValue,
    });
  };

  const onAnimationSpeedUpdate = (newValue: number) => {
    setPreferences({
      animation_speed: newValue,
    });
  };

  const onTransparencyToggle = (newValue: boolean) => {
    setPreferences({
      transparency_on: newValue,
    });
  };

  const onHighDpiAdjust = (newValue: boolean) => {
    setPreferences({
      high_dpi_adjust: newValue,
    });
  };

  const onSoundToggle = (newValue: boolean) => {
    setPreferences({
      sound_on: newValue,
    });
  };

  const onVolumeUpdate = (newValue: number) => {
    setPreferences({
      volume: newValue,
    });
  };
</script>

<InfoBox title="Settings">
  <div class="container">
    <SubInfoBox subheading="Animation">
      <SettingsSwitch
        label="Enabled"
        bind:toggledOn={$animationsOn}
        onToggle={onAnimationsToggle}
      />
      <SettingsSwitch
        label="Background"
        bind:toggledOn={$backgroundAnimation}
        onToggle={onbackgroundAnimationToggle}
      />
      <SettingsSlider
        label="General speed"
        min={0.1}
        max={1}
        bind:value={$animationSpeed}
        onUpdate={onAnimationSpeedUpdate}
      />
    </SubInfoBox>
  </div>

  <div class="container">
    <SubInfoBox subheading="Other visual effects">
      <SettingsSwitch
        label="Transparency"
        bind:toggledOn={$transparencyOn}
        onToggle={onTransparencyToggle}
      />
      <SettingsSwitch
        label="High DPI adjust"
        bind:toggledOn={$highDpiAdjust}
        onToggle={onHighDpiAdjust}
      />
    </SubInfoBox>
  </div>

  <div class="container">
    <SubInfoBox subheading="Sound">
      <SettingsSwitch
        label="Enabled"
        bind:toggledOn={$soundOn}
        onToggle={onSoundToggle}
      />
      <SettingsSlider
        label="Volume"
        min={0}
        max={2}
        onUpdate={onVolumeUpdate}
        bind:value={$volume}
      />
    </SubInfoBox>
  </div>
</InfoBox>

<style>
  .container {
    margin-top: 1rem;
  }

  .container:first-of-type {
    margin-top: 0;
  }

  .container :global(.sub-info-box .content) {
    --side-padding: 0.8rem;
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.1rem;
    margin: 0.5rem calc(-1 * var(--side-padding));
  }

  /* this takes sidebar width into account */
  @media (min-width: 52rem) {
    .container :global(.sub-info-box .content) {
      grid-template-columns: 1fr 1fr;
    }
  }

  @media (min-width: 43.5rem) {
    :global(.high-dpi-adjust) .container :global(.sub-info-box .content) {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
