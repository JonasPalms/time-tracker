<script lang="ts">
  import { tick, onMount, onDestroy } from "svelte";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Command from "$lib/components/ui/command/index.js";
  import CommandIcon from "@lucide/svelte/icons/command";

  let {
    onAddTask,
    suggestions = [],
  }: {
    onAddTask: (taskName: string, initialSeconds?: number) => void;
    suggestions?: string[];
  } = $props();

  let newTaskName = $state("");
  let open = $state(false);
  let triggerRef = $state<HTMLInputElement>(null!);
  let commandRef = $state<HTMLDivElement>(null!);
  let isUsingKeyboard = $state(false);

  // Favourites - hardcoded for now
  const favourites = [{ name: "Frokost", duration: "30m", seconds: 30 * 60 }];

  // Filter favourites based on input
  const filteredFavourites = $derived(
    newTaskName.trim()
      ? favourites.filter((f) => f.name.toLowerCase().includes(newTaskName.toLowerCase()))
      : favourites
  );

  // Get favourite names for filtering
  const favouriteNames = new Set(favourites.map((f) => f.name.toLowerCase()));

  // Filter suggestions based on input, excluding favourites
  const filteredSuggestions = $derived(
    (newTaskName.trim()
      ? suggestions.filter((s) => s.toLowerCase().includes(newTaskName.toLowerCase()))
      : suggestions
    ).filter((s) => !favouriteNames.has(s.toLowerCase()))
  );

  function closeAndFocusTrigger() {
    open = false;
    tick().then(() => {
      triggerRef?.focus();
    });
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!newTaskName.trim()) return;

    onAddTask(newTaskName.trim());
    newTaskName = "";
    closeAndFocusTrigger();
  }

  function handleSelect(suggestion: string, seconds?: number) {
    onAddTask(suggestion, seconds);
    newTaskName = "";
    closeAndFocusTrigger();
  }

  function handleKeydown(e: KeyboardEvent) {
    // Forward arrow keys to the Command component for navigation
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault();
      isUsingKeyboard = true;
      commandRef?.dispatchEvent(
        new KeyboardEvent("keydown", {
          key: e.key,
          bubbles: true,
        })
      );
    } else if (e.key === "Enter") {
      // If we have a selected item, let Command handle it
      const selectedItem = commandRef?.querySelector("[aria-selected='true']");
      if (selectedItem && open) {
        e.preventDefault();
        commandRef?.dispatchEvent(
          new KeyboardEvent("keydown", {
            key: "Enter",
            bubbles: true,
          })
        );
      }
      // Otherwise, let the form submit handle it
    }
  }

  function handleMouseMove() {
    isUsingKeyboard = false;
  }

  function handleGlobalKeydown(e: KeyboardEvent) {
    // Cmd+N (Mac) or Ctrl+N (Windows/Linux) to focus input
    if ((e.metaKey || e.ctrlKey) && e.key === "n") {
      e.preventDefault();
      triggerRef?.click();
      triggerRef?.focus();
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleGlobalKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleGlobalKeydown);
  });
</script>

<div class="relative mb-4">
  <Popover.Root bind:open>
    <Popover.Trigger bind:ref={triggerRef} class="w-full block">
      {#snippet child({ props })}
        <form onsubmit={handleSubmit} class="relative">
          <input
            {...props}
            type="text"
            bind:value={newTaskName}
            placeholder="What are you working on?"
            class="w-full px-4 py-3 pr-16 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted focus:outline-none focus:ring-1 focus:ring-on-surface/50"
            role="combobox"
            autocomplete="off"
            onkeydown={handleKeydown}
          />
          {#if !newTaskName}
            <div
              class="absolute right-5 top-1/2 -translate-y-1/2 flex items-center gap-1 text-on-surface-muted pointer-events-none"
            >
              <CommandIcon class="size-4" />
              <span class="text-md font-medium">N</span>
            </div>
          {/if}
        </form>
      {/snippet}
    </Popover.Trigger>
    {#if filteredFavourites.length > 0 || filteredSuggestions.length > 0}
      <Popover.Content
        class="w-(--bits-popover-anchor-width) p-0 bg-surface-raised border-surface-hover"
        sideOffset={8}
        align="start"
        onOpenAutoFocus={(e) => e.preventDefault()}
      >
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          onmousemove={handleMouseMove}
          class={isUsingKeyboard ? "**:data-command-item:pointer-events-none!" : ""}
        >
          <Command.Root bind:ref={commandRef} class="bg-transparent" shouldFilter={false}>
            <Command.List class="max-h-64">
              {#if filteredFavourites.length > 0}
                <Command.Group heading="Favourites">
                  {#each filteredFavourites as favourite (favourite.name)}
                    <Command.Item
                      value={favourite.name}
                      onSelect={() => handleSelect(favourite.name, favourite.seconds)}
                      class="cursor-pointer py-2 pr-3 mr-2 text-on-surface aria-selected:bg-surface-hover aria-selected:text-on-surface text-base {isUsingKeyboard
                        ? ''
                        : 'hover:bg-surface-hover'}"
                    >
                      <span>{favourite.name}</span>
                      <span class="ml-auto text-on-surface-muted text-sm">{favourite.duration}</span
                      >
                    </Command.Item>
                  {/each}
                </Command.Group>
              {/if}
              {#if filteredSuggestions.length > 0}
                <Command.Group heading="Recent">
                  {#each filteredSuggestions as suggestion (suggestion)}
                    <Command.Item
                      value={suggestion}
                      onSelect={() => handleSelect(suggestion)}
                      class="cursor-pointer py-2 pr-3 mr-2 text-on-surface aria-selected:bg-surface-hover aria-selected:text-on-surface text-base {isUsingKeyboard
                        ? ''
                        : 'hover:bg-surface-hover'}"
                    >
                      {suggestion}
                    </Command.Item>
                  {/each}
                </Command.Group>
              {/if}
            </Command.List>
          </Command.Root>
        </div>
      </Popover.Content>
    {/if}
  </Popover.Root>
</div>
