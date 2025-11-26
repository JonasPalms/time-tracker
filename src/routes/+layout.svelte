<script lang="ts">
  import "../app.css"
  import { createThemeContext } from "$lib/theme.svelte"
  import { createTrackingContext } from "$lib/tracking.svelte"
  import { getDb } from "$lib/db"
  import { getCurrentWindow } from "@tauri-apps/api/window"
  import { onMount, onDestroy } from "svelte"

  let { children } = $props()

  // Create and provide theme context to all children
  const { init: initTheme } = createThemeContext()

  // Create and provide tracking context to all children
  const { cleanup: cleanupTracking, stopTracking } = createTrackingContext()

  let unlisten: (() => void) | null = null

  onMount(async () => {
    // Initialize theme
    await initTheme()

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
  {@render children()}
</div>
