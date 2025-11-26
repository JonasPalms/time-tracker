<script lang="ts">
  import Icon from "$lib/components/Icon.svelte"
  import { useTracking } from "$lib/tracking.svelte"
  import { getTodaysTasks, createTask, formatTime, type Task } from "$lib/tasks"
  import { onMount } from "svelte"
  let tasks = $state<Task[]>([])
  let newTaskName = $state("")
  let isLoading = $state(true)

  const tracking = useTracking()

  // Load tasks on mount
  onMount(async () => {
    await loadTasks()
    isLoading = false
  })

  async function loadTasks() {
    tasks = await getTodaysTasks()
  }

  async function handleAddTask(e: Event) {
    e.preventDefault()
    if (!newTaskName.trim()) return

    const task = await createTask(newTaskName.trim())
    tasks = [task, ...tasks]
    newTaskName = ""
  }

  async function handlePlayPause(task: Task) {
    if (tracking.currentTask?.id === task.id) {
      // Stop tracking this task
      await tracking.stopTracking()
      await loadTasks() // Refresh to get updated totals
    } else {
      // If tracking another task, stop it first
      if (tracking.isTracking) {
        await tracking.stopTracking()
        await loadTasks()
      }
      // Start tracking this task
      tracking.startTracking(task)
    }
  }

  // Computed: total time for a task (including current elapsed if tracking)
  function getDisplayTime(task: Task): string {
    const baseSeconds = task.total_seconds
    const extraSeconds = tracking.currentTask?.id === task.id ? tracking.elapsedSeconds : 0
    return formatTime(baseSeconds + extraSeconds)
  }
</script>

<main class="container mx-auto p-6 max-w-2xl">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-black text-accent">Time Tracker</h1>
    <div class="flex items-center gap-2">
      <a
        href="/history"
        class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
        aria-label="View history"
      >
        <Icon name="clock" class="w-6 h-6" />
      </a>
      <a
        href="/settings"
        class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
        aria-label="Open settings"
      >
        <Icon name="settings" class="w-6 h-6" />
      </a>
    </div>
  </div>

  <!-- Current Tracking Display (always rendered to prevent layout shift) -->
  <div
    class="rounded-xl p-4 mb-6 transition-colors {tracking.isTracking
      ? 'bg-accent/10 border border-accent/30'
      : 'bg-surface-raised border border-transparent'}"
  >
    {#if tracking.isTracking && tracking.currentTask}
      <div class="flex items-center justify-between gap-4 h-9">
        <div class="font-semibold text-lg flex-1 truncate">{tracking.currentTask.name}</div>
        <div class="text-3xl font-mono font-bold text-accent">
          {formatTime(tracking.elapsedSeconds)}
        </div>
        <button
          class="w-8 h-8 flex items-center justify-center bg-accent text-on-accent rounded-lg hover:bg-accent-hover transition-colors"
          onclick={() => handlePlayPause(tracking.currentTask!)}
          aria-label="Stop tracking"
        >
          <Icon name="stop" />
        </button>
      </div>
    {:else}
      <div class="flex items-center justify-center gap-2 text-on-surface-muted h-9">
        <span>Select a task to start tracking</span>
      </div>
    {/if}
  </div>

  <!-- Task List -->
  <div class="space-y-2 mb-6">
    {#if isLoading}
      <div class="text-center py-8 text-on-surface-muted">Loading...</div>
    {:else if tasks.length === 0}
      <div class="text-center py-8 text-on-surface-muted bg-surface-raised rounded-xl">
        No tasks yet. Add one below to get started!
      </div>
    {:else}
      {#each tasks as task (task.id)}
        {@const isActive = tracking.currentTask?.id === task.id}
        <div class="flex items-center gap-3 p-3 bg-surface-raised rounded-xl transition-colors">
          <!-- Play/Pause Button -->
          <button
            class="w-10 h-10 flex items-center bg-surface justify-center rounded-full transition-colors {isActive
              ? 'text-accent hover:bg-accent/20'
              : 'hover:bg-accent/20'}"
            onclick={() => handlePlayPause(task)}
            aria-label={isActive ? "Stop tracking" : "Start tracking"}
          >
            <Icon name={isActive ? "pause" : "play"} />
          </button>

          <!-- Task Name -->
          <div class="flex-1 min-w-0">
            <div class="font-medium truncate">{task.name}</div>
          </div>

          <!-- Time -->
          <div class="font-mono text-lg {isActive ? 'text-accent font-bold' : 'text-on-surface-muted'}">
            {getDisplayTime(task)}
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <!-- Add Task Form -->
  <form onsubmit={handleAddTask}>
    <div class="flex gap-2">
      <input
        type="text"
        bind:value={newTaskName}
        placeholder="What are you working on?"
        class="flex-1 px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted"
      />
      <button
        type="submit"
        disabled={!newTaskName.trim()}
        class="px-6 py-3 bg-accent text-on-accent rounded-xl font-medium hover:bg-accent-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Add
      </button>
    </div>
  </form>
</main>
