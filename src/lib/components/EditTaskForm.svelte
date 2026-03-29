<script lang="ts">
  import { parseDate, type DateValue } from "@internationalized/date";
  import { Button } from "$lib/components/ui/button";
  import { DatePicker } from "$lib/components/ui/date-picker/index.js";
  import { Input } from "$lib/components/ui/input";
  import { useTracking } from "$lib/hooks/tracking.svelte";
  import "$lib/components/TaskItemTime.css";
  import {
    formatTime,
    parseTimeString,
    getDateFromTimestamp,
    createTimestampFromDate,
  } from "$lib/utils/time";
  import {
    getTaskById,
    updateTaskName,
    updateTaskTime,
    updateTaskNote,
    updateTaskDate,
    type Task,
  } from "$lib/services/tasks";

  let {
    taskId,
    initialTask = null,
    onCancel,
    onTaskChange,
    onSaveSuccess,
  }: {
    taskId: number;
    initialTask?: Task | null;
    onCancel?: () => void;
    onTaskChange?: () => void | Promise<void>;
    onSaveSuccess?: () => void;
  } = $props();

  let task = $state<Task | null>(null);
  let isLoading = $state(true);
  let isSaving = $state(false);
  let initializedTaskId = $state<number | null>(null);
  let nameValue = $state("");
  let noteValue = $state("");
  let dateValue = $state<DateValue | undefined>(undefined);
  let timeValue = $state("");

  const tracking = useTracking();

  const isTracking = $derived(tracking.currentTask?.id === taskId);

  $effect(() => {
    taskId;
    initialTask;

    if (initialTask && initialTask.id === taskId && initializedTaskId !== taskId) {
      applyTask(initialTask);
      initializedTaskId = taskId;
      return;
    }

    if (initializedTaskId === taskId) return;

    void loadTask(taskId);
  });

  async function loadTask(id: number) {
    isLoading = true;
    task = null;

    const loadedTask = await getTaskById(id);
    if (taskId !== id) return;

    if (loadedTask) {
      applyTask(loadedTask);
      initializedTaskId = id;
    }

    isLoading = false;
  }

  function applyTask(source: Task) {
    task = { ...source };
    nameValue = source.name;
    noteValue = source.note ?? "";
    dateValue = parseDate(getDateFromTimestamp(source.created_at));
    timeValue = formatTime(source.total_seconds);
    isLoading = false;
  }

  async function notifyTaskChange() {
    await onTaskChange?.();
  }

  function handleNameKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
    } else if (e.key === "Escape") {
      e.preventDefault();
      nameValue = task?.name ?? "";
    }
  }

  const parsedTimeValue = $derived(parseTimeString(timeValue));
  const normalizedName = $derived(nameValue.trim());
  const normalizedNote = $derived(noteValue.trim() || null);
  const hasChanges = $derived.by(() => {
    if (!task || !dateValue) return false;

    return (
      normalizedName !== task.name ||
      normalizedNote !== task.note ||
      dateValue.toString() !== getDateFromTimestamp(task.created_at) ||
      (!isTracking && parsedTimeValue !== null && parsedTimeValue !== task.total_seconds)
    );
  });

  const canSave = $derived(
    !isLoading &&
      !isSaving &&
      task !== null &&
      normalizedName.length > 0 &&
      dateValue !== undefined &&
      hasChanges &&
      (isTracking || parsedTimeValue !== null)
  );

  async function handleSave() {
    if (!task || !dateValue || !canSave) return;

    isSaving = true;

    try {
      if (normalizedName !== task.name) {
        await updateTaskName(task.id, normalizedName);
        task.name = normalizedName;
      }

      const nextDate = dateValue.toString();
      if (nextDate !== getDateFromTimestamp(task.created_at)) {
        await updateTaskDate(task.id, nextDate);
        task.created_at = createTimestampFromDate(nextDate);
      }

      if (!isTracking && parsedTimeValue !== null && parsedTimeValue !== task.total_seconds) {
        await updateTaskTime(task.id, parsedTimeValue);
        task.total_seconds = parsedTimeValue;
      }

      if (normalizedNote !== task.note) {
        await updateTaskNote(task.id, normalizedNote);
        task.note = normalizedNote;
      }

      noteValue = task.note ?? "";
      timeValue = formatTime(task.total_seconds);
      await notifyTaskChange();
      onSaveSuccess?.();
    } finally {
      isSaving = false;
    }
  }

  function handleCancel() {
    onCancel?.();
  }
</script>

<form
  class="space-y-6"
  onsubmit={(e) => {
    e.preventDefault();
    void handleSave();
  }}
>
  {#if isLoading}
    <div class="text-center py-8 text-on-surface-muted">Loading...</div>
  {:else if !task}
    <div class="text-center py-8 text-on-surface-muted">Task not found</div>
  {:else}
    <div class="space-y-2">
      <label for="task-name" class="block text-sm font-medium text-on-surface-muted">Name</label>
      <input
        id="task-name"
        type="text"
        bind:value={nameValue}
        onkeydown={handleNameKeydown}
        class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted focus:outline-none focus:ring-1 focus:ring-on-surface/50"
      />
    </div>

    <div class="grid gap-4 sm:grid-cols-2">
      <div class="space-y-2">
        <span class="block text-sm font-medium text-on-surface-muted">Date</span>
        <DatePicker
          bind:value={dateValue}
          class="h-auto justify-start rounded-xl border-none bg-surface-raised px-4 py-3 text-xl font-mono text-on-surface shadow-none hover:bg-surface-raised/80"
        />
      </div>

      <div class="space-y-2">
        <span class="block text-sm font-medium text-on-surface-muted">Time</span>
        <div class="space-y-2 rounded-xl bg-surface-raised px-4 py-3">
          <Input
            type="time"
            step="1"
            bind:value={timeValue}
            disabled={isTracking}
            readonly={isTracking}
            class="time-input-foreground h-auto w-33 border-none bg-transparent! px-0 py-0 font-mono text-xl shadow-none focus-visible:ring-0 disabled:opacity-100 md:text-xl"
            aria-label={isTracking ? "Time is being tracked" : "Time"}
          />
          {#if isTracking}
            <span class="text-sm text-accent">Currently tracking</span>
          {:else if parsedTimeValue === null}
            <span class="text-sm text-destructive">Enter time as HH:MM:SS</span>
          {/if}
        </div>
      </div>
    </div>

    <div class="space-y-2">
      <label for="task-note" class="block text-sm font-medium text-on-surface-muted">Notes</label>
      <textarea
        id="task-note"
        bind:value={noteValue}
        rows="5"
        placeholder="Add notes about this task..."
        class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted focus:outline-none focus:ring-1 focus:ring-on-surface/50 resize-none"
      ></textarea>
    </div>

    <div class="flex justify-end gap-3 pt-2">
      <Button variant="outline" onclick={handleCancel}>Cancel</Button>
      <Button type="submit" disabled={!canSave} class="min-w-28 bg-accent hover:bg-accent/90">
        {isSaving ? "Saving..." : "Save"}
      </Button>
    </div>
  {/if}
</form>
