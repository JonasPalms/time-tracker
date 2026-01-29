<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, onDestroy } from "svelte";
  import { useSidebar } from "$lib/hooks/sidebar.svelte";
  import { PanelLeftClose, PanelLeft } from "@lucide/svelte";

  const sidebar = useSidebar();

  // Import traffic light SVGs
  import noFocusIcon from "$lib/icons/traffic-lights/0-all-three-nofocus.svg?raw";
  import closeNormal from "$lib/icons/traffic-lights/1-close-1-normal.svg?raw";
  import closeHover from "$lib/icons/traffic-lights/2-close-2-hover.svg?raw";
  import closePress from "$lib/icons/traffic-lights/2-close-3-press.svg?raw";
  import minimizeNormal from "$lib/icons/traffic-lights/2-minimize-1-normal.svg?raw";
  import minimizeHover from "$lib/icons/traffic-lights/2-minimize-2-hover.svg?raw";
  import minimizePress from "$lib/icons/traffic-lights/2-minimize-3-press.svg?raw";
  import maximizeNormal from "$lib/icons/traffic-lights/3-maximize-1-normal.svg?raw";
  import maximizeHover from "$lib/icons/traffic-lights/3-maximize-2-hover.svg?raw";
  import maximizePress from "$lib/icons/traffic-lights/3-maximize-3-press.svg?raw";

  let isMacOS = $state(false);
  let isWindowFocused = $state(true);
  let isGroupHovered = $state(false);

  let closeState = $state<"normal" | "hover" | "press">("normal");
  let minimizeState = $state<"normal" | "hover" | "press">("normal");
  let maximizeState = $state<"normal" | "hover" | "press">("normal");

  // Store cleanup functions in a plain object to avoid $state TypeScript narrowing issues
  const cleanup: { focus?: () => void; blur?: () => void } = {};

  onMount(async () => {
    isMacOS = navigator.userAgent.toLowerCase().includes("mac");

    // Track Tauri window focus
    const tauriWindow = getCurrentWindow();

    cleanup.focus = await tauriWindow.onFocusChanged(({ payload: focused }) => {
      isWindowFocused = focused;
    });
  });

  onDestroy(() => {
    cleanup.focus?.();
    cleanup.blur?.();
  });

  async function handleClose() {
    const window = getCurrentWindow();
    await window.close();
  }

  async function handleMinimize() {
    const window = getCurrentWindow();
    await window.minimize();
  }

  async function handleMaximize() {
    const window = getCurrentWindow();
    const isMaximized = await window.isMaximized();
    if (isMaximized) {
      await window.unmaximize();
    } else {
      await window.maximize();
    }
  }

  function getCloseIcon() {
    if (!isWindowFocused && !isGroupHovered) return noFocusIcon;
    if (closeState === "press") return closePress;
    if (closeState === "hover" || isGroupHovered) return closeHover;
    return closeNormal;
  }

  function getMinimizeIcon() {
    if (!isWindowFocused && !isGroupHovered) return noFocusIcon;
    if (minimizeState === "press") return minimizePress;
    if (minimizeState === "hover" || isGroupHovered) return minimizeHover;
    return minimizeNormal;
  }

  function getMaximizeIcon() {
    if (!isWindowFocused && !isGroupHovered) return noFocusIcon;
    if (maximizeState === "press") return maximizePress;
    if (maximizeState === "hover" || isGroupHovered) return maximizeHover;
    return maximizeNormal;
  }
</script>

{#if isMacOS}
  <div class="flex items-center gap-2">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="flex items-center gap-2 px-2"
      onmouseenter={() => (isGroupHovered = true)}
      onmouseleave={() => {
        isGroupHovered = false;
        closeState = "normal";
        minimizeState = "normal";
        maximizeState = "normal";
      }}
    >
      <!-- Close button -->
      <button
        onclick={handleClose}
        onmouseenter={() => (closeState = "hover")}
        onmouseleave={() => (closeState = "normal")}
        onmousedown={() => (closeState = "press")}
        onmouseup={() => (closeState = "hover")}
        class="size-3 focus:outline-none [&_svg]:pointer-events-none"
        aria-label="Close window"
      >
        {@html getCloseIcon()}
      </button>

      <!-- Minimize button -->
      <button
        onclick={handleMinimize}
        onmouseenter={() => (minimizeState = "hover")}
        onmouseleave={() => (minimizeState = "normal")}
        onmousedown={() => (minimizeState = "press")}
        onmouseup={() => (minimizeState = "hover")}
        class="size-3 focus:outline-none [&_svg]:pointer-events-none"
        aria-label="Minimize window"
      >
        {@html getMinimizeIcon()}
      </button>

      <!-- Maximize button -->
      <button
        onclick={handleMaximize}
        onmouseenter={() => (maximizeState = "hover")}
        onmouseleave={() => (maximizeState = "normal")}
        onmousedown={() => (maximizeState = "press")}
        onmouseup={() => (maximizeState = "hover")}
        class="size-3 focus:outline-none [&_svg]:pointer-events-none"
        aria-label="Maximize window"
      >
        {@html getMaximizeIcon()}
      </button>
    </div>

    <!-- Sidebar toggle button -->
    <button
      onclick={sidebar.toggle}
      class="p-1 rounded hover:bg-surface-hover text-on-surface-muted hover:text-on-surface transition-colors"
      aria-label={sidebar.hidden ? "Show sidebar" : "Hide sidebar"}
    >
      {#if sidebar.hidden}
        <PanelLeft class="size-4" />
      {:else}
        <PanelLeftClose class="size-4" />
      {/if}
    </button>
  </div>
{/if}
