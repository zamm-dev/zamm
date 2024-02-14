import { spawn, ChildProcess } from "child_process";
import fetch from "node-fetch";
import { tick } from "svelte";

async function startStorybook(): Promise<ChildProcess> {
  return new Promise((resolve) => {
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
        resolve(storybookProcess);
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

export async function tickFor(ticks: number) {
  for (let i = 0; i < ticks; i++) {
    await tick();
  }
}
