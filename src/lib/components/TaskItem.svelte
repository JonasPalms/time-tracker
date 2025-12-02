<script lang="ts">
  import Icon from "./Icon.svelte";
  import { type Task } from "$lib/tasks";
  import { formatTime } from "$lib/timeUtils";
  import { useTimeAdjust } from "$lib/timeAdjust.svelte";
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import MinusIcon from "@lucide/svelte/icons/minus";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";

  let {
    task,
    isTracking,
    elapsedSeconds = 0,
    onPlayPause,
    onAdjustTime,
    onUpdateName,
    onDelete,
  }: {
    task: Task;
    isTracking: boolean;
    elapsedSeconds?: number;
    onPlayPause: () => void;
    onAdjustTime: (seconds: number) => void;
    onUpdateName?: (newName: string) => void;
    onDelete?: () => void;
  } = $props();

  const timeAdjust = useTimeAdjust();

  // State for edited name
  let editedName = $state(task.name);
  let dropdownOpen = $state(false);

  // Sync editedName with task.name when task prop changes externally
  $effect(() => {
    editedName = task.name;
  });

  // Calculate display time (base + current elapsed if tracking)
  const displayTime = $derived(() => {
    const baseSeconds = task.total_seconds;
    const extraSeconds = isTracking ? elapsedSeconds : 0;
    return formatTime(baseSeconds + extraSeconds);
  });

  // Get adjustment interval in seconds
  const adjustmentSeconds = $derived(timeAdjust.intervalMinutes * 60);

  function handleAdd(e: MouseEvent) {
    e.stopPropagation();
    e.preventDefault(); // Prevent dropdown from closing
    onAdjustTime(adjustmentSeconds);
  }

  function handleSubtract(e: MouseEvent) {
    e.stopPropagation();
    e.preventDefault(); // Prevent dropdown from closing
    onAdjustTime(-adjustmentSeconds);
  }

  function handleDelete(e?: MouseEvent | Event) {
    if (e) {
      e.stopPropagation();
    }
    if (onDelete) {
      onDelete();
    }
    dropdownOpen = false;
  }

  function handlePlayPause(e: MouseEvent) {
    e.stopPropagation();
    onPlayPause();
  }

  function handleInputClick(e: MouseEvent) {
    e.stopPropagation();
  }

  function handleInputFocus(e: FocusEvent) {
    e.stopPropagation();
  }

  function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      saveName();
      (e.target as HTMLInputElement).blur();
    } else if (e.key === "Escape") {
      e.preventDefault();
      editedName = task.name;
      (e.target as HTMLInputElement).blur();
    }
  }

  function handleInputBlur() {
    saveName();
  }

  function saveName() {
    if (editedName.trim() !== task.name.trim() && onUpdateName) {
      onUpdateName(editedName.trim());
    }
  }
</script>

<div
  class="group flex items-center gap-3 px-2 py-3 rounded-xl transition-colors hover:bg-surface-raised focus-within:bg-surface-raised"
>
  <!-- Play/Stop Button -->
  <button
    class="w-12 h-12 flex items-center justify-center rounded-full transition-transform {isTracking
      ? 'text-accent'
      : ''} hover:text-accent hover:scale-120"
    onclick={handlePlayPause}
    aria-label={isTracking ? "Stop tracking" : "Start tracking"}
  >
    <Icon name={isTracking ? "stop" : "play"} class="w-6 h-6" />
  </button>

  <!-- Task Name -->
  <div class="flex-1 min-w-0">
    <input
      type="text"
      bind:value={editedName}
      class="font-medium truncate px-1 -mx-1 rounded selection:bg-accent selection:text-on-accent"
      onclick={handleInputClick}
      onfocus={handleInputFocus}
      onkeydown={handleInputKeydown}
      onblur={handleInputBlur}
    />
  </div>

  <!-- Time -->
  <div class="font-mono text-xl {isTracking ? 'text-accent font-bold' : 'text-on-surface-muted'}">
    {displayTime()}
  </div>

  {#if onDelete}
    <DropdownMenu.Root bind:open={dropdownOpen}>
      <DropdownMenu.Trigger>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="Open menu"
          class="opacity-0 group-hover:opacity-100 group-focus:opacity-100 focus-within:opacity-100 {dropdownOpen
            ? 'opacity-100'
            : ''} transition-opacity"
        >
          <EllipsisIcon class="w-4 h-4" />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content class="w-[180px]" align="end">
        <!-- Inline time adjustment buttons -->
        <div class="flex items-center">
          <button
            class="flex-1 flex items-center justify-center gap-2 rounded-sm px-1 py-1.5 text-sm hover:bg-muted transition-colors"
            onclick={handleSubtract}
            aria-label="Subtract {timeAdjust.intervalMinutes} {timeAdjust.intervalMinutes === 1
              ? 'minute'
              : 'minutes'}"
          >
            <MinusIcon class="size-4" />
            <span>Sub</span>
          </button>
          <button
            class="flex-1 flex items-center justify-center gap-2 rounded-sm px-1 py-1.5 text-sm hover:bg-muted transition-colors"
            onclick={handleAdd}
            aria-label="Add {timeAdjust.intervalMinutes} {timeAdjust.intervalMinutes === 1
              ? 'minute'
              : 'minutes'}"
          >
            <PlusIcon class="size-4" />
            <span>Add</span>
          </button>
        </div>
        <DropdownMenu.Separator />
        <DropdownMenu.Item onclick={handleDelete} variant="destructive">
          <TrashIcon class="me-2 size-4" />
          Delete
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  {/if}
</div>
