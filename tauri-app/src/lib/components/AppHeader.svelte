<script lang="ts">
  import WindowControls from "./WindowControls.svelte";
  import Icon from "./Icon.svelte";

  // Prevent drag region from activating on interactive elements
  function handleInteractiveMouseDown(e: MouseEvent) {
    e.stopPropagation();
  }
</script>

<header
  class="shrink-0 h-14 w-full flex items-center justify-between px-app"
  data-tauri-drag-region
>
  <!-- Window controls (macOS only) -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div onmousedown={handleInteractiveMouseDown}>
    <WindowControls />
  </div>

  <!-- Navigation icons -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="flex items-center gap-1" onmousedown={handleInteractiveMouseDown}>
    <a href="/" class="p-2 rounded-lg hover:bg-surface-raised transition-colors" aria-label="Home">
      <Icon name="home" class="w-5 h-5" />
    </a>
    <a
      href="/history"
      class="p-2 rounded-lg hover:bg-surface-raised transition-colors"
      aria-label="View history"
    >
      <Icon name="clock" class="w-5 h-5" />
    </a>
    <a
      href="/settings"
      class="p-2 rounded-lg hover:bg-surface-raised transition-colors"
      aria-label="Open settings"
    >
      <Icon name="settings" class="w-5 h-5" />
    </a>
  </div>
</header>
