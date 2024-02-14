import type { StoryFn, Decorator, StoryContext } from "@storybook/svelte";
import { TauriInvokePlayback } from "$lib/sample-call-testing";

let playback: TauriInvokePlayback;
let nextShouldWait = false;

function mockInvokeFn<T>(
  command: string,
  args?: Record<string, string>,
): Promise<T> {
  if (nextShouldWait) {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(null as T);
      }, 1_000_000); // the re-render never happens, so any timeout is fine
    });
  } else {
    const allArgs = args === undefined ? [command] : [command, args];
    return playback.mockCall(...allArgs) as Promise<T>;
  }
}

window.__TAURI_INVOKE__ = mockInvokeFn;

interface TauriInvokeArgs {
  sampleCallFiles?: string[];
  shouldWait?: boolean | undefined;
  [key: string]: any;
}

const TauriInvokeDecorator: Decorator = (
  story: StoryFn,
  context: StoryContext,
) => {
  const { args, parameters } = context;
  const { sampleCallFiles, shouldWait } = parameters as TauriInvokeArgs;
  playback = new TauriInvokePlayback();
  if (sampleCallFiles !== undefined) {
    playback.addSamples(...sampleCallFiles);
  }
  nextShouldWait = shouldWait || false;
  return story(args, context);
};

export default TauriInvokeDecorator;
