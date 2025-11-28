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
    if (!newName.trim()) return // Don't allow empty names
    await updateTaskName(task.id, newName)
    await loadTasks() // Refresh to get updated task names
  }

  async function handleDelete(task: Task) {
    // Stop tracking if this task is currently being tracked
    if (tracking.currentTask?.id === task.id) {
      await tracking.stopTracking()
    }
    await deleteTask(task.id)
    await loadTasks() // Refresh to remove deleted task
  }
</script>


  <Header />
  <form onsubmit={handleAddTask} class="mb-6">
    <input
      type="text"
      bind:value={newTaskName}
      placeholder="What are you working on?"
      class="w-full px-4 py-3 bg-surface-raised rounded-xl border-none placeholder:text-on-surface-muted"
    />
  </form>
  <div class="space-y-2 {tracking.currentTask ? 'mb-20' : ''}">
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

{#if tracking.currentTask}
  <div class="fixed bottom-0 left-0 right-0 z-50 pointer-events-none">
    <div class="container mx-auto max-w-2xl pointer-events-auto">
      <CurrentTracking
        currentTask={tracking.currentTask}
        elapsedSeconds={tracking.elapsedSeconds}
        onStop={handleStop}
      />
    </div>
  </div>
{/if}
