<script lang="ts">
  import "../app.css";
  import { createThemeContext } from "$lib/theme.svelte";
  import { createTrackingContext } from "$lib/tracking.svelte";
  import { getDb } from "$lib/db";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";
  import AppHeader from "$lib/components/AppHeader.svelte";

  let { children } = $props();

  const { init: initTheme } = createThemeContext();
  const { cleanup: cleanupTracking, stopTracking } = createTrackingContext();

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    await initTheme();
    await getDb();

    const currentWindow = getCurrentWindow();
    unlisten = await currentWindow.onCloseRequested(async () => {
      await stopTracking();
    });
  });

  onDestroy(() => {
    cleanupTracking();
    unlisten?.();
  });
</script>

<div class="h-screen flex flex-col bg-surface text-on-surface overflow-hidden rounded-2xl">
  <AppHeader />
  <main class="flex-1 overflow-hidden">
    {@render children()}
  </main>
</div>
