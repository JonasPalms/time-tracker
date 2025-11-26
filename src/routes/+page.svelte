<script lang="ts">
  import Icon from "$lib/components/Icon.svelte"
  import TaskItem from "$lib/components/TaskItem.svelte"
  import CurrentTracking from "$lib/components/CurrentTracking.svelte"
  import { useTracking } from "$lib/tracking.svelte"
  import { getTodaysTasks, createTask, addTimeToTask, type Task } from "$lib/tasks"
  import { onMount } from "svelte"
  let tasks = $state<Task[]>([])
  let newTaskName = $state("")
  let isLoading = $state(true)
  let selectedTask = $state<Task | null>(null)

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

  // Select a task (doesn't start tracking)
  function handleSelectTask(task: Task) {
    selectedTask = task
  }

  // Play/pause from task item or bottom bar
  async function handlePlayPause(task: Task) {
    // Select the task first
    selectedTask = task

    if (tracking.currentTask?.id === task.id) {
      // Stop tracking
      await tracking.stopTracking()
      await loadTasks()
    } else {
      // If tracking another task, stop it first
      if (tracking.isTracking) {
        await tracking.stopTracking()
        await loadTasks()
      }
      // Start tracking the task
      tracking.startTracking(task)
    }
  }

  // Play/pause from the bottom bar
  async function handlePlayPauseFromBottom() {
    // Use currentTask if tracking, otherwise use selectedTask
    const task = tracking.currentTask || selectedTask
    if (!task) return
    await handlePlayPause(task)
  }

  async function handleAdjustTime(task: Task, seconds: number) {
    // Prevent negative time - only allow adjustment if result would be >= 0
    const newTotal = task.total_seconds + seconds
    if (newTotal < 0) {
      return // Don't adjust if it would go negative
    }
    await addTimeToTask(task.id, seconds)
    await loadTasks() // Refresh to get updated totals
  }


  function getCurrentDate(): string {
    return new Date().toLocaleDateString("en-US", {
      weekday: "short",
      day: "numeric",
      month: "short",
    })
  }
</script>

<main class="container mx-auto p-6 max-w-2xl">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div>
      <h1 class="text-2xl font-black text-accent">{getCurrentDate()}</h1>
    </div>
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

  <!-- Add Task Form -->
  <form onsubmit={handleAddTask} class="mb-6">
    <input
      type="text"
      bind:value={newTaskName}
      placeholder="What are you working on?"
      class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted"
    />
  </form>

  <!-- Task List -->
  <div class="space-y-2 {selectedTask ? 'mb-20' : ''}">
    {#if isLoading}
      <div class="text-center py-8 text-on-surface-muted">Loading...</div>
    {:else if tasks.length === 0}
      <div class="text-center py-8 text-on-surface-muted bg-surface-raised rounded-xl">
        No tasks yet. Add one above to get started!
      </div>
    {:else}
      {#each tasks as task (task.id)}
        {@const isSelected = selectedTask?.id === task.id}
        {@const isTracking = tracking.currentTask?.id === task.id}
        <TaskItem
          {task}
          isSelected={isSelected}
          isTracking={isTracking}
          elapsedSeconds={tracking.elapsedSeconds}
          onSelect={() => handleSelectTask(task)}
          onPlayPause={() => handlePlayPause(task)}
          onAdjustTime={(seconds) => handleAdjustTime(task, seconds)}
        />
      {/each}
    {/if}
  </div>
</main>

<!-- Fixed Current Tracking Display at Bottom -->
{#if selectedTask}
  <div class="fixed bottom-0 left-0 right-0 z-50 pointer-events-none">
    <div class="container mx-auto max-w-2xl pointer-events-auto">
      <CurrentTracking
        isTracking={tracking.isTracking}
        selectedTask={selectedTask}
        currentTask={tracking.currentTask}
        elapsedSeconds={tracking.elapsedSeconds}
        onPlayPause={handlePlayPauseFromBottom}
      />
    </div>
  </div>
{/if}
