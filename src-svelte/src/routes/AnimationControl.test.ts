import { expect, test } from "vitest";
import "@testing-library/jest-dom";
import { render } from "@testing-library/svelte";
import AnimationControl from "./AnimationControl.svelte";
import { animationsOn, animationSpeed } from "$lib/preferences";

describe("AnimationControl", () => {
  beforeEach(() => {
    animationsOn.set(true);
    animationSpeed.set(1);
  });

  test("will enable animations by default", async () => {
    render(AnimationControl, {});

    const animationControl = document.querySelector(".container") as Element;
    expect(animationControl.classList).not.toContainEqual(
      "animations-disabled",
    );
    expect(animationControl.getAttribute("style")).toEqual(
      "--base-animation-speed: 1; --standard-duration: 100.00ms;",
    );
  });

  test("will disable animations if preference overridden", async () => {
    animationsOn.set(false);
    render(AnimationControl, {});

    const animationControl = document.querySelector(".container") as Element;
    expect(animationControl.classList).toContainEqual("animations-disabled");
    expect(animationControl.getAttribute("style")).toEqual(
      "--base-animation-speed: 1; --standard-duration: 0.00ms;",
    );
  });

  test("will slow down animations if preference overridden", async () => {
    animationSpeed.set(0.9);
    render(AnimationControl, {});

    const animationControl = document.querySelector(".container") as Element;
    expect(animationControl.classList).not.toContainEqual(
      "animations-disabled",
    );
    expect(animationControl.getAttribute("style")).toEqual(
      "--base-animation-speed: 0.9; --standard-duration: 111.11ms;",
    );
  });
});
