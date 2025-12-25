<script lang="ts">
  import { formatTime, parseTimeString } from "$lib/utils/time";
  import { Input } from "./ui/input";
  import "./TaskItemTime.css";

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
  step="1"
  bind:value={editedTimeStr}
  placeholder="HH:MM:SS"
  disabled={isTracking}
  readonly={isTracking}
  class="font-mono text-xl transition-colors md:text-xl w-33 -mr-3 appearance-none border-none bg-transparent! disabled:opacity-100! disabled:cursor-default {!isTracking &&
    'cursor-text'}"
  onkeydown={handleInputKeydown}
  onblur={handleInputBlur}
  onfocus={handleInputFocus}
  aria-label={isTracking ? "Time is being tracked" : "Time (click to edit)"}
/>
