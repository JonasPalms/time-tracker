<script lang="ts">
  import Icon from "./Icon.svelte"
  import { formatTime } from "$lib/tasks"
  import type { Task } from "$lib/tasks"

  let {
    isTracking,
    selectedTask,
    currentTask,
    elapsedSeconds,
    onPlayPause,
  }: {
    isTracking: boolean
    selectedTask: Task | null
    currentTask: Task | null
    elapsedSeconds: number
    onPlayPause: () => void
  } = $props()

  // Show selected task if not tracking, otherwise show tracking task
  const displayTask = $derived(selectedTask || currentTask)
  const showTimer = $derived(isTracking && currentTask !== null)
</script>

{#if displayTask}
  <div class="w-full rounded-xl p-4 transition-colors bg-surface-raised border border-on-surface/10 shadow-lg pointer-events-auto">
    <div class="flex items-center justify-between gap-4 h-9">
      <div class="font-semibold text-lg flex-1 truncate">{displayTask.name}</div>
      {#if showTimer}
        <div class="text-3xl font-mono font-bold text-accent">
          {formatTime(elapsedSeconds)}
        </div>
      {/if}
      <button
        class="w-10 h-10 flex items-center justify-center bg-accent text-on-accent rounded-lg transition-transform hover:scale-105"
        onclick={onPlayPause}
        aria-label={isTracking ? "Pause tracking" : "Start tracking"}
      >
        <Icon name={isTracking ? "pause" : "play"} />
      </button>
    </div>
  </div>
{:else}
  <div class="rounded-xl p-4 bg-surface-raised border border-transparent pointer-events-auto">
    <div class="flex items-center justify-center gap-2 text-on-surface-muted h-9">
      <span>Select a task to start tracking</span>
    </div>
  </div>
{/if}
