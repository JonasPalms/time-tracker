<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import * as Command from "$lib/components/ui/command/index.js";
  import CommandIcon from "@lucide/svelte/icons/command";
  import { useFavourites } from "$lib/hooks/favourites.svelte";
  import { useKeyboard } from "$lib/hooks/keyboard.svelte";
  import { formatTimeHuman } from "$lib/utils/time";

  let {
    onAddTask,
    suggestions = [],
  }: {
    onAddTask: (taskName: string, initialSeconds?: number) => void;
    suggestions?: string[];
  } = $props();

  const NO_SELECTION = "__none__";
  const FORCE_DIALOG_OPEN = false;

  let newTaskName = $state("");
  let open = $state(FORCE_DIALOG_OPEN);
  let commandRef = $state<HTMLDivElement>(null!);
  let commandValue = $state("");
  let isUsingKeyboard = $state(false);

  // Favourites from shared context
  const favouritesContext = useFavourites();
  const keyboard = useKeyboard();

  // Register Cmd+N shortcut to focus input
  let unregisterShortcut: (() => void) | null = null;
  let unlistenWindowFocus: (() => void) | null = null;

  onMount(async () => {
    unregisterShortcut = keyboard.register("focus-new-task", (e) => {
      const isMac = navigator.userAgent.toLowerCase().includes("mac");
      const modKey = isMac ? e.metaKey : e.ctrlKey;

      if (FORCE_DIALOG_OPEN) {
        return false;
      }

      if (modKey && e.key === "n") {
        if (open) {
          open = false;
        } else {
          newTaskName = "";
          open = true;
        }
        return true; // Handled
      }
      return false;
    });

    // Close dropdown when window loses focus
    const tauriWindow = getCurrentWindow();
    unlistenWindowFocus = await tauriWindow.onFocusChanged(({ payload: focused }) => {
      if (FORCE_DIALOG_OPEN) {
        open = true;
        return;
      }

      if (!focused) {
        open = false;
      }
    });
  });

  onDestroy(() => {
    unregisterShortcut?.();
    unlistenWindowFocus?.();
  });

  // Filter favourites based on input
  const filteredFavourites = $derived(
    newTaskName.trim()
      ? favouritesContext.favourites.filter((f) =>
          f.name.toLowerCase().includes(newTaskName.toLowerCase())
        )
      : favouritesContext.favourites
  );

  // Get favourite names for filtering
  const favouriteNames = $derived(
    new Set(favouritesContext.favourites.map((f) => f.name.toLowerCase()))
  );

  // Filter suggestions based on input, excluding favourites
  const filteredSuggestions = $derived(
    (newTaskName.trim()
      ? suggestions.filter((s) => s.toLowerCase().includes(newTaskName.toLowerCase()))
      : suggestions
    ).filter((s) => !favouriteNames.has(s.toLowerCase()))
  );

  // Whether dropdown should show
  const hasItems = $derived(filteredFavourites.length > 0 || filteredSuggestions.length > 0);

  $effect(() => {
    if (FORCE_DIALOG_OPEN && !open) {
      open = true;
      return;
    }

    if (open) {
      commandValue = NO_SELECTION;
    }
  });

  function handleSubmit() {
    if (FORCE_DIALOG_OPEN) return;
    if (!newTaskName.trim()) return;

    onAddTask(newTaskName.trim());
    newTaskName = "";
    open = false;
  }

  function handleSelect(suggestion: string, seconds?: number) {
    if (FORCE_DIALOG_OPEN) return;
    onAddTask(suggestion, seconds);
    newTaskName = "";
    open = false;
  }

  function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      isUsingKeyboard = true;
    }

    if (e.key === "Enter") {
      const selectedItem = commandRef?.querySelector(
        "[aria-selected='true']"
      ) as HTMLElement | null;

      if (!selectedItem && newTaskName.trim()) {
        e.preventDefault();
        handleSubmit();
      }
      return;
    }

    if (e.key === "Escape" && newTaskName) {
      e.preventDefault();
      newTaskName = "";
    }
  }

  function handleSuggestionsMouseMove() {
    isUsingKeyboard = false;
  }

  function openDialog() {
    if (FORCE_DIALOG_OPEN) {
      open = true;
      return;
    }

    newTaskName = "";
    open = true;
  }
