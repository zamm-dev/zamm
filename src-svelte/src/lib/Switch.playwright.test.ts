import {
  type Browser,
  type BrowserContext,
  chromium,
  expect,
  type Page,
  type Frame,
} from "@playwright/test";
import { afterAll, beforeAll, describe, test } from "vitest";
import type { ChildProcess } from "child_process";
import { ensureStorybookRunning, killStorybook } from "$lib/test-helpers";

const DEBUG_LOGGING = false;

describe("Switch drag test", () => {
  let storybookProcess: ChildProcess | undefined;
  let page: Page;
  let frame: Frame;
  let browser: Browser;
  let context: BrowserContext;
  let numSoundsPlayed: number;
  let soundDelays: number[];

  beforeAll(async () => {
    storybookProcess = await ensureStorybookRunning();

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
    await killStorybook(storybookProcess);
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

  test(
    "switches state when drag released at end",
    async () => {
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle();
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
      expect(numSoundsPlayed === 0).toBeTruthy();

      await toggle.dragTo(onOffSwitch, {
        targetPosition: { x: switchBounds.width, y: switchBounds.height / 2 },
      });
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
      expect(numSoundsPlayed === 1).toBeTruthy();
    },
    { retry: 2 },
  );

  test(
    "switches state when drag released more than halfway to end",
    async () => {
      const { onOffSwitch, toggle, switchBounds } = await getSwitchAndToggle();
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
      expect(numSoundsPlayed === 0).toBeTruthy();

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.55,
          y: switchBounds.height / 2,
        },
      });
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
      expect(numSoundsPlayed === 1).toBeTruthy();
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
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
      expect(numSoundsPlayed === 0).toBeTruthy();

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.55,
          y: switchBounds.height / 2,
        },
      });
      // sound is delayed, wait for it to fire
      await new Promise((r) => setTimeout(r, 500));
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "true");
      expect(numSoundsPlayed === 1).toBeTruthy();
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
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
      expect(numSoundsPlayed === 0).toBeTruthy();

      await toggle.dragTo(onOffSwitch, {
        targetPosition: {
          x: switchBounds.width * 0.25,
          y: switchBounds.height / 2,
        },
      });
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
      expect(numSoundsPlayed === 0).toBeTruthy();
    },
    { retry: 2 },
  );

  test(
    "clicks twice when dragged to end and back",
    async () => {
      const { onOffSwitch, toggle, switchBounds } =
        await getSwitchAndToggle("slow-motion");
      const finalY = switchBounds.y + switchBounds.height / 2;
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
      expect(numSoundsPlayed === 0).toBeTruthy();

      await toggle.hover();
      await page.mouse.down();

      // move to the very end
      await page.mouse.move(switchBounds.x + switchBounds.width, finalY);
      expect(numSoundsPlayed === 1).toBeTruthy();

      // move back to the beginning
      await page.mouse.move(switchBounds.x, finalY);
      await expect(onOffSwitch).toHaveAttribute("aria-checked", "false");
      expect(numSoundsPlayed === 2).toBeTruthy();

      // does not play sound when released
      await page.mouse.up();
      expect(numSoundsPlayed === 2).toBeTruthy();

      // should have no delay for both sounds played despite slower animation
      const expectedDelays = [0, 0];
      expect(
        JSON.stringify(soundDelays) === JSON.stringify(expectedDelays),
      ).toBeTruthy();
    },
    { retry: 2 },
  );
});
