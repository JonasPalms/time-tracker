import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
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
    // Restart the app with the new version
    await relaunch();
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
