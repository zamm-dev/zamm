import { cubicInOut } from "svelte/easing";

export interface TransitionTimingMs {
  durationMs(): number;
  delayMs(): number;
  startMs(): number;
  endMs(): number;
  round(): TransitionTimingMs;
  delayByMs(delayMs: number): TransitionTimingMs;
  hastenByMs(hastenMs: number): TransitionTimingMs;
  scaleBy(factor: number): TransitionTimingMs;
  nestInside(container: TransitionTimingMs): TransitionTimingFraction;
}

export interface TransitionTimingFraction {
  durationFraction(): number;
  delayFraction(): number;
  startFraction(): number;
  endFraction(): number;
  round(): TransitionTimingFraction;
  unnestFrom(container: TransitionTimingMs): TransitionTimingMs;
  localize(globalTimeFraction: number): number;
}

export class PrimitiveTimingMs implements TransitionTimingMs {
  _durationMs: number;
  _delayMs: number;

  constructor({
    durationMs,
    delayMs,
    startMs,
    endMs,
  }: {
    durationMs?: number;
    delayMs?: number;
    startMs?: number;
    endMs?: number;
  }) {
    if ((delayMs === undefined) === (startMs === undefined)) {
      throw new Error("Exactly one of delayMs or startMs must be provided");
    }
    if ((durationMs === undefined) === (endMs === undefined)) {
      throw new Error("Exactly one of durationMs or endMs must be provided");
    }
    this._delayMs = (delayMs ?? startMs) as number;
    this._durationMs = durationMs ?? (endMs as number) - this._delayMs;
  }

  round(): PrimitiveTimingMs {
    return new PrimitiveTimingMs({
      durationMs: Math.round(this._durationMs),
      delayMs: Math.round(this._delayMs),
    });
  }

  durationMs(): number {
    return this._durationMs;
  }

  delayMs(): number {
    return this._delayMs;
  }

  startMs(): number {
    return this._delayMs;
  }

  endMs(): number {
    return this._delayMs + this._durationMs;
  }

  delayByMs(delayMs: number): PrimitiveTimingMs {
    return new PrimitiveTimingMs({
      durationMs: this._durationMs,
      delayMs: this._delayMs + delayMs,
    });
  }

  hastenByMs(hastenMs: number): PrimitiveTimingMs {
    return this.delayByMs(-hastenMs);
  }

  scaleBy(factor: number): PrimitiveTimingMs {
    return new PrimitiveTimingMs({
      durationMs: this._durationMs * factor,
      delayMs: this._delayMs * factor,
    });
  }

  toFraction(totalDurationMs: number): PrimitiveTimingFraction {
    if (totalDurationMs === 0) {
      return new PrimitiveTimingFraction({
        // if duration is total, then the fraction is meaningless
        // might as well set it to 1 to prevent further division by zero
        durationFraction: 1,
        delayFraction: 1,
      });
    }
    return new PrimitiveTimingFraction({
      durationFraction: this._durationMs / totalDurationMs,
      delayFraction: this._delayMs / totalDurationMs,
    });
  }

  nestInside(container: TransitionTimingMs): PrimitiveTimingFraction {
    return this.hastenByMs(container.delayMs()).toFraction(
      container.durationMs(),
    );
  }
}

export class PrimitiveTimingFraction implements TransitionTimingFraction {
  _durationFraction: number;
  _delayFraction: number;

  constructor({
    durationFraction,
    delayFraction,
    startFraction,
    endFraction,
  }: {
    durationFraction?: number;
    delayFraction?: number;
    startFraction?: number;
    endFraction?: number;
  }) {
    if ((delayFraction === undefined) === (startFraction === undefined)) {
      throw new Error(
        "Exactly one of delayFraction or startMs must be provided",
      );
    }
    if ((durationFraction === undefined) === (endFraction === undefined)) {
      throw new Error(
        "Exactly one of durationFraction or endMs must be provided",
      );
    }
    this._delayFraction = (delayFraction ?? startFraction) as number;
    this._durationFraction =
      durationFraction ?? (endFraction as number) - this._delayFraction;
  }

  round(): PrimitiveTimingFraction {
    const precision = 10_000;
    return new PrimitiveTimingFraction({
      durationFraction:
        Math.round(this._durationFraction * precision) / precision,
      delayFraction: Math.round(this._delayFraction * precision) / precision,
    });
  }

  delayFraction(): number {
    return this._delayFraction;
  }

  durationFraction(): number {
    return this._durationFraction;
  }

  startFraction(): number {
    return this._delayFraction;
  }

  endFraction(): number {
    return this._delayFraction + this._durationFraction;
  }

  toMs(totalDurationMs: number): PrimitiveTimingMs {
    return new PrimitiveTimingMs({
      durationMs: this._durationFraction * totalDurationMs,
      delayMs: this._delayFraction * totalDurationMs,
    });
  }

  unnestFrom(container: TransitionTimingMs): PrimitiveTimingMs {
    return this.toMs(container.durationMs()).delayByMs(container.delayMs());
  }

