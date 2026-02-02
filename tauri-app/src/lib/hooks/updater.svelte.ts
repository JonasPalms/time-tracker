import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { message } from "@tauri-apps/plugin-dialog";
import type { Update } from "@tauri-apps/plugin-updater";

let currentUpdate = $state<Update | null>(null);
let showUpdateDialog = $state(false);

async function checkForUpdates(manual: boolean = false) {
  // Only check in production builds
  if (import.meta.env.DEV) {
    if (manual) {
      await message("Update checking is disabled in development mode.", {
        title: "Check for Updates",
        kind: "info",
      });
    }
    return;
  }

  try {
    const update = await check();

    if (update) {
      currentUpdate = update;
      showUpdateDialog = true;
    } else if (manual) {
      await message("You're running the latest version.", {
        title: "Check for Updates",
        kind: "info",
      });
    }
  } catch (error) {
    console.error("[Updater] Failed to check for updates:", error);
    if (manual) {
      await message("Failed to check for updates. Please try again later.", {
        title: "Check for Updates",
        kind: "error",
      });
    }
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
