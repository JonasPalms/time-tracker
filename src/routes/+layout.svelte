<script lang="ts">
  import "../app.css"
  import { createThemeContext } from "$lib/theme.svelte"
  import { createTrackingContext } from "$lib/tracking.svelte"
  import { createTimeAdjustContext } from "$lib/timeAdjust.svelte"
  import { getDb } from "$lib/db"
  import { getCurrentWindow } from "@tauri-apps/api/window"
  import { onMount, onDestroy } from "svelte"

  let { children } = $props()

  // Create and provide theme context to all children
  const { init: initTheme } = createThemeContext()

  // Create and provide tracking context to all children
  const { cleanup: cleanupTracking, stopTracking } = createTrackingContext()

  // Create and provide time adjust context to all children
  const { init: initTimeAdjust } = createTimeAdjustContext()

  let unlisten: (() => void) | null = null

  onMount(async () => {
    // Initialize theme
    await initTheme()

    // Initialize time adjust settings
    await initTimeAdjust()

    // Initialize database
    await getDb()

    // Handle window close - stop tracking before app quits
    const currentWindow = getCurrentWindow()
    unlisten = await currentWindow.onCloseRequested(async () => {
      await stopTracking()
    })
  })

  onDestroy(() => {
    cleanupTracking()
    unlisten?.()
  })
</script>

<div class="min-h-screen bg-surface text-on-surface">
  <div class="h-10 w-full" data-tauri-drag-region></div>
  <main class="container mx-auto px-6 pb-6 max-w-2xl">
    {@render children()}
  </main>
</div>
