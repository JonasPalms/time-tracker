import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { message } from "@tauri-apps/plugin-dialog";
import type { Update } from "@tauri-apps/plugin-updater";
import { getChangelogSections } from "$lib/services/changelog";
import type { ChangelogSection } from "$lib/services/changelog";

let currentUpdate = $state<Update | null>(null);
let showUpdateDialog = $state(false);
let currentReleaseNotes = $state<ChangelogSection[]>([]);

type UpdaterState = {
  readonly update: Update | null;
  showDialog: boolean;
  readonly releaseNotes: ChangelogSection[];
  checkForUpdates: (manual?: boolean) => Promise<void>;
  installUpdate: () => Promise<void>;
  dismissUpdate: () => void;
};

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
      currentReleaseNotes = getChangelogSections(update.version);
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
  currentReleaseNotes = [];
}

export function useUpdater(): UpdaterState {
  return {
    get update() {
      return currentUpdate;
    },
    get showDialog() {
      return showUpdateDialog;
    },
    get releaseNotes() {
      return currentReleaseNotes;
    },
    set showDialog(value: boolean) {
      showUpdateDialog = value;
    },
    checkForUpdates,
    installUpdate,
    dismissUpdate,
  };
}
