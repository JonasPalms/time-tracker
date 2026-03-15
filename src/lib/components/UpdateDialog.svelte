<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import type { Update } from "@tauri-apps/plugin-updater";
  import type { ChangelogSection } from "$lib/services/changelog";

  let {
    open = $bindable(false),
    update,
    releaseNotes = [],
    onInstall,
    onCancel,
  }: {
    open?: boolean;
    update: Update | null;
    releaseNotes?: ChangelogSection[];
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
    onInteractOutside={(e: PointerEvent) => e.preventDefault()}
    onEscapeKeydown={(e: KeyboardEvent) => e.preventDefault()}
  >
    <Dialog.Header>
      <Dialog.Title>Update Available</Dialog.Title>
      <Dialog.Description>A new version of TimeTracker is ready to install.</Dialog.Description>
    </Dialog.Header>

    {#if update}
      <div class="space-y-4 py-4">
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

        {#if releaseNotes.length > 0}
          <div class="rounded-lg border border-surface-hover p-4 space-y-4">
            <div>
              <h3 class="text-sm font-medium text-on-surface">What's new</h3>
              <p class="text-sm text-on-surface-muted mt-1">
                Here's what is included in version {update.version}.
              </p>
            </div>

            <div class="space-y-3 max-h-64 overflow-y-auto pr-1">
              {#each releaseNotes as section}
                <section class="space-y-2">
                  <h4
                    class="text-xs font-semibold uppercase tracking-[0.12em] text-on-surface-muted"
                  >
                    {section.title}
                  </h4>
                  <ul class="space-y-2">
                    {#each section.items as item}
                      <li class="flex gap-2 text-sm text-on-surface-muted leading-5">
                        <span class="mt-2 h-1.5 w-1.5 shrink-0 rounded-full bg-accent"></span>
                        <span>{item}</span>
                      </li>
                    {/each}
                  </ul>
                </section>
              {/each}
            </div>
          </div>
        {/if}

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
