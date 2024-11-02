import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { ask } from "@tauri-apps/plugin-dialog";

export async function checkForUpdates() {
  const update = await check();
  if (update?.available) {
    const yes = await ask(
      `ZAMM v${update.version} is available!\n\nRelease notes: ${update.body}`,
      {
        title: "Update Available",
        kind: "info",
        okLabel: "Update",
        cancelLabel: "Cancel",
      },
    );
    if (yes) {
      await update.downloadAndInstall();
      await relaunch();
    }
  }
}
