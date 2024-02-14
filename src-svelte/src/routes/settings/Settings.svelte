<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import SettingsSwitch from "./SettingsSwitch.svelte";
  import SettingsSlider from "./SettingsSlider.svelte";
  import {
    animationsOn,
    animationSpeed,
    backgroundAnimation,
    soundOn,
    volume,
    NullPreferences,
  } from "$lib/preferences";
  import { setPreferences } from "$lib/bindings";

  const onAnimationsToggle = (newValue: boolean) => {
    setPreferences({
      ...NullPreferences,
      animations_on: newValue,
    });
  };

  const onbackgroundAnimationToggle = (newValue: boolean) => {
    setPreferences({
      ...NullPreferences,
      background_animation: newValue,
    });
  };

  const onAnimationSpeedUpdate = (newValue: number) => {
    setPreferences({
      ...NullPreferences,
      animation_speed: newValue,
    });
  };

  const onSoundToggle = (newValue: boolean) => {
    setPreferences({
      ...NullPreferences,
      sound_on: newValue,
    });
  };

  const onVolumeUpdate = (newValue: number) => {
    setPreferences({
      ...NullPreferences,
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
</style>
