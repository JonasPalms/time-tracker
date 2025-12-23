<script lang="ts">
  import { tick } from "svelte";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Command from "$lib/components/ui/command/index.js";

  let {
    onAddTask,
    suggestions = [],
  }: {
    onAddTask: (taskName: string) => void;
    suggestions?: string[];
  } = $props();

  let newTaskName = $state("");
  let open = $state(false);
  let triggerRef = $state<HTMLInputElement>(null!);
  let commandRef = $state<HTMLDivElement>(null!);
  let isUsingKeyboard = $state(false);

  // Filter suggestions based on input
  const filteredSuggestions = $derived(
    newTaskName.trim()
      ? suggestions.filter((s) => s.toLowerCase().includes(newTaskName.toLowerCase()))
      : suggestions
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

  function handleSelect(suggestion: string) {
    onAddTask(suggestion);
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
</script>

<div class="relative mb-4">
  <Popover.Root bind:open>
    <Popover.Trigger bind:ref={triggerRef} class="w-full block">
      {#snippet child({ props })}
        <form onsubmit={handleSubmit}>
          <input
            {...props}
            type="text"
            bind:value={newTaskName}
            placeholder="What are you working on?"
            class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted focus:outline-none focus:ring-1 focus:ring-on-surface/50"
            role="combobox"
            autocomplete="off"
            onkeydown={handleKeydown}
          />
        </form>
      {/snippet}
    </Popover.Trigger>
    {#if filteredSuggestions.length > 0}
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
            <Command.List class="max-h-48">
              <Command.Group>
                {#each filteredSuggestions as suggestion (suggestion)}
                  <Command.Item
                    value={suggestion}
                    onSelect={() => handleSelect(suggestion)}
                    class="cursor-pointer py-2 text-on-surface aria-selected:bg-surface-hover aria-selected:text-on-surface text-base {isUsingKeyboard
                      ? ''
                      : 'hover:bg-surface-hover'}"
                  >
                    {suggestion}
                  </Command.Item>
                {/each}
              </Command.Group>
            </Command.List>
          </Command.Root>
        </div>
      </Popover.Content>
    {/if}
  </Popover.Root>
</div>
