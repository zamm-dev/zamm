// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

import "unplugin-icons/types/svelte";
import IconSettings from "~icons/ion/settings";

declare global {
  namespace App {
    interface Route {
      name: string;
      path: string;
      icon: typeof IconSettings;
    }
  }

  interface Window {
    __TAURI_IPC__?: () => void;
    // @ts-ignore
    [key: string]: any;
  }
}

export {};
