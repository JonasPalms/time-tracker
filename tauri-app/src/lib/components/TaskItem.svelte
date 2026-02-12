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

<tr class="group transition-colors">
  <td class="w-14 py-2 pl-2 pr-1 align-middle rounded-l-xl group-hover:bg-surface-raised/80 group-focus-within:bg-surface-raised/80">
    <button
      class="w-10 h-10 flex items-center justify-center rounded-full transition-transform {isTracking
        ? 'text-accent'
        : ''} hover:text-accent hover:scale-120 relative"
      onclick={handlePlayPause}
      aria-label={isTracking ? "Stop tracking" : "Start tracking"}
    >
      {#if isTracking}
        <AnimatedClock
          class="w-5 h-5 group-hover:opacity-0 group-focus-within:opacity-0 transition-opacity"
        />
        <Icon
          name="stop"
          class="w-5 h-5 absolute opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 transition-opacity"
        />
      {:else}
        <Icon name="play" class="w-5 h-5" />
      {/if}
    </button>
  </td>
  <td class="py-2 pr-3 align-middle group-hover:bg-surface-raised group-focus-within:bg-surface-raised">
    <TaskItemName name={task.name} {onUpdateName} />
  </td>
  <td class="py-2 pr-2 align-middle text-right group-hover:bg-surface-raised group-focus-within:bg-surface-raised">
    <TaskItemTime totalSeconds={task.total_seconds} {isTracking} {elapsedSeconds} {onUpdateTime} />
  </td>
  <td class="w-12 py-2 pl-1 pr-2 align-middle text-right rounded-r-xl group-hover:bg-surface-raised/80 group-focus-within:bg-surface-raised/80">
    <TaskItemMenu {onEdit} {onDelete} />
  </td>
</tr>
