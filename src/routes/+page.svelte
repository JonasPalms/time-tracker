<script lang="ts">
  import SettingsModal from "$lib/components/SettingsModal.svelte"
  import { useTracking } from "$lib/tracking.svelte"
  import {
    getTodaysTasks,
    createTask,
    formatTime,
    type Task,
  } from "$lib/tasks"
  import { onMount } from "svelte"

  let settingsOpen = $state(false)
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
    const extraSeconds =
      tracking.currentTask?.id === task.id ? tracking.elapsedSeconds : 0
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
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      </a>
      <button
        class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
        onclick={() => (settingsOpen = true)}
        aria-label="Open settings"
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
      </button>
    </div>
  </div>

  <!-- Current Tracking Display -->
  {#if tracking.isTracking && tracking.currentTask}
    <div class="bg-accent/10 border border-accent/30 rounded-xl p-4 mb-6">
      <div class="flex items-center justify-between">
        <div>
          <div class="text-sm text-on-surface-muted mb-1">Currently tracking</div>
          <div class="font-semibold text-lg">{tracking.currentTask.name}</div>
        </div>
        <div class="text-right">
          <div class="text-3xl font-mono font-bold text-accent">
            {formatTime(tracking.elapsedSeconds)}
          </div>
          <button
            class="mt-2 px-3 py-1 text-sm bg-accent text-on-accent rounded-lg hover:bg-accent-hover transition-colors"
            onclick={() => handlePlayPause(tracking.currentTask!)}
          >
            Stop
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Add Task Form -->
  <form onsubmit={handleAddTask} class="mb-6">
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

  <!-- Task List -->
  <div class="space-y-2">
    {#if isLoading}
      <div class="text-center py-8 text-on-surface-muted">Loading...</div>
    {:else if tasks.length === 0}
      <div class="text-center py-8 text-on-surface-muted bg-surface-raised rounded-xl">
        No tasks yet. Add one above to get started!
      </div>
    {:else}
      {#each tasks as task (task.id)}
        {@const isActive = tracking.currentTask?.id === task.id}
        <div
          class="flex items-center gap-3 p-3 bg-surface-raised rounded-xl transition-colors {isActive
            ? 'ring-2 ring-accent'
            : ''}"
        >
          <!-- Play/Pause Button -->
          <button
            class="w-10 h-10 flex items-center justify-center rounded-full transition-colors {isActive
              ? 'bg-accent text-on-accent'
              : 'bg-surface hover:bg-accent/20'}"
            onclick={() => handlePlayPause(task)}
            aria-label={isActive ? "Stop tracking" : "Start tracking"}
          >
            {#if isActive}
              <!-- Pause icon -->
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" />
              </svg>
            {:else}
              <!-- Play icon -->
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M8 5v14l11-7z" />
              </svg>
            {/if}
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
</main>

<!-- Settings Modal -->
<SettingsModal bind:open={settingsOpen} />
