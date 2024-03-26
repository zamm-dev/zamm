import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Page,
  type Frame,
  type Locator,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";

const DEBUG_LOGGING = false;

describe("Slider drag test", () => {
  let page: Page;
  let frame: Frame;
  let browser: Browser;
  let context: BrowserContext;
  let numSoundsPlayed: number;

  beforeAll(async () => {
    browser = await chromium.launch({ headless: true });
    context = await browser.newContext();
    await context.exposeFunction(
      "_testRecordSoundPlayed",
      () => numSoundsPlayed++,
    );
    page = await context.newPage();

    if (DEBUG_LOGGING) {
      page.on("console", (msg) => {
        console.log(msg);
      });
    }
  });

  afterAll(async () => {
    await browser.close();
  });

  beforeEach(() => {
    numSoundsPlayed = 0;
  });

  const getSliderAndThumb = async (
    variant = "tiny-phone-screen-with-long-label",
  ) => {
    await page.goto(
      `http://localhost:6006/?path=/story/reusable-slider--${variant}`,
    );

    const maybeFrame = page.frame({ name: "storybook-preview-iframe" });
    if (!maybeFrame) {
      throw new Error("Could not find Storybook iframe");
    }
    frame = maybeFrame;
    const slider = frame.getByRole("slider");
    const thumb = slider.locator(".toggle");
    const sliderBounds = await slider.boundingBox();
    if (!sliderBounds) {
      throw new Error("Could not get slider bounding box");
    }

    return { slider, thumb, sliderBounds };
  };

  const expectAriaValue = async (slider: Locator, expectedValue: number) => {
    const sliderValueStr = await slider.evaluate((el) =>
      el.getAttribute("aria-valuenow"),
    );
    expect(sliderValueStr).not.toBeNull();
    if (!sliderValueStr) {
      // just for type-checking
      throw new Error("aria-valuenow attribute was null");
    }
    const sliderValue = parseFloat(sliderValueStr);
    expect(sliderValue).toEqual(expectedValue);
  };

  test(
    "goes to maximum even when thumb released past end",
    async () => {
      const { slider, thumb, sliderBounds } = await getSliderAndThumb();
      await expectAriaValue(slider, 5);

      await thumb.dragTo(slider, {
        targetPosition: { x: sliderBounds.width, y: sliderBounds.height / 2 },
      });
      await expectAriaValue(slider, 10);
    },
    { retry: 2 },
  );

  test(
    "goes to minimum even when thumb released past end",
    async () => {
      const { slider, thumb, sliderBounds } = await getSliderAndThumb();
      await expectAriaValue(slider, 5);

      await thumb.dragTo(slider, {
        targetPosition: { x: 0, y: sliderBounds.height / 2 },
      });
      await expectAriaValue(slider, 0);
    },
    { retry: 2 },
  );

  test(
    "goes to intermediate value when thumb released in-between",
    async () => {
      const { slider, thumb, sliderBounds } = await getSliderAndThumb();
      await expectAriaValue(slider, 5);

      await thumb.dragTo(slider, {
        targetPosition: {
          x: sliderBounds.width * 0.75,
          y: sliderBounds.height / 2,
        },
      });
      const valueString = (await slider.getAttribute(
        "aria-valuenow",
      )) as string;
      const value = parseFloat(valueString);
      expect(value).toBeGreaterThan(5);
      expect(value).toBeLessThan(10);
    },
    { retry: 2 },
  );

  test(
    "allows for arrow key use",
    async () => {
      const { slider } = await getSliderAndThumb();
      await expectAriaValue(slider, 5);

      await slider.press("ArrowRight");
      await expectAriaValue(slider, 6);
    },
    { retry: 2 },
  );

  test(
    "allows for mouse click",
    async () => {
      const { slider, sliderBounds } = await getSliderAndThumb();
      await expectAriaValue(slider, 5);

      await page.mouse.click(
        sliderBounds.x + sliderBounds.width * 0.25,
        sliderBounds.y + sliderBounds.height / 2,
      );
      const valueString = (await slider.getAttribute(
        "aria-valuenow",
      )) as string;
      const value = parseFloat(valueString);
      expect(value).toBeLessThan(5);
      expect(value).toBeGreaterThan(0);
    },
    { retry: 2 },
  );
});
