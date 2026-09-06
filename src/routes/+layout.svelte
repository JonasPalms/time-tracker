<script lang="ts">
  import "../app.css";
  import { useTheme } from "$lib/hooks/theme.svelte";
  import {
    startTasksRefresh,
    stopTasksRefresh,
    tasksRefreshGeneration,
  } from "$lib/hooks/tasks-refresh.svelte";
  import { useTracking } from "$lib/hooks/tracking.svelte";
  import { useFavourites } from "$lib/hooks/favourites.svelte";
  import { useKeyboard } from "$lib/hooks/keyboard.svelte";
  import { useUpdater } from "$lib/hooks/updater.svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import WindowControls from "$lib/components/WindowControls.svelte";
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import UpdateDialog from "$lib/components/UpdateDialog.svelte";

  let { children } = $props();

  useTheme();
  const tracking = useTracking();
  const keyboard = useKeyboard();
  const updater = useUpdater();

  let unlistenCheckUpdates: (() => void) | null = null;

  onMount(async () => {
    const currentWindow = getCurrentWindow();

    try {
      keyboard.init();
      updater.checkForUpdates();
      void useFavourites().reload();
      void startTasksRefresh();
    } finally {
      await currentWindow.show();
    }

    unlistenCheckUpdates = await listen("check-for-updates", () => {
      updater.checkForUpdates(true);
    });
  });

  $effect(() => {
    tasksRefreshGeneration();
    void tracking.refresh();
  });

  onDestroy(() => {
    stopTasksRefresh();
    tracking.cleanup();
    keyboard.cleanup();
    unlistenCheckUpdates?.();
  });
</script>

<div
  id="app-shell"
  class="relative isolate h-screen flex flex-col bg-surface text-on-surface overflow-hidden rounded-2xl"
>
  <!-- Title bar with window controls -->
  <header
    class="shrink-0 h-10 flex items-center px-2 border-b border-border"
    data-tauri-drag-region
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div onmousedown={(e) => e.stopPropagation()}>
      <WindowControls />
    </div>
  </header>

  <!-- Main content area with sidebar -->
  <div class="flex-1 flex overflow-hidden">
    <AppSidebar />
    <main class="flex-1 overflow-hidden">
      {@render children()}
    </main>
  </div>
</div>

<UpdateDialog
  bind:open={updater.showDialog}
  update={updater.update}
  releaseNotes={updater.releaseNotes}
  onInstall={() => updater.installUpdate()}
  onCancel={() => updater.dismissUpdate()}
/>
