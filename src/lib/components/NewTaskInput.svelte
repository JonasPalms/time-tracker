<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Tween } from "svelte/motion";
  import { backInOut } from "svelte/easing";
  import * as Command from "$lib/components/ui/command/index.js";
  import CommandIcon from "@lucide/svelte/icons/command";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import { useFavourites } from "$lib/hooks/favourites.svelte";
  import { useKeyboard } from "$lib/hooks/keyboard.svelte";
  import { useModalState } from "$lib/hooks/modal-state.svelte";
  import { formatTimeHuman } from "$lib/utils/time";

  let {
    onAddTask,
    suggestions = [],
    raised = false,
  }: {
    onAddTask: (taskName: string, initialSeconds?: number) => void;
    suggestions?: string[];
    raised?: boolean;
  } = $props();

  const NO_SELECTION = "__none__";
  const FORCE_DIALOG_OPEN = false;
  const MODAL_ID = "new-task-dialog";

  let newTaskName = $state("");
  let open = $state(FORCE_DIALOG_OPEN);
  let commandRef = $state<HTMLDivElement>(null!);
  let commandValue = $state("");
  let isUsingKeyboard = $state(false);
  let isMacOS = $state(true);

  // Favourites from shared context
  const favouritesContext = useFavourites();
  const keyboard = useKeyboard();
  const modalState = useModalState();
  const lift = Tween.of(() => (raised ? -100 : 0), { duration: 200, easing: backInOut });

  // Register Cmd+N shortcut to focus input
  let unregisterShortcut: (() => void) | null = null;
  let unlistenWindowFocus: (() => void) | null = null;

  onMount(async () => {
    isMacOS = navigator.userAgent.toLowerCase().includes("mac");

    unregisterShortcut = keyboard.register("focus-new-task", (e) => {
      const isMac = navigator.userAgent.toLowerCase().includes("mac");
      const modKey = isMac ? e.metaKey : e.ctrlKey;

      if (FORCE_DIALOG_OPEN) {
        return false;
      }

      if (modKey && e.key === "n") {
        if (modalState.hasOtherOpen(MODAL_ID)) {
          return true;
        }

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
    modalState.setOpen(MODAL_ID, open);

    if (FORCE_DIALOG_OPEN && !open) {
      open = true;
      return;
    }

    if (open) {
      commandValue = NO_SELECTION;
    }
  });

  $effect(() => {
    return () => {
      modalState.setOpen(MODAL_ID, false);
    };
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

    if (modalState.hasOtherOpen(MODAL_ID)) {
      return;
    }

    newTaskName = "";
    open = true;
  }
</script>

<div
  class="pointer-events-none absolute right-5 bottom-5 z-20"
  style="transform: translateY({lift.current}px)"
>
  <div class="group pointer-events-auto relative">
    <button
      type="button"
      class="flex size-11 items-center justify-center rounded-full bg-accent text-on-accent shadow-lg"
      aria-haspopup="dialog"
      aria-expanded={open}
      aria-label="New task"
      onclick={openDialog}
    >
      <PlusIcon class="size-6" />
    </button>
    <div
      role="tooltip"
      class="pointer-events-none absolute right-0 bottom-full mb-2 flex items-center gap-2 whitespace-nowrap rounded-lg bg-stone-900 px-3 py-1.5 text-sm text-white opacity-0 shadow-md transition-opacity delay-0 duration-150 group-hover:opacity-100 group-hover:delay-300 group-focus-within:opacity-100 group-focus-within:delay-300 dark:bg-stone-100 dark:text-stone-900"
    >
      <span>New task</span>
      <span class="inline-flex items-center gap-0.5 text-white/70 dark:text-stone-500" aria-hidden="true">
        {#if isMacOS}
          <CommandIcon class="size-3.5" />
        {:else}
          <span class="text-xs font-medium">Ctrl</span>
        {/if}
        <span class="text-xs font-medium">N</span>
      </span>
    </div>
  </div>

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
