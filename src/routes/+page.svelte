<script lang="ts">
  import TaskItem from "$lib/components/TaskItem.svelte";
  import CurrentTracking from "$lib/components/CurrentTracking.svelte";
  import Header from "$lib/components/Header.svelte";
  import NewTaskInput from "$lib/components/NewTaskInput.svelte";
  import { useTracking } from "$lib/tracking.svelte";
  import {
    getTasksForDate,
    getTodayDate,
    createTask,
    addTimeToTask,
    updateTaskName,
    deleteTask,
    type Task,
  } from "$lib/tasks";
  import { onMount } from "svelte";
  let tasks = $state<Task[]>([]);
  let isLoading = $state(true);
  let selectedDate = $state(getTodayDate());

  const tracking = useTracking();

  // Load tasks on mount and when date changes
  onMount(async () => {
    await loadTasks();
    isLoading = false;
  });

  $effect(() => {
    // Reload tasks when selectedDate changes
    if (!isLoading) {
      loadTasks();
    }
  });

  async function loadTasks() {
    tasks = await getTasksForDate(selectedDate);
  }

  function handleDateChange(newDate: string) {
    selectedDate = newDate;
  }

  async function handleAddTask(taskName: string) {
    // Create task with the selected date
    const task = await createTask(taskName, selectedDate);
    tasks = [task, ...tasks];
  }

  // Play/stop from task item
  async function handlePlayPause(task: Task) {
    // If clicking the same task that's currently tracking, stop it
    if (tracking.currentTask?.id === task.id) {
      await tracking.stopTracking();
      await loadTasks();
    } else {
      // If tracking another task, stop it first and save
      if (tracking.currentTask) {
        await tracking.stopTracking();
        await loadTasks();
      }
      // Start tracking the new task
      tracking.startTracking(task);
    }
  }

  // Stop from the bottom bar
  async function handleStop() {
    if (!tracking.currentTask) return;
    await tracking.stopTracking();
    await loadTasks();
  }

  async function handleAdjustTime(task: Task, seconds: number) {
    // Prevent negative time - only allow adjustment if result would be >= 0
    const newTotal = task.total_seconds + seconds;
    if (newTotal < 0) {
      return; // Don't adjust if it would go negative
    }
    await addTimeToTask(task.id, seconds);
    await loadTasks(); // Refresh to get updated totals
  }

  async function handleUpdateTaskName(task: Task, newName: string) {
    if (!newName.trim()) return;
    await updateTaskName(task.id, newName);
    await loadTasks();
  }

  async function handleDelete(task: Task) {
    if (tracking.currentTask?.id === task.id) {
      await tracking.stopTracking();
    }
    await deleteTask(task.id);
    await loadTasks();
  }
</script>

<div class="h-full flex flex-col">
  <section class="shrink-0 px-6 border-b border-on-surface/10">
    <Header {selectedDate} onDateChange={handleDateChange} />
    <NewTaskInput onAddTask={handleAddTask} />
  </section>
  <section class="flex-1 overflow-y-auto py-4">
    <div class="px-6 space-y-2">
      {#if isLoading}
        <div class="text-center py-8 text-on-surface-muted">Loading...</div>
      {:else}
        {#each tasks as task (task.id)}
          {@const isTracking = tracking.currentTask?.id === task.id}
          <TaskItem
            {task}
            {isTracking}
            elapsedSeconds={tracking.elapsedSeconds}
            onPlayPause={() => handlePlayPause(task)}
            onAdjustTime={(seconds) => handleAdjustTime(task, seconds)}
            onUpdateName={(newName) => handleUpdateTaskName(task, newName)}
            onDelete={() => handleDelete(task)}
          />
        {/each}
      {/if}
    </div>
  </section>

  <section class="shrink-0 bg-surface-raised border-t border-on-surface/10 min-h-22">
    <div class="container mx-auto max-w-2xl px-6">
      {#if tracking.currentTask}
        <CurrentTracking
          currentTask={tracking.currentTask}
          elapsedSeconds={tracking.elapsedSeconds}
          onStop={handleStop}
        />
      {/if}
    </div>
  </section>
</div>
