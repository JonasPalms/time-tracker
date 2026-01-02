<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import type { Update } from "@tauri-apps/plugin-updater";

  let {
    open = $bindable(false),
    update,
    onInstall,
    onCancel,
  }: {
    open?: boolean;
    update: Update | null;
    onInstall: () => void;
    onCancel: () => void;
  } = $props();

  function handleInstall() {
    open = false;
    onInstall();
  }

  function handleCancel() {
    open = false;
    onCancel();
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content
    class="sm:max-w-lg bg-surface border-surface-hover"
    onInteractOutside={(e) => e.preventDefault()}
    onEscapeKeyDown={(e) => e.preventDefault()}
  >
    <Dialog.Header>
      <Dialog.Title>Update Available</Dialog.Title>
      <Dialog.Description>A new version of TimeTracker is ready to install.</Dialog.Description>
    </Dialog.Header>

    {#if update}
      <div class="space-y-4 py-4">
        <!-- Version Info -->
        <div class="rounded-lg border border-surface-hover p-4">
          <div class="flex items-baseline justify-between mb-2">
            <span class="text-sm text-on-surface-muted">Current Version</span>
            <span class="font-mono text-sm">{update.currentVersion}</span>
          </div>
          <div class="flex items-baseline justify-between">
            <span class="text-sm text-on-surface-muted">New Version</span>
            <span class="font-mono text-sm font-medium text-accent">{update.version}</span>
          </div>
        </div>

        <!-- Changelog Link -->
        <div class="text-center">
          <a
            href="https://github.com/JonasPalms/time-tracker/releases/tag/v{update.version}"
            target="_blank"
            rel="noopener noreferrer"
            class="text-sm text-accent hover:underline inline-flex items-center gap-1"
          >
            View changelog
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
              />
            </svg>
          </a>
        </div>
      </div>
    {/if}

    <Dialog.Footer class="flex gap-3 justify-center max-w-sm mx-auto">
      <Button variant="outline" onclick={handleCancel} class="flex-1 min-w-40">Later</Button>
      <Button onclick={handleInstall} class="bg-accent hover:bg-accent/90 flex-1 min-w-40">
        Install Update
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
