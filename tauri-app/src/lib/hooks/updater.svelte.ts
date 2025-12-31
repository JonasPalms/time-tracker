import { check } from "@tauri-apps/plugin-updater";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Update } from "@tauri-apps/plugin-updater";

let currentUpdate = $state<Update | null>(null);

async function checkForUpdates() {
  // Only check in production builds
  if (import.meta.env.DEV) {
    return;
  }

  try {
    const update = await check();

    if (update) {
      currentUpdate = update;

      // Show native dialog
      const message = `Update available: v${update.version}\n\n${update.body || "A new version is available."}\n\nWould you like to update now?`;
      const shouldUpdate = confirm(message);

      if (shouldUpdate) {
        await installUpdate();
      }
    }
  } catch (error) {
    console.error("Failed to check for updates:", error);
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
    console.error("Failed to install update:", error);
    alert(`Failed to install update: ${error}`);
  }
}

export function useUpdater() {
  return {
    checkForUpdates,
    installUpdate,
  };
}
