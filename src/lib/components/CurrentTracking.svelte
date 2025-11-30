<script lang="ts">
  import Icon from "./Icon.svelte"
  import { formatTime } from "$lib/timeUtils"
  import type { Task } from "$lib/tasks"

  let {
    currentTask,
    elapsedSeconds,
    onStop,
  }: {
    currentTask: Task | null
    elapsedSeconds: number
    onStop: () => void
  } = $props()
</script>

<div class="px-4 py-5 transition-colors">
    <div class="flex items-center justify-between gap-4">
      <div class="font-semibold text-lg flex-1 truncate">{currentTask?.name}</div>
      <div class="text-3xl font-mono font-bold text-accent">
        {formatTime(elapsedSeconds)}
      </div>
      <button
        class="w-10 h-10 flex items-center justify-center bg-accent text-on-accent rounded-lg transition-transform hover:scale-105"
        onclick={onStop}
        aria-label="Stop tracking"
      >
        <Icon name="stop" />
      </button>
    </div>  
</div>