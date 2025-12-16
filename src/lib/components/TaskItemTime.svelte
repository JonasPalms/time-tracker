<script lang="ts">
  import { formatTime, parseTimeString } from "$lib/timeUtils";
  import { Input } from "./ui/input";

  let {
    totalSeconds,
    isTracking,
    elapsedSeconds = 0,
    onUpdateTime,
  }: {
    totalSeconds: number;
    isTracking: boolean;
    elapsedSeconds?: number;
    onUpdateTime?: (newTotalSeconds: number) => void;
  } = $props();

  let editedTimeStr = $state("");

  // Calculate display time (base + current elapsed if tracking)
  const displayTime = $derived(() => {
    const baseSeconds = totalSeconds;
    const extraSeconds = isTracking ? elapsedSeconds : 0;
    return formatTime(baseSeconds + extraSeconds);
  });

  // Format time for input (always HH:MM:SS)
  const inputTimeStr = $derived(() => {
    return formatTime(totalSeconds);
  });

  // Sync editedTimeStr with displayTime
  $effect(() => {
    editedTimeStr = displayTime();
  });

  function handleInputKeydown(e: KeyboardEvent) {
    if (isTracking) return;

    if (e.key === "Enter") {
      e.preventDefault();
      saveTime();
      (e.target as HTMLInputElement).blur();
    } else if (e.key === "Escape") {
      e.preventDefault();
      editedTimeStr = displayTime();
      (e.target as HTMLInputElement).blur();
    }
  }

  function handleInputBlur() {
    if (!isTracking) {
      saveTime();
    }
  }

  function handleInputFocus(e: FocusEvent) {
    e.stopPropagation();
  }

  function saveTime() {
    if (isTracking) return;

    const parsedSeconds = parseTimeString(editedTimeStr);
    if (parsedSeconds !== null && parsedSeconds !== totalSeconds && onUpdateTime) {
      onUpdateTime(parsedSeconds);
    } else {
      // Reset to display time if invalid or unchanged
      editedTimeStr = displayTime();
    }
  }
</script>

<Input
  type="time"
  bind:value={editedTimeStr}
  placeholder="HH:MM:SS"
  disabled={isTracking}
  readonly={isTracking}
  class="font-mono text-xl md:text-xl w-32 appearance-none border-none bg-transparent! dark:bg-transparent! disabled:opacity-100! {isTracking
    ? 'text-accent! font-bold cursor-default'
    : 'text-on-surface-muted cursor-text'} transition-colors"
  onkeydown={handleInputKeydown}
  onblur={handleInputBlur}
  onfocus={handleInputFocus}
  aria-label={isTracking ? "Time is being tracked" : "Time (click to edit)"}
/>

<style>
  /* Override webkit datetime edit field colors to match app theme */
  :global(input[type="time"])::-webkit-datetime-edit-hour-field,
  :global(input[type="time"])::-webkit-datetime-edit-minute-field,
  :global(input[type="time"])::-webkit-datetime-edit-second-field {
    color: var(--muted-foreground);
  }

  :global(input[type="time"])::-webkit-datetime-edit-text {
    color: var(--muted-foreground);
  }

  :global(input[type="time"]:disabled)::-webkit-datetime-edit-hour-field,
  :global(input[type="time"]:disabled)::-webkit-datetime-edit-minute-field,
  :global(input[type="time"]:disabled)::-webkit-datetime-edit-second-field {
    color: var(--accent) !important;
    font-weight: bold;
  }

  :global(input[type="time"]:disabled)::-webkit-datetime-edit-text {
    color: var(--accent) !important;
  }

  :global(input[type="time"])::-webkit-datetime-edit-hour-field:focus,
  :global(input[type="time"])::-webkit-datetime-edit-minute-field:focus,
  :global(input[type="time"])::-webkit-datetime-edit-second-field:focus {
    background-color: var(--accent);
    color: var(--accent-foreground);
    outline: none;
  }
</style>
