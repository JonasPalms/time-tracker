<script lang="ts">
  import Icon from "./Icon.svelte"
  import { formatTime } from "$lib/tasks"
  import type { Task } from "$lib/tasks"

  let {
    isTracking,
    currentTask,
    elapsedSeconds,
    onStop,
  }: {
    isTracking: boolean
    currentTask: Task | null
    elapsedSeconds: number
    onStop: () => void
  } = $props()
</script>

{#if isTracking && currentTask}
  <button
    class="w-full rounded-xl p-4 mb-6 transition-colors bg-surface-raised border border-on-surface/10 hover:bg-accent/20 text-left"
    onclick={onStop}
    aria-label="Stop tracking"
  >
    <div class="flex items-center justify-between gap-4 h-9">
      <div class="font-semibold text-lg flex-1 truncate">{currentTask.name}</div>
      <div class="text-3xl font-mono font-bold text-accent">
        {formatTime(elapsedSeconds)}
      </div>
      <div class="w-8 h-8 flex items-center justify-center bg-accent text-on-accent rounded-lg transition-transform hover:scale-105">
        <Icon name="stop" />
      </div>
    </div>
  </button>
{:else}
  <div class="rounded-xl p-4 mb-6 bg-surface-raised border border-transparent">
    <div class="flex items-center justify-center gap-2 text-on-surface-muted h-9">
      <span>Select a task to start tracking</span>
    </div>
  </div>
{/if}

