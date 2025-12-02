<script lang="ts">
  import "../app.css";
  import "../shadcn.css";
  import { createThemeContext } from "$lib/theme.svelte";
  import { createTrackingContext } from "$lib/tracking.svelte";
  import { createTimeAdjustContext } from "$lib/timeAdjust.svelte";
  import { getDb } from "$lib/db";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";

  let { children } = $props();

  const { init: initTheme } = createThemeContext();
  const { cleanup: cleanupTracking, stopTracking } = createTrackingContext();
  const { init: initTimeAdjust } = createTimeAdjustContext();

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    await initTheme();

    await initTimeAdjust();
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

<div class="h-screen flex flex-col bg-surface text-on-surface overflow-hidden">
  <div class="h-10 w-full shrink-0" data-tauri-drag-region></div>
  <main class="flex-1 container mx-auto overflow-hidden">
    {@render children()}
  </main>
</div>
