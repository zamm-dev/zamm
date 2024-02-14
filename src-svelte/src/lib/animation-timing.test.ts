import {
  PrimitiveTimingMs,
  PrimitiveTimingFraction,
  TimingGroupAsCollection,
  TimingGroupAsIndividual,
  inverseCubicInOut,
} from "./animation-timing";
import { cubicInOut } from "svelte/easing";

describe("Animation timing", () => {
  it("should invert cubic in-out correctly", () => {
    expect(inverseCubicInOut(0)).toEqual(0);
    expect(inverseCubicInOut(0.5)).toEqual(0.5);
    expect(inverseCubicInOut(1)).toEqual(1);
    expect(inverseCubicInOut(cubicInOut(0.25))).toEqual(0.25);
    expect(inverseCubicInOut(cubicInOut(0.75))).toEqual(0.75);
  });

  it("should enable ms timings to be defined in different ways", () => {
    const timingMs1 = new PrimitiveTimingMs({ startMs: 100, endMs: 300 });
    const timingMs2 = new PrimitiveTimingMs({ delayMs: 100, durationMs: 200 });
    expect(timingMs1).toEqual(timingMs2);
  });

  it("should enable fractional timings to be defined in different ways", () => {
    const timingMs1 = new PrimitiveTimingFraction({
      startFraction: 0.2,
      endFraction: 0.7,
    });
    const timingMs2 = new PrimitiveTimingFraction({
      delayFraction: 0.2,
      durationFraction: 0.5,
    });
    expect(timingMs1.round()).toEqual(timingMs2.round());
  });

  it("should nest and unnest timings correctly", () => {
    const timingMs = new PrimitiveTimingMs({ startMs: 200, endMs: 400 });
    const timingFraction = new PrimitiveTimingFraction({
      startFraction: 0.2,
      endFraction: 0.6,
    });
    const overall = new PrimitiveTimingMs({ startMs: 100, endMs: 600 });
    expect(timingMs.nestInside(overall).round()).toEqual(
      timingFraction.round(),
    );
    expect(timingFraction.unnestFrom(overall).round()).toEqual(timingMs);
  });

  it("should correctly combine groups of sub-animations into one", () => {
    const collectionMs = new TimingGroupAsCollection([
      new PrimitiveTimingMs({ startMs: 100, endMs: 400 }),
      new PrimitiveTimingMs({ startMs: 200, endMs: 500 }),
    ]);
    const collectionFraction = new TimingGroupAsIndividual({
      overall: new PrimitiveTimingMs({ startMs: 100, endMs: 500 }),
      children: [
        new PrimitiveTimingFraction({ startFraction: 0.0, endFraction: 0.75 }),
        new PrimitiveTimingFraction({ startFraction: 0.25, endFraction: 1.0 }),
      ],
    });
    expect(collectionMs.asIndividual()).toEqual(collectionFraction);
    expect(collectionFraction.asCollection()).toEqual(collectionMs);
  });
});
