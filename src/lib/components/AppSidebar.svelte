<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { useKeyboard } from "$lib/hooks/keyboard.svelte";
  import { useSidebar } from "$lib/hooks/sidebar.svelte";
  import SidebarNavItem from "./SidebarNavItem.svelte";
  import SettingsDialog from "./SettingsDialog.svelte";

  const keyboard = useKeyboard();
  const sidebar = useSidebar();

  let settingsOpen = $state(false);
  let unregisterShortcut: (() => void) | null = null;

  onMount(() => {
    unregisterShortcut = keyboard.register("open-settings", (e) => {
      const isMac = navigator.userAgent.toLowerCase().includes("mac");
      const modKey = isMac ? e.metaKey : e.ctrlKey;

      if (modKey && !e.shiftKey && !e.altKey && e.key === ",") {
        settingsOpen = true;
        return true;
      }

      return false;
    });
  });

  onDestroy(() => {
    unregisterShortcut?.();
  });
</script>

<aside
  class="shrink-0 flex flex-col relative overflow-hidden transition-[width] duration-200 ease-out"
  style="width: {sidebar.hidden ? 0 : sidebar.width}px"
  data-tauri-drag-region
>
  <div class="shrink-0 h-[52px]" data-tauri-drag-region></div>

  <nav class="flex-1 flex flex-col gap-0.5 px-2 pt-1 pb-2" data-tauri-drag-region>
    <SidebarNavItem href="/" icon="home" label="Home" />
    <SidebarNavItem href="/history" icon="clock" label="History" />

    <div class="flex-1" data-tauri-drag-region></div>

    <SidebarNavItem icon="settings" label="Settings" onclick={() => (settingsOpen = true)} />
  </nav>
</aside>

<SettingsDialog bind:open={settingsOpen} />
