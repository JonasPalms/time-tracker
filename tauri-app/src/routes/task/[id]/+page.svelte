<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { parseDate, type DateValue } from "@internationalized/date";
  import ChevronLeftIcon from "@lucide/svelte/icons/chevron-left";
  import TaskItemTime from "$lib/components/TaskItemTime.svelte";
  import { DatePicker } from "$lib/components/ui/date-picker/index.js";
  import { useTracking } from "$lib/hooks/tracking.svelte";
  import { getDateFromTimestamp, createTimestampFromDate } from "$lib/utils/time";
  import {
    getTaskById,
    updateTaskName,
    updateTaskTime,
    updateTaskNote,
    updateTaskDate,
    type Task,
  } from "$lib/services/tasks";

  let task = $state<Task | null>(null);
  let isLoading = $state(true);
  let nameValue = $state("");
  let noteValue = $state("");
  let dateValue = $state<DateValue | undefined>(undefined);

  const tracking = useTracking();

  const taskId = $derived(Number($page.params.id));
  const isTracking = $derived(tracking.currentTask?.id === taskId);

  onMount(async () => {
    await loadTask();
    isLoading = false;
  });

  async function loadTask() {
    const loadedTask = await getTaskById(taskId);
    if (loadedTask) {
      task = loadedTask;
      nameValue = loadedTask.name;
      noteValue = loadedTask.note ?? "";
      dateValue = parseDate(getDateFromTimestamp(loadedTask.created_at));
    }
  }

  function handleBack() {
    if (history.length > 1) {
      history.back();
    } else {
      goto("/");
    }
  }

  async function handleNameBlur() {
    if (!task || !nameValue.trim() || nameValue === task.name) return;
    await updateTaskName(task.id, nameValue.trim());
    task.name = nameValue.trim();
  }

  function handleNameKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      (e.target as HTMLInputElement).blur();
    } else if (e.key === "Escape") {
      e.preventDefault();
      nameValue = task?.name ?? "";
      (e.target as HTMLInputElement).blur();
    }
  }

  async function handleTimeUpdate(newTotalSeconds: number) {
    if (!task) return;
    await updateTaskTime(task.id, newTotalSeconds);
    task.total_seconds = newTotalSeconds;
  }

  async function handleNoteBlur() {
    if (!task) return;
    const newNote = noteValue.trim() || null;
    if (newNote === task.note) return;
    await updateTaskNote(task.id, newNote);
    task.note = newNote;
  }

  async function handleDateChange(newDate: DateValue | undefined) {
    if (!task || !newDate) return;
    const newDateStr = newDate.toString();
    const currentDateStr = getDateFromTimestamp(task.created_at);
    if (newDateStr === currentDateStr) return;
    await updateTaskDate(task.id, newDateStr);
    task.created_at = createTimestampFromDate(newDateStr);
  }
</script>

<div class="h-full flex flex-col">
  <section class="shrink-0 px-app border-b border-on-surface/10">
    <div class="flex items-center gap-3 mb-4">
      <button
        class="p-2 rounded-lg hover:bg-surface-raised transition-colors"
        onclick={handleBack}
        aria-label="Go back"
      >
        <ChevronLeftIcon class="w-6 h-6" />
      </button>
      <h1 class="text-2xl ml-2 font-black text-accent">Edit Task</h1>
    </div>
  </section>

  <section class="flex-1 overflow-y-auto py-6">
    <div class="px-app space-y-6">
      {#if isLoading}
        <div class="text-center py-8 text-on-surface-muted">Loading...</div>
      {:else if !task}
        <div class="text-center py-8 text-on-surface-muted">Task not found</div>
      {:else}
        <div class="space-y-2">
          <label for="task-name" class="block text-sm font-medium text-on-surface-muted">
            Name
          </label>
          <input
            id="task-name"
            type="text"
            bind:value={nameValue}
            onblur={handleNameBlur}
            onkeydown={handleNameKeydown}
            class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted focus:outline-none focus:ring-1 focus:ring-on-surface/50"
          />
        </div>

        <div class="space-y-2">
          <span class="block text-sm font-medium text-on-surface-muted">Date</span>
          <DatePicker
            bind:value={dateValue}
            onValueChange={handleDateChange}
            class="bg-surface-raised rounded-xl border-none hover:bg-surface-raised/80"
          />
        </div>

        <div class="space-y-2">
          <span class="block text-sm font-medium text-on-surface-muted">Time</span>
          <div class="flex items-center px-4 py-2 bg-surface-raised rounded-xl">
            <TaskItemTime
              totalSeconds={task.total_seconds}
              {isTracking}
              elapsedSeconds={tracking.elapsedSeconds}
              onUpdateTime={handleTimeUpdate}
            />
            {#if isTracking}
              <span class="ml-4 text-sm text-accent">Currently tracking</span>
            {/if}
          </div>
        </div>

        <div class="space-y-2">
          <label for="task-note" class="block text-sm font-medium text-on-surface-muted">
            Notes
          </label>
          <textarea
            id="task-note"
            bind:value={noteValue}
            onblur={handleNoteBlur}
            rows="5"
            placeholder="Add notes about this task..."
            class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted focus:outline-none focus:ring-1 focus:ring-on-surface/50 resize-none"
          ></textarea>
        </div>
      {/if}
    </div>
  </section>
</div>
