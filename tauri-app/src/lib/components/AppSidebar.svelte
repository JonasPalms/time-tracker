<script lang="ts">
  import { useSidebar } from "$lib/hooks/sidebar.svelte";
  import SidebarNavItem from "./SidebarNavItem.svelte";
  import SettingsDialog from "./SettingsDialog.svelte";

  const sidebar = useSidebar();

  let settingsOpen = $state(false);
  let isResizing = $state(false);

  function handleResizeStart(e: MouseEvent) {
    e.preventDefault();
    isResizing = true;

    const startX = e.clientX;
    const startWidth = sidebar.width;

    function handleMouseMove(e: MouseEvent) {
      const delta = e.clientX - startX;
      sidebar.setWidth(startWidth + delta);
    }

    function handleMouseUp() {
      isResizing = false;
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    }

    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
  }
</script>

<aside
  class="shrink-0 flex flex-col relative border-r border-border"
  style="width: {sidebar.width}px"
  data-tauri-drag-region
>
  <!-- Navigation -->
  <nav class="flex-1 flex flex-col gap-1 p-2" data-tauri-drag-region>
    <SidebarNavItem href="/" icon="home" label="Home" collapsed={sidebar.collapsed} />
    <SidebarNavItem href="/history" icon="clock" label="History" collapsed={sidebar.collapsed} />

    <!-- Spacer for future integrations -->
    <div class="flex-1" data-tauri-drag-region></div>

    <!-- Settings -->
    <SidebarNavItem
      icon="settings"
      label="Settings"
      collapsed={sidebar.collapsed}
      onclick={() => (settingsOpen = true)}
    />
  </nav>

  <!-- Resize handle -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="absolute top-0 -right-0.5 w-1 h-full cursor-col-resize transition-colors
      {isResizing ? 'bg-on-surface/30' : 'hover:bg-on-surface/20'}"
    onmousedown={handleResizeStart}
  ></div>
</aside>

<SettingsDialog bind:open={settingsOpen} />