</script>

<div class="mb-4">
  <button
    type="button"
    class="group relative w-full rounded-lg bg-surface-raised px-4 py-3 pr-16 text-left text-on-surface outline-none transition-colors focus-visible:ring-2 focus-visible:ring-on-surface/20"
    aria-haspopup="dialog"
    aria-expanded={open}
    onclick={openDialog}
  >
    <span class="text-on-surface-muted">What are you working on?</span>
    <div
      class="absolute right-5 top-1/2 hidden -translate-y-1/2 items-center gap-1 text-on-surface-muted pointer-events-none group-hover:flex"
    >
      <CommandIcon class="size-4" />
      <span class="text-md font-medium">N</span>
    </div>
  </button>

  <Command.Dialog
    bind:open
    bind:ref={commandRef}
    bind:value={commandValue}
    shouldFilter={false}
    title="Create task"
    description="Search favourites or recents, or press Enter to create a new task."
    class="border-none text-on-surface shadow-2xl **:data-[slot=command]:p-2 [&_[data-slot=command-input-wrapper]]:h-14 [&_[data-slot=command-input-wrapper]]:rounded-lg [&_[data-slot=command-input-wrapper]]:border [&_[data-slot=command-input-wrapper]]:border-border  [&_[data-slot=command-input-wrapper]]:bg-surface-elevated [&_[data-slot=command-input-wrapper]]:px-4 [&_[data-slot=command-input-wrapper]_svg]:size-5 [&_[data-slot=command-list]]:pt-3"
  >
    <Command.Input
      bind:value={newTaskName}
      placeholder="Search or create a task"
      class="h-14 bg-transparent text-base placeholder:text-on-surface-muted"
      autocomplete="off"
      autocorrect="off"
      autocapitalize="off"
      spellcheck="false"
      onkeydown={handleInputKeydown}
    />
    <Command.List class="min-h-64 max-h-90 pt-3" onmousemove={handleSuggestionsMouseMove}>
      <Command.Empty class="py-8 text-on-surface-muted">
        {#if newTaskName.trim()}
          <button
            type="button"
            class="w-full rounded-lg border border-dashed border-on-surface/10 px-4 py-3 text-left transition-colors hover:bg-surface-raised"
            onclick={handleSubmit}
          >
            Create <span class="font-medium text-on-surface">{newTaskName.trim()}</span>
          </button>
        {:else}
          Start typing to create a task.
        {/if}
      </Command.Empty>

      {#if filteredFavourites.length > 0}
        <Command.Group
          heading="Favourites"
          class={isUsingKeyboard ? "px-0 **:data-command-item:pointer-events-none!" : "px-0"}
        >
          {#each filteredFavourites as favourite (favourite.name)}
            <Command.Item
              value={favourite.name}
              keywords={[favourite.name]}
              onSelect={() => handleSelect(favourite.name, favourite.duration_seconds)}
              class="mr-2 cursor-pointer py-2 pr-3 text-base text-on-surface aria-selected:bg-surface-hover/70 aria-selected:text-on-surface hover:bg-surface-hover/70"
            >
              <span>{favourite.name}</span>
              <span class="ml-auto text-sm text-on-surface-muted"
                >{formatTimeHuman(favourite.duration_seconds)}</span
              >
            </Command.Item>
          {/each}
        </Command.Group>
      {/if}

      {#if filteredSuggestions.length > 0}
        <Command.Group
          heading="Recent"
          class={isUsingKeyboard ? "px-0 **:data-command-item:pointer-events-none!" : "px-0"}
        >
          {#each filteredSuggestions as suggestion (suggestion)}
            <Command.Item
              value={suggestion}
              keywords={[suggestion]}
              onSelect={() => handleSelect(suggestion)}
              class="mr-2 cursor-pointer py-2 pr-3 text-base text-on-surface aria-selected:bg-surface-hover/70 aria-selected:text-on-surface hover:bg-surface-hover/70"
            >
              {suggestion}
            </Command.Item>
          {/each}
        </Command.Group>
      {/if}
    </Command.List>
  </Command.Dialog>
</div>
