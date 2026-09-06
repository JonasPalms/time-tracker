<script lang="ts">
  import Icon from "./Icon.svelte";
  import { formatTime } from "$lib/utils/time";
  import type { Task } from "$lib/services/tasks";

  let {
    currentTask,
    elapsedSeconds,
    onStop,
  }: {
    currentTask: Task | null;
    elapsedSeconds: number;
    onStop: () => void;
  } = $props();
</script>

<div class="px-4 py-5 transition-colors">
  <div class="flex items-center justify-between gap-4">
    <button
      class="flex size-8 shrink-0 items-center justify-center bg-accent text-on-accent rounded-lg focus-visible:brightness-95"
      onclick={onStop}
      aria-label="Stop tracking"
    >
      <Icon name="stop" class="w-4 h-4" />
    </button>
    <div class="font-semibold text-lg flex-1 truncate">{currentTask?.name}</div>
    <div class="text-3xl font-mono font-bold tabular-nums text-accent">
      {formatTime(elapsedSeconds)}
    </div>
  </div>
</div>
