<script lang="ts">
  import Icon from "./Icon.svelte";
  import { type Task } from "$lib/tasks";
  import { formatTime } from "$lib/timeUtils";
  import { useTimeAdjust } from "$lib/timeAdjust.svelte";
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
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
    onAdjustTime(adjustmentSeconds);
  }

  function handleSubtract(e: MouseEvent) {
    e.stopPropagation();
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
  class="group flex items-center gap-3 p-3 rounded-xl transition-colors hover:bg-surface-raised focus-within:bg-surface-raised"
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

  <!-- Adjust Time Buttons -->
  <div
    class="flex flex-col gap-0.5 opacity-0 group-hover:opacity-100 group-focus:opacity-100 transition-opacity"
  >
    <button
      class="w-6 h-6 flex items-center justify-center rounded hover:bg-accent/20 transition-colors hover:scale-105 text-on-surface-muted"
      onclick={handleAdd}
      aria-label="Add {timeAdjust.intervalMinutes} {timeAdjust.intervalMinutes === 1
        ? 'minute'
        : 'minutes'}"
    >
      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24">
        <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
      </svg>
    </button>
    <button
      class="w-6 h-6 flex items-center justify-center rounded hover:bg-accent/20 transition-colors hover:scale-105 text-on-surface-muted"
      onclick={handleSubtract}
      aria-label="Subtract {timeAdjust.intervalMinutes} {timeAdjust.intervalMinutes === 1
        ? 'minute'
        : 'minutes'}"
    >
      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24">
        <path d="M19 13H5v-2h14v2z" />
      </svg>
    </button>
  </div>
  {#if onDelete}
    <DropdownMenu.Root bind:open={dropdownOpen}>
      <DropdownMenu.Trigger>
        <Button
          variant="ghost"
          size="icon-sm"
          class="opacity-0 group-hover:opacity-100 group-focus:opacity-100 transition-opacity"
          aria-label="Open menu"
        >
          <EllipsisIcon class="w-4 h-4" />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content class="w-[180px]" align="end">
        <DropdownMenu.Item variant="destructive" onselect={handleDelete}>
          <TrashIcon class="me-2 size-4" />
          Delete
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  {/if}
</div>
