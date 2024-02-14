import { getAnimationTiming } from "./InfoBox.svelte";
import { PrimitiveTimingMs, TimingGroupAsCollection } from "./animation-timing";

describe("InfoBox animation timing", () => {
  it("should be the default if no additional scaling or delay", () => {
    const preDelay = 0;
    const timingScaleFactor = 1;
    const timing = getAnimationTiming(preDelay, timingScaleFactor);

    // regular border box animation values
    const borderBoxMs = new TimingGroupAsCollection([
      // grow x
      new PrimitiveTimingMs({ startMs: 0, endMs: 200 }),
      // grow y
      new PrimitiveTimingMs({ startMs: 180, endMs: 330 }),
    ]);
    expect(timing.borderBox.asCollection()).toEqual(borderBoxMs);

    // regular title animation values
    const titleCollectionMs = new TimingGroupAsCollection([
      // typewriter
      new PrimitiveTimingMs({ startMs: 20, endMs: 200 }),
      // cursor fade
      new PrimitiveTimingMs({ startMs: 240, endMs: 330 }),
    ]);
    expect(timing.title.asCollection()).toEqual(titleCollectionMs);

    // regular info box animation values
    expect(timing.infoBox).toEqual(
      new PrimitiveTimingMs({
        delayMs: 180,
        durationMs: 260,
      }),
    );
  });

  it("should not let delays affect fractions or durations", () => {
    const delay = 100;
    const scaleFactor = 1;
    const timing = getAnimationTiming(0, scaleFactor);
    const scaledTiming = getAnimationTiming(delay, scaleFactor);

    // regular border box animation values
    expect(timing.borderBox.growX()).toEqual(scaledTiming.borderBox.growX());
    expect(timing.borderBox.growY()).toEqual(scaledTiming.borderBox.growY());
    expect(timing.borderBox.overall.delayMs() + delay).toEqual(
      scaledTiming.borderBox.overall.delayMs(),
    );
    expect(timing.borderBox.overall.durationMs()).toEqual(
      scaledTiming.borderBox.overall.durationMs(),
    );

    // regular title animation values
    expect(timing.title.typewriter()).toEqual(scaledTiming.title.typewriter());
    expect(timing.title.cursorFade()).toEqual(scaledTiming.title.cursorFade());
    expect(timing.title.overall.delayMs() + delay).toEqual(
      scaledTiming.title.overall.delayMs(),
    );
    expect(timing.title.overall.durationMs()).toEqual(
      scaledTiming.title.overall.durationMs(),
    );

    // regular info box animation values
    expect(timing.infoBox.delayMs() + delay).toEqual(
      scaledTiming.infoBox.delayMs(),
    );
    expect(timing.infoBox.durationMs()).toEqual(
      scaledTiming.infoBox.durationMs(),
    );
  });

  it("should not let scaling affect fractions", () => {
    const preDelay = 0;
    const scaleFactor = 10;
    const timing = getAnimationTiming(preDelay, 1);
    const scaledTiming = getAnimationTiming(preDelay, scaleFactor);

    // regular border box animation values
    expect(timing.borderBox.growX()).toEqual(scaledTiming.borderBox.growX());
    expect(timing.borderBox.growY()).toEqual(scaledTiming.borderBox.growY());
    expect(timing.borderBox.overall.startMs() * scaleFactor).toEqual(
      scaledTiming.borderBox.overall.startMs(),
    );
    expect(timing.borderBox.overall.endMs() * scaleFactor).toEqual(
      scaledTiming.borderBox.overall.endMs(),
    );

    // regular title animation values
    expect(timing.title.typewriter()).toEqual(scaledTiming.title.typewriter());
    expect(timing.title.cursorFade()).toEqual(scaledTiming.title.cursorFade());
    expect(timing.title.overall.startMs() * scaleFactor).toEqual(
      scaledTiming.title.overall.startMs(),
    );
    expect(timing.title.overall.endMs() * scaleFactor).toEqual(
      scaledTiming.title.overall.endMs(),
    );

    // regular info box animation values
    expect(timing.infoBox.startMs() * 10).toEqual(
      scaledTiming.infoBox.startMs(),
    );
    expect(timing.infoBox.endMs() * 10).toEqual(scaledTiming.infoBox.endMs());
  });
});
