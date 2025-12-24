<script lang="ts">
  import { fly } from "svelte/transition";
  import { backInOut } from "svelte/easing";
  import TaskItem from "$lib/components/TaskItem.svelte";
  import CurrentTracking from "$lib/components/CurrentTracking.svelte";
  import NewTaskInput from "$lib/components/NewTaskInput.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import { useTracking } from "$lib/tracking.svelte";
  import { formatDateForDisplay, addDays } from "$lib/timeUtils";
  import {
    getTasksForDate,
    getTodayDate,
    createTask,
    updateTaskName,
    updateTaskTime,
    deleteTask,
    getUniqueTaskNames,
    type Task,
  } from "$lib/tasks";
  import { onMount } from "svelte";
  let tasks = $state<Task[]>([]);
  let suggestions = $state<string[]>([]);
  let isLoading = $state(true);
  let selectedDate = $state(getTodayDate());

  const tracking = useTracking();

  // Date navigation
  const displayDate = $derived(formatDateForDisplay(selectedDate));

  function handlePreviousDay() {
    selectedDate = addDays(selectedDate, -1);
  }

  function handleNextDay() {
    selectedDate = addDays(selectedDate, 1);
  }

  // Load tasks on mount and when date changes
  onMount(async () => {
    await loadTasks();
    await loadSuggestions();
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

  async function loadSuggestions() {
    suggestions = await getUniqueTaskNames();
  }

  async function handleAddTask(taskName: string, initialSeconds?: number) {
    const task = await createTask(taskName, selectedDate, initialSeconds);
    tasks = [task, ...tasks];
    // Refresh suggestions to include the new task name
    await loadSuggestions();
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

  async function handleUpdateTaskName(task: Task, newName: string) {
    if (!newName.trim()) return;
    await updateTaskName(task.id, newName);
    await loadTasks();
  }

  async function handleUpdateTaskTime(task: Task, newTotalSeconds: number) {
    if (newTotalSeconds < 0) return;
    await updateTaskTime(task.id, newTotalSeconds);
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
  <section class="shrink-0 px-app border-b border-on-surface/10">
    <!-- Date Navigation -->
    <div class="flex items-center gap-3 mb-4">
      <button
        class="p-2 rounded-lg hover:bg-surface-raised transition-colors"
        onclick={handlePreviousDay}
        aria-label="Previous day"
      >
        <Icon name="chevron-left" class="w-6 h-6" />
      </button>
      <button
        class="p-2 rounded-lg hover:bg-surface-raised transition-colors"
        onclick={handleNextDay}
        aria-label="Next day"
      >
        <Icon name="chevron-right" class="w-6 h-6" />
      </button>
      <h1 class="text-2xl ml-2 font-black text-accent">
        {displayDate}
      </h1>
    </div>

    <NewTaskInput onAddTask={handleAddTask} {suggestions} />
  </section>
  <section class="flex-1 overflow-y-auto py-4">
    <div class="px-app space-y-2">
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
            onUpdateName={(newName) => handleUpdateTaskName(task, newName)}
            onUpdateTime={(newTotalSeconds) => handleUpdateTaskTime(task, newTotalSeconds)}
            onDelete={() => handleDelete(task)}
          />
        {/each}
      {/if}
    </div>
  </section>

  {#if tracking.currentTask}
    <section
      class="shrink-0 px-app pb-app"
      transition:fly={{ y: 100, duration: 200, easing: backInOut }}
    >
      <div class="bg-surface-raised/70 rounded-2xl border border-on-surface/10">
        <CurrentTracking
          currentTask={tracking.currentTask}
          elapsedSeconds={tracking.elapsedSeconds}
          onStop={handleStop}
        />
      </div>
    </section>
  {/if}
</div>
