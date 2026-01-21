<script lang="ts">
  import "../app.css";
  import { useTheme } from "$lib/hooks/theme.svelte";
  import { useTracking } from "$lib/hooks/tracking.svelte";
  import { useFavourites } from "$lib/hooks/favourites.svelte";
  import { useKeyboard } from "$lib/hooks/keyboard.svelte";
  import { useUpdater } from "$lib/hooks/updater.svelte";
  import { useSidebar } from "$lib/hooks/sidebar.svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";
  import WindowControls from "$lib/components/WindowControls.svelte";
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import UpdateDialog from "$lib/components/UpdateDialog.svelte";

  let { children } = $props();

  const theme = useTheme();
  const tracking = useTracking();
  const keyboard = useKeyboard();
  const updater = useUpdater();
  const sidebar = useSidebar();

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    await theme.init();
    await useFavourites().reload();
    keyboard.init();
    sidebar.init();

    updater.checkForUpdates();

    const currentWindow = getCurrentWindow();
    unlisten = await currentWindow.onCloseRequested(async () => {
      await tracking.stopTracking();
    });
  });

  onDestroy(() => {
    tracking.cleanup();
    keyboard.cleanup();
    unlisten?.();
  });
</script>

<div class="h-screen flex flex-col bg-surface text-on-surface overflow-hidden rounded-2xl">
  <!-- Title bar with window controls -->
  <header class="shrink-0 h-10 flex items-center px-2 border-b border-border" data-tauri-drag-region>
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
  onInstall={() => updater.installUpdate()}
  onCancel={() => updater.dismissUpdate()}
/>
