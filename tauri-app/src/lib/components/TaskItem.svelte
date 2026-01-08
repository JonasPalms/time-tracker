<script lang="ts">
  import Icon from "./Icon.svelte";
  import AnimatedClock from "./AnimatedClock.svelte";
  import { type Task } from "$lib/services/tasks";
  import TaskItemMenu from "./TaskItemMenu.svelte";
  import TaskItemName from "./TaskItemName.svelte";
  import TaskItemTime from "./TaskItemTime.svelte";

  let {
    task,
    isTracking,
    elapsedSeconds = 0,
    onPlayPause,
    onUpdateName,
    onUpdateTime,
    onEdit,
    onDelete,
  }: {
    task: Task;
    isTracking: boolean;
    elapsedSeconds?: number;
    onPlayPause: () => void;
    onUpdateName?: (newName: string) => void;
    onUpdateTime?: (newTotalSeconds: number) => void;
    onEdit?: () => void;
    onDelete: () => void;
  } = $props();

  function handlePlayPause(e: MouseEvent) {
    e.stopPropagation();
    onPlayPause();
  }
</script>

<div
  class="group flex items-center gap-2 px-2 py-3 rounded-xl transition-colors hover:bg-surface-raised focus-within:bg-surface-raised"
>
  <button
    class="w-12 h-12 flex items-center justify-center rounded-full transition-transform {isTracking
      ? 'text-accent'
      : ''} hover:text-accent hover:scale-120 relative"
    onclick={handlePlayPause}
    aria-label={isTracking ? "Stop tracking" : "Start tracking"}
  >
    {#if isTracking}
      <AnimatedClock
        class="w-6 h-6 group-hover:opacity-0 group-focus:opacity-0 transition-opacity"
      />
      <Icon
        name="stop"
        class="w-6 h-6 absolute opacity-0 group-hover:opacity-100 group-focus:opacity-100 transition-opacity"
      />
    {:else}
      <Icon name="play" class="w-6 h-6" />
    {/if}
  </button>
  <TaskItemName name={task.name} {onUpdateName} />
  <TaskItemTime totalSeconds={task.total_seconds} {isTracking} {elapsedSeconds} {onUpdateTime} />
  <TaskItemMenu {onEdit} {onDelete} />
</div>
