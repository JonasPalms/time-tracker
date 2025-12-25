<script lang="ts">
  import "../app.css";
  import { useTheme } from "$lib/hooks/theme.svelte";
  import { useTracking } from "$lib/hooks/tracking.svelte";
  import { useFavourites } from "$lib/hooks/favourites.svelte";
  import { useKeyboard } from "$lib/hooks/keyboard.svelte";
  import { getDb } from "$lib/services/db";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";
  import AppHeader from "$lib/components/AppHeader.svelte";

  let { children } = $props();

  const theme = useTheme();
  const tracking = useTracking();
  const keyboard = useKeyboard();

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    await theme.init();
    await getDb();
    await useFavourites().reload();
    keyboard.init();

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
  <AppHeader />
  <main class="flex-1 overflow-hidden">
    {@render children()}
  </main>
</div>
