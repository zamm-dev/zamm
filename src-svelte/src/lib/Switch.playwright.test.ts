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

describe("Switch drag test", () => {
  let page: Page;
  let frame: Frame;
  let browser: Browser;
  let context: BrowserContext;
  let numSoundsPlayed: number;
  let soundDelays: number[];

  beforeAll(async () => {
    browser = await chromium.launch({ headless: true });
    context = await browser.newContext();
    await context.exposeFunction(
      "_testRecordSoundPlayed",
      () => numSoundsPlayed++,
    );
    await context.exposeFunction("_testRecordSoundDelay", (delay: number) =>
      soundDelays.push(delay),
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
    soundDelays = [];
  });

  const getSwitchAndToggle = async (initialState = "off") => {
    const url = `http://localhost:6006/?path=/story/reusable-switch--${initialState}`;
    await page.goto(url);

    const maybeFrame = page.frame({ name: "storybook-preview-iframe" });
    if (!maybeFrame) {
      throw new Error("Could not find Storybook iframe");
    }
    frame = maybeFrame;
    const onOffSwitch = frame.getByRole("switch");
    const toggle = onOffSwitch.locator(".toggle");
    const switchBounds = await onOffSwitch.boundingBox();
    if (!switchBounds) {
      throw new Error("Could not get switch bounding box");
    }

    return { onOffSwitch, toggle, switchBounds };
  };

  const expectAriaValue = async (
    switchElement: Locator,
    expectedValue: boolean,
  ) => {
    const switchValueStr = await switchElement.evaluate((el) =>
      el.getAttribute("aria-checked"),
    );
    expect(switchValueStr).not.toBeNull();
    expect(switchValueStr).toEqual(expectedValue.toString());
  };

  test(
    "switches state when drag released at end",
    async () => {
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle();
      await expectAriaValue(onOffSwitch, false);
      expect(numSoundsPlayed).toEqual(0);

      await toggle.dragTo(onOffSwitch, {
        targetPosition: { x: switchBounds.width, y: switchBounds.height / 2 },
      });
      await expectAriaValue(onOffSwitch, true);
      expect(numSoundsPlayed).toEqual(1);
    },
    { retry: 2 },
  );

  test(
    "switches state when drag released more than halfway to end",
    async () => {
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle();
      await expectAriaValue(onOffSwitch, false);
      expect(numSoundsPlayed).toEqual(0);

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.55,
          y: switchBounds.height / 2,
        },
      });
      await expectAriaValue(onOffSwitch, true);
      expect(numSoundsPlayed).toEqual(1);
      const expectedDelays = [0];
      expect(
        JSON.stringify(soundDelays) === JSON.stringify(expectedDelays),
      ).toBeTruthy();
    },
    { retry: 2 },
  );

  test(
    "delays click sound when animation speed slow",
    async () => {
      const { onOffSwitch, toggle, switchBounds } =
        await getSwitchAndToggle("slow-motion");
      // ===== same as previous test =====
      await expectAriaValue(onOffSwitch, false);
      expect(numSoundsPlayed).toEqual(0);

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.55,
          y: switchBounds.height / 2,
        },
      });
      // sound is delayed, wait for it to fire
      await new Promise((r) => setTimeout(r, 500));
      await expectAriaValue(onOffSwitch, true);
      expect(numSoundsPlayed).toEqual(1);
      // ===== end similarity block =====
      const expectedDelays = [450];
      expect(
        JSON.stringify(soundDelays) === JSON.stringify(expectedDelays),
      ).toBeTruthy();
    },
    { retry: 2 },
  );

  test(
    "maintains state when drag released less than halfway to end",
    async () => {
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle();
      await expectAriaValue(onOffSwitch, false);
      expect(numSoundsPlayed).toEqual(0);

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.25,
          y: switchBounds.height / 2,
        },
      });
      await expectAriaValue(onOffSwitch, false);
      expect(numSoundsPlayed).toEqual(0);
    },
    { retry: 2 },
  );

  test(
    "clicks twice when dragged to end and back",
    async () => {
      const { onOffSwitch, toggle, switchBounds } =
        await getSwitchAndToggle("slow-motion");
      const finalY = switchBounds.y + switchBounds.height / 2;
      await expectAriaValue(onOffSwitch, false);
      expect(numSoundsPlayed).toEqual(0);

      await toggle.hover();
      await page.mouse.down();

      // move to the very end
      await page.mouse.move(switchBounds.x + switchBounds.width, finalY);
      expect(numSoundsPlayed).toEqual(1);

      // move back to the beginning
      await page.mouse.move(switchBounds.x, finalY);
      await expectAriaValue(onOffSwitch, false);
      expect(numSoundsPlayed).toEqual(2);

      // does not play sound when released
      await page.mouse.up();
      expect(numSoundsPlayed).toEqual(2);

      // should have no delay for both sounds played despite slower animation
      const expectedDelays = [0, 0];
      expect(
        JSON.stringify(soundDelays) === JSON.stringify(expectedDelays),
      ).toBeTruthy();
    },
    { retry: 2 },
  );
});
