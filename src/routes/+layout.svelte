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
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import UpdateDialog from "$lib/components/UpdateDialog.svelte";
  import WindowControls from "$lib/components/WindowControls.svelte";
  import { useSidebar } from "$lib/hooks/sidebar.svelte";

  let { children } = $props();

  useTheme();
  const sidebar = useSidebar();
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
  class="relative isolate h-screen flex bg-stone-200 text-on-surface overflow-hidden rounded-2xl dark:bg-black"
>
  <div
    class="absolute inset-x-0 top-0 z-30 flex items-center pt-5 pl-5 pr-2"
    data-tauri-drag-region
  >
    <WindowControls />
  </div>

  <AppSidebar />
  <main
    class="relative flex-1 min-w-0 my-2.5 mr-2.5 overflow-hidden rounded-2xl bg-surface pt-5
      {sidebar.hidden ? 'ml-2' : ''}
      shadow-[0_0_0_1px_rgba(0,0,0,0.04),0_2px_10px_rgba(0,0,0,0.06)]
      dark:shadow-[0_0_0_1px_rgba(255,255,255,0.06),0_8px_24px_rgba(0,0,0,0.45)]"
  >
    <div class="absolute inset-x-0 top-0 z-10 h-3" data-tauri-drag-region></div>
    {@render children()}
  </main>
</div>

<UpdateDialog
  bind:open={updater.showDialog}
  update={updater.update}
  releaseNotes={updater.releaseNotes}
  onInstall={() => updater.installUpdate()}
  onCancel={() => updater.dismissUpdate()}
/>
