<script lang="ts">
  import { fly } from "svelte/transition";
  import { backInOut } from "svelte/easing";
  import { goto } from "$app/navigation";
  import TaskItem from "$lib/components/TaskItem.svelte";
  import CurrentTracking from "$lib/components/CurrentTracking.svelte";
  import NewTaskInput from "$lib/components/NewTaskInput.svelte";
  import TaskDateNavigation from "$lib/components/TaskDateNavigation.svelte";
  import TaskTableHeader from "$lib/components/TaskTableHeader.svelte";
  import { useTracking } from "$lib/hooks/tracking.svelte";
  import { formatDateForDisplay, addDays } from "$lib/utils/time";
  import {
    getTasksForDate,
    getTodayDate,
    createTask,
    updateTaskName,
    updateTaskTime,
    deleteTask,
    getUniqueTaskNames,
    type Task,
  } from "$lib/services/tasks";
  import { onMount } from "svelte";

  const SORT_PREFERENCE_KEY = "tasks-sort-preference";

  type SortBy = "name" | "time" | null;
  type SortDirection = "asc" | "desc";

  interface SortPreference {
    sortBy: SortBy;
    sortDirection: SortDirection;
  }

  let tasks = $state<Task[]>([]);
  let suggestions = $state<string[]>([]);
  let isLoading = $state(true);
  let selectedDate = $state(getTodayDate());
  let sortBy = $state<SortBy>(null);
  let sortDirection = $state<SortDirection>("asc");
  let hasLoadedSortPreference = $state(false);

  const tracking = useTracking();

  // Date navigation
  const displayDate = $derived(formatDateForDisplay(selectedDate));
  const isToday = $derived(selectedDate === getTodayDate());
  const sortedTasks = $derived.by(() => {
    if (!sortBy) {
      return tasks;
    }

    const sorted = [...tasks];
    const direction = sortDirection === "asc" ? 1 : -1;

    if (sortBy === "name") {
      sorted.sort((a, b) => a.name.localeCompare(b.name, undefined, { sensitivity: "base" }) * direction);
      return sorted;
    }

    sorted.sort((a, b) => (a.total_seconds - b.total_seconds) * direction);
    return sorted;
  });

  function handlePreviousDay() {
    selectedDate = addDays(selectedDate, -1);
  }

  function handleNextDay() {
    selectedDate = addDays(selectedDate, 1);
  }

  function handleGoToToday() {
    selectedDate = getTodayDate();
  }

  function toggleSort(column: "name" | "time") {
    if (sortBy !== column) {
      sortBy = column;
      sortDirection = "asc";
      return;
    }

    if (sortDirection === "asc") {
      sortDirection = "desc";
      return;
    }

    sortBy = null;
    sortDirection = "asc";
  }

  function loadSortPreference() {
    const stored = localStorage.getItem(SORT_PREFERENCE_KEY);
    if (!stored) return;

    try {
      const parsed = JSON.parse(stored) as Partial<SortPreference>;
      if (parsed.sortBy === "name" || parsed.sortBy === "time" || parsed.sortBy === null) {
        sortBy = parsed.sortBy;
      }
      if (parsed.sortDirection === "asc" || parsed.sortDirection === "desc") {
        sortDirection = parsed.sortDirection;
      }
    } catch {
      localStorage.removeItem(SORT_PREFERENCE_KEY);
    }
  }

  function saveSortPreference() {
    const preference: SortPreference = { sortBy, sortDirection };
    localStorage.setItem(SORT_PREFERENCE_KEY, JSON.stringify(preference));
  }

  // Load tasks on mount and when date changes
  onMount(async () => {
    loadSortPreference();
    hasLoadedSortPreference = true;
    await loadTasks();
    await loadSuggestions();
    isLoading = false;
  });

  $effect(() => {
    if (hasLoadedSortPreference) {
      saveSortPreference();
    }
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

  function handleEdit(task: Task) {
    goto(`/task/${task.id}`);
  }
</script>

<div class="h-full flex flex-col">
  <section class="shrink-0 px-app pt-4 border-b border-on-surface/10">
    <TaskDateNavigation
      {displayDate}
      {isToday}
      onPreviousDay={handlePreviousDay}
      onNextDay={handleNextDay}
      onGoToToday={handleGoToToday}
    />

    <NewTaskInput onAddTask={handleAddTask} {suggestions} />
  </section>
  <section class="flex-1 overflow-y-auto py-4">
    <div class="px-app">
      {#if isLoading}
        <div class="text-center py-8 text-on-surface-muted">Loading...</div>
      {:else}
        <table class="w-full border-collapse table-fixed">
          <TaskTableHeader {sortBy} {sortDirection} onToggleSort={toggleSort} />
          <tbody>
            {#each sortedTasks as task (task.id)}
              {@const isTracking = tracking.currentTask?.id === task.id}
              <TaskItem
                {task}
                {isTracking}
                elapsedSeconds={tracking.elapsedSeconds}
                onPlayPause={() => handlePlayPause(task)}
                onUpdateName={(newName) => handleUpdateTaskName(task, newName)}
                onUpdateTime={(newTotalSeconds) => handleUpdateTaskTime(task, newTotalSeconds)}
                onEdit={() => handleEdit(task)}
                onDelete={() => handleDelete(task)}
              />
            {/each}
          </tbody>
        </table>
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
