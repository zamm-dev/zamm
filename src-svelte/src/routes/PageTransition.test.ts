import { getTransitionTiming } from "./PageTransition.svelte";
import PageTransitionControl from "./PageTransitionControl.svelte";
import { act, render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { get } from "svelte/store";
import { firstPageLoad } from "$lib/firstPageLoad";

describe("PageTransition durations", () => {
  it("should halve the duration if no overlap", () => {
    const totalTime = 100;
    const overlapFraction = 0;
    const expectedDuration = 50;
    const expectedDelay = 50;
    // check that our test is doing the math right
    // both durations will have the same length, so the total time is the time delay
    // before the second one starts plus the length of the second one
    expect(expectedDuration + expectedDelay).toEqual(totalTime);
    // check that our function is doing the math right
    expect(getTransitionTiming(totalTime, overlapFraction)).toEqual({
      duration: expectedDuration,
      delay: expectedDelay,
    });
  });

  it("should increase delay if negative overlap", () => {
    const totalTime = 220;
    const overlapFraction = -0.2;
    const expectedDuration = 100;
    const expectedDelay = 120;
    expect(expectedDuration + expectedDelay).toEqual(totalTime);
    expect(getTransitionTiming(totalTime, overlapFraction)).toEqual({
      duration: expectedDuration,
      delay: expectedDelay,
    });
  });

  it("should increase duration if positive overlap", () => {
    const totalTime = 180;
    const overlapFraction = 0.2;
    const expectedDuration = 100;
    const expectedDelay = 80;
    expect(expectedDuration + expectedDelay).toEqual(totalTime);
    expect(getTransitionTiming(totalTime, overlapFraction)).toEqual({
      duration: expectedDuration,
      delay: expectedDelay,
    });
  });

  it("should have zero delay at total overlap", () => {
    const totalTime = 100;
    const overlapFraction = 1.0;
    const expectedDuration = 100;
    const expectedDelay = 0;
    expect(expectedDuration + expectedDelay).toEqual(totalTime);
    expect(getTransitionTiming(totalTime, overlapFraction)).toEqual({
      duration: expectedDuration,
      delay: expectedDelay,
    });
  });
});

describe("PageTransition", () => {
  let routeInput: HTMLElement;
  let navigateButton: HTMLElement;

  const navigateTo = async (url: string) => {
    await act(() => userEvent.type(routeInput, url));
    await act(() => userEvent.click(navigateButton));
  };

  beforeEach(() => {
    render(PageTransitionControl, { currentRoute: "/" });
    routeInput = screen.getByLabelText("Route");
    navigateButton = screen.getByText("Navigate");
  });

  it("should set first page load on initial visit", async () => {
    expect(get(firstPageLoad)).toEqual(true);
  });

  it("should set first page load on visit to new page", async () => {
    await navigateTo("/settings");
    expect(get(firstPageLoad)).toEqual(true);
  });

  it("should unset first page load on visit to old page", async () => {
    await navigateTo("/settings");
    await navigateTo("/");
    expect(get(firstPageLoad)).toEqual(false);
  });
});
