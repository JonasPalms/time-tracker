<script lang="ts">
  import TaskItem from "$lib/components/TaskItem.svelte"
  import CurrentTracking from "$lib/components/CurrentTracking.svelte"
  import Header from "$lib/components/Header.svelte"
  import { useTracking } from "$lib/tracking.svelte"
  import { getTodaysTasks, createTask, addTimeToTask, updateTaskName, deleteTask, type Task } from "$lib/tasks"
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

  // Play/stop from task item
  async function handlePlayPause(task: Task) {
    // If clicking the same task that's currently tracking, stop it
    if (tracking.currentTask?.id === task.id) {
      await tracking.stopTracking()
      await loadTasks()
    } else {
      // If tracking another task, stop it first and save
      if (tracking.currentTask) {
        await tracking.stopTracking()
        await loadTasks()
      }
      // Start tracking the new task
      tracking.startTracking(task)
    }
  }

  // Stop from the bottom bar
  async function handleStop() {
    if (!tracking.currentTask) return
    await tracking.stopTracking()
    await loadTasks()
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

  async function handleUpdateTaskName(task: Task, newName: string) {
    if (!newName.trim()) return
    await updateTaskName(task.id, newName)
    await loadTasks()
  }

  async function handleDelete(task: Task) {
    if (tracking.currentTask?.id === task.id) {
      await tracking.stopTracking()
    }
    await deleteTask(task.id)
    await loadTasks()
  }
</script>

<div class="h-full flex flex-col">
  <div class="shrink-0 px-6 border-b border-on-surface/10">
    <Header />
    <form onsubmit={handleAddTask} class="mb-4">
      <input
        type="text"
        bind:value={newTaskName}
        placeholder="What are you working on?"
        class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted"
      />
    </form>
  </div>
  <div class="flex-1 overflow-y-auto py-4">
    <div class="px-6 space-y-2">
      {#if isLoading}
        <div class="text-center py-8 text-on-surface-muted">Loading...</div>
      {:else}
        {#each tasks as task (task.id)}
          {@const isTracking = tracking.currentTask?.id === task.id}
          <TaskItem
            {task}
            isTracking={isTracking}
            elapsedSeconds={tracking.elapsedSeconds}
            onPlayPause={() => handlePlayPause(task)}
            onAdjustTime={(seconds) => handleAdjustTime(task, seconds)}
            onUpdateName={(newName) => handleUpdateTaskName(task, newName)}
            onDelete={() => handleDelete(task)}
          />
        {/each}
      {/if}
    </div>
  </div>

  <div class="shrink-0 bg-surface-raised border-t border-on-surface/10 min-h-20">
    <div class="container mx-auto max-w-2xl px-6">
      {#if tracking.currentTask}
      <CurrentTracking
        currentTask={tracking.currentTask}
        elapsedSeconds={tracking.elapsedSeconds}
        onStop={handleStop}
      />
      {/if}
    </div>
  </div>
</div>
