<script lang="ts">
  import Icon from "./Icon.svelte"
  import { type Task } from "$lib/tasks"
  import { formatTime } from "$lib/tasks"
  import { useTimeAdjust } from "$lib/timeAdjust.svelte"

  let {
    task,
    isActive,
    elapsedSeconds = 0,
    onPlayPause,
    onAdjustTime,
  }: {
    task: Task
    isActive: boolean
    elapsedSeconds?: number
    onPlayPause: () => void
    onAdjustTime: (seconds: number) => void
  } = $props()

  const timeAdjust = useTimeAdjust()

  // Calculate display time (base + current elapsed if active)
  const displayTime = $derived(() => {
    const baseSeconds = task.total_seconds
    const extraSeconds = isActive ? elapsedSeconds : 0
    return formatTime(baseSeconds + extraSeconds)
  })

  // Get adjustment interval in seconds
  const adjustmentSeconds = $derived(timeAdjust.intervalMinutes * 60)

  function handleAdd() {
    onAdjustTime(adjustmentSeconds)
  }

  function handleSubtract() {
    onAdjustTime(-adjustmentSeconds)
  }
</script>

<div class="group flex items-center gap-3 p-3 bg-surface-raised rounded-xl transition-colors">
  <!-- Play/Pause Button -->
  <button
    class="w-10 h-10 flex items-center bg-surface justify-center rounded-full transition-all {isActive
      ? 'text-accent hover:bg-accent/20'
      : 'hover:bg-accent/20'} hover:scale-105"
    onclick={onPlayPause}
    aria-label={isActive ? "Stop tracking" : "Start tracking"}
  >
    <Icon name={isActive ? "pause" : "play"} />
  </button>

  <!-- Task Name -->
  <div class="flex-1 min-w-0">
    <div class="font-medium truncate">{task.name}</div>
  </div>

  <!-- Time -->
  <div class="font-mono text-xl {isActive ? 'text-accent font-bold' : 'text-on-surface-muted'}">
    {displayTime()}
  </div>

  <!-- Adjust Time Buttons -->
  <div class="flex flex-col gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
    <button
      class="w-6 h-6 flex items-center justify-center rounded hover:bg-accent/20 transition-colors hover:scale-105 text-on-surface-muted"
      onclick={handleAdd}
      aria-label="Add {timeAdjust.intervalMinutes} {timeAdjust.intervalMinutes === 1 ? 'minute' : 'minutes'}"
    >
      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24">
        <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
      </svg>
    </button>
    <button
      class="w-6 h-6 flex items-center justify-center rounded hover:bg-accent/20 transition-colors hover:scale-105 text-on-surface-muted"
      onclick={handleSubtract}
      aria-label="Subtract {timeAdjust.intervalMinutes} {timeAdjust.intervalMinutes === 1 ? 'minute' : 'minutes'}"
    >
      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24">
        <path d="M19 13H5v-2h14v2z" />
      </svg>
    </button>
  </div>
</div>
