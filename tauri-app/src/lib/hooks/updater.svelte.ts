import { check } from "@tauri-apps/plugin-updater";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Update } from "@tauri-apps/plugin-updater";

let currentUpdate = $state<Update | null>(null);
let showUpdateDialog = $state(false);

async function checkForUpdates() {
  // Only check in production builds
  if (import.meta.env.DEV) {
    return;
  }

  try {
    const update = await check();

    if (update) {
      currentUpdate = update;
      showUpdateDialog = true;
    }
  } catch (error) {
    console.error("[Updater] Failed to check for updates:", error);
  }
}

async function installUpdate() {
  if (!currentUpdate) {
    return;
  }

  try {
    await currentUpdate.downloadAndInstall();
    // After installation, restart the app
    const currentWindow = getCurrentWindow();
    await currentWindow.close();
  } catch (error) {
    console.error("[Updater] Failed to install update:", error);
  }
}

function dismissUpdate() {
  showUpdateDialog = false;
  currentUpdate = null;
}

export function useUpdater() {
  return {
    get update() {
      return currentUpdate;
    },
    get showDialog() {
      return showUpdateDialog;
    },
    set showDialog(value: boolean) {
      showUpdateDialog = value;
    },
    checkForUpdates,
    installUpdate,
    dismissUpdate,
  };
}
