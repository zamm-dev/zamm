import { ensureStorybookRunning, killStorybook } from "$lib/test-helpers";

export default async function setup() {
  const storybookProcess = await ensureStorybookRunning();

  return () => killStorybook(storybookProcess);
}