  localize(globalTimeFraction: number): number {
    if (globalTimeFraction < this.startFraction()) {
      return 0;
    } else if (globalTimeFraction > this.endFraction()) {
      return 1;
    }

    const localTimeFraction =
      (globalTimeFraction - this._delayFraction) / this._durationFraction;
    return localTimeFraction;
  }
}

export class TimingGroupAsCollection implements TransitionTimingMs {
  children: TransitionTimingMs[];

  constructor(children: TransitionTimingMs[]) {
    this.children = children;
  }

  startMs(): number {
    const startTimes = this.children.map((child) => child.startMs());
    return Math.min(...startTimes);
  }

  endMs(): number {
    const endTimes = this.children.map((child) => child.endMs());
    return Math.max(...endTimes);
  }

  durationMs(): number {
    return this.endMs() - this.startMs();
  }

  delayMs(): number {
    return this.startMs();
  }

  overallTiming(): PrimitiveTimingMs {
    return new PrimitiveTimingMs({
      durationMs: this.durationMs(),
      delayMs: this.delayMs(),
    });
  }

  round(): TimingGroupAsCollection {
    return new TimingGroupAsCollection(
      this.children.map((child) => child.round()),
    );
  }

  delayByMs(delayMs: number): TimingGroupAsCollection {
    return new TimingGroupAsCollection(
      this.children.map((child) => child.delayByMs(delayMs)),
    );
  }

  hastenByMs(hastenMs: number): TimingGroupAsCollection {
    return new TimingGroupAsCollection(
      this.children.map((child) => child.hastenByMs(hastenMs)),
    );
  }

  scaleBy(factor: number): TimingGroupAsCollection {
    return new TimingGroupAsCollection(
      this.children.map((child) => child.scaleBy(factor)),
    );
  }

  nestInside(_: TransitionTimingMs): TransitionTimingFraction {
    throw new Error("Recursive nesting not implemented");
  }

  asIndividual(): TimingGroupAsIndividual {
    const overall = this.overallTiming();
    const nestedChildren = this.children.map((child) =>
      child.nestInside(overall),
    );
    return new TimingGroupAsIndividual({
      overall,
      children: nestedChildren,
    });
  }
}

export class TimingGroupAsIndividual implements TransitionTimingMs {
  overall: TransitionTimingMs;
  children: TransitionTimingFraction[];

  constructor({
    overall,
    children,
  }: {
    overall: TransitionTimingMs;
    children: TransitionTimingFraction[];
  }) {
    this.overall = overall;
    this.children = children;
  }

  durationMs(): number {
    return this.overall.durationMs();
  }

  delayMs(): number {
    return this.overall.delayMs();
  }

  startMs(): number {
    return this.overall.startMs();
  }

  endMs(): number {
    return this.overall.endMs();
  }

  round(): TimingGroupAsIndividual {
    return new TimingGroupAsIndividual({
      overall: this.overall.round(),
      children: this.children.map((child) => child.round()),
    });
  }

  delayByMs(delayMs: number): TimingGroupAsIndividual {
    return new TimingGroupAsIndividual({
      overall: this.overall.delayByMs(delayMs),
      children: this.children,
    });
  }

  hastenByMs(hastenMs: number): TimingGroupAsIndividual {
    return new TimingGroupAsIndividual({
      overall: this.overall.hastenByMs(hastenMs),
      children: this.children,
    });
  }

  scaleBy(factor: number): TimingGroupAsIndividual {
    return new TimingGroupAsIndividual({
      overall: this.overall.scaleBy(factor),
      children: this.children,
    });
  }

  nestInside(_: TransitionTimingMs): TransitionTimingFraction {
    throw new Error("Recursive nesting not implemented");
  }

  asCollection(): TimingGroupAsCollection {
    const unnestedChildren = this.children.map((child) =>
      child.unnestFrom(this.overall),
    );
    return new TimingGroupAsCollection(unnestedChildren);
  }
}

export function inverseCubicInOut(t: number) {
  if (t < 0.5) {
    // Solve the cubic equation for t < 0.5
    return Math.cbrt(t / 4.0);
  } else {
    // Solve the cubic equation for t >= 0.5
    return (Math.cbrt(2.0 * (t - 1.0)) + 2.0) / 2.0;
  }
}

export class SubAnimation<T> {
  timing: TransitionTimingFraction;
  tick: (tLocalFraction: number) => T;

  constructor(anim: {
    timing: TransitionTimingFraction;
    tick: (tLocalFraction: number) => T;
  }) {
    this.timing = anim.timing;
    this.tick = anim.tick;
  }

  tickForGlobalTime(tGlobalFraction: number): T {
    return this.tick(this.timing.localize(tGlobalFraction));
  }
}

export class PropertyAnimation extends SubAnimation<string> {
  max: number;

  constructor(anim: {
    timing: TransitionTimingFraction;
    property: string;
    min: number;
    max: number;
    unit: string;
    easingFunction?: (t: number) => number;
  }) {
    const easingFunction = anim.easingFunction ?? cubicInOut;
    const css = (t: number) => {
      const easing = easingFunction(t);
      const growth = this.max - anim.min;
      const value = anim.min + growth * easing;
      return `${anim.property}: ${value}${anim.unit};`;
    };
    super({ timing: anim.timing, tick: css });

    this.max = anim.max;
  }
}
