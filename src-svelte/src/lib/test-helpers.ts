import { spawn, ChildProcess } from "child_process";
import fetch from "node-fetch";
import { type Page } from "@playwright/test";
import { tick } from "svelte";

export const PLAYWRIGHT_TIMEOUT =
  process.env.PLAYWRIGHT_TIMEOUT === undefined
    ? 9_000
    : parseInt(process.env.PLAYWRIGHT_TIMEOUT);
export const PLAYWRIGHT_TEST_TIMEOUT = 2.2 * PLAYWRIGHT_TIMEOUT;

async function startStorybook(): Promise<ChildProcess> {
  return new Promise((resolve, reject) => {
    const storybookProcess = spawn("yarn", ["storybook", "--ci"]);
    if (!storybookProcess) {
      throw new Error("Could not start storybook process");
    } else if (!storybookProcess.stdout || !storybookProcess.stderr) {
      throw new Error("Could not get storybook output");
    }

    const storybookStartupMessage =
      /Storybook \d+\.\d+\.\d+ for sveltekit started/;

    storybookProcess.stdout.on("data", (data) => {
      const strippedData = data.toString().replace(/\\x1B\[\d+m/g, "");
      if (storybookStartupMessage.test(strippedData)) {
        fetch("http://localhost:6006")
          .then(() => {
            resolve(storybookProcess);
          })
          .catch(reject);
      }
    });

    storybookProcess.stderr.on("data", (data) => {
      console.error(`Storybook error: ${data}`);
    });
  });
}

async function checkIfStorybookIsRunning(): Promise<boolean> {
  try {
    await fetch("http://localhost:6006");
    return true;
  } catch {
    return false;
  }
}

export async function ensureStorybookRunning(): Promise<
  ChildProcess | undefined
> {
  if (!(await checkIfStorybookIsRunning())) {
    return await startStorybook();
  }
}

export async function killStorybook(process?: ChildProcess) {
  if (!process) {
    return;
  }

  process.kill();
}

export async function getStorybookFrame(page: Page, url: string) {
  await page.goto(url);
  const hideAddonsButton = page.locator("button[title^='Hide addons ']");
  // first click minimizes it, second click restores it, third click gets rid of it
  // for good. We no longer have to do the first click because manager.ts minimizes
  // it by default already
  for (let i = 0; i < 2; i++) {
    await hideAddonsButton.dispatchEvent("click");
    await page.waitForTimeout(100);
  }

  await page.waitForSelector("iframe");
  const maybeFrame = page.frame({ name: "storybook-preview-iframe" });
  if (!maybeFrame) {
    throw new Error("Could not find Storybook iframe");
  }
  return maybeFrame;
}

export async function tickFor(ticks: number) {
  for (let i = 0; i < ticks; i++) {
    await tick();
  }
}
