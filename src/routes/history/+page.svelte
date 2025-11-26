<script lang="ts">
  import { getTasksInRange, formatTimeHuman, type Task } from "$lib/tasks"

  // State
  let weekOffset = $state(0) // 0 = current week, -1 = last week, etc.
  let tasksByDate = $state<Map<string, Task[]>>(new Map())
  let isLoading = $state(true)

  // Get week start (Monday) and end (Sunday) for a given offset
  function getWeekRange(offset: number): { start: Date; end: Date } {
    const now = new Date()
    const dayOfWeek = now.getDay()
    // Adjust so Monday is 0
    const mondayOffset = dayOfWeek === 0 ? -6 : 1 - dayOfWeek

    const monday = new Date(now)
    monday.setDate(now.getDate() + mondayOffset + offset * 7)
    monday.setHours(0, 0, 0, 0)

    const sunday = new Date(monday)
    sunday.setDate(monday.getDate() + 6)
    sunday.setHours(23, 59, 59, 999)

    return { start: monday, end: sunday }
  }

  function formatDate(date: Date): string {
    return date.toISOString().split("T")[0]
  }

  function formatDateDisplay(dateStr: string): string {
    const date = new Date(dateStr + "T00:00:00")
    return date.toLocaleDateString("en-US", {
      weekday: "short",
      month: "short",
      day: "numeric",
    })
  }

  function formatWeekRange(start: Date, end: Date): string {
    const startStr = start.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
    })
    const endStr = end.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: "numeric",
    })
    return `${startStr} - ${endStr}`
  }

  async function loadWeekTasks() {
    isLoading = true
    const { start, end } = getWeekRange(weekOffset)

    const tasks = await getTasksInRange(formatDate(start), formatDate(end))

    // Group tasks by date
    const grouped = new Map<string, Task[]>()
    for (const task of tasks) {
      const existing = grouped.get(task.date) || []
      grouped.set(task.date, [...existing, task])
    }

    tasksByDate = grouped
    isLoading = false
  }

  // Load tasks when week changes
  $effect(() => {
    weekOffset // Dependency
    loadWeekTasks()
  })

  // Calculate total seconds for a day
  function getDayTotal(tasks: Task[]): number {
    return tasks.reduce((sum, task) => sum + task.total_seconds, 0)
  }

  // Calculate week total
  let weekTotal = $derived(() => {
    let total = 0
    for (const tasks of tasksByDate.values()) {
      total += getDayTotal(tasks)
    }
    return total
  })

  // Get current week range for display
  let currentRange = $derived(() => {
    const { start, end } = getWeekRange(weekOffset)
    return formatWeekRange(start, end)
  })

  // Get all dates in the week for display (even if no tasks)
  let weekDates = $derived(() => {
    const { start } = getWeekRange(weekOffset)
    const dates: string[] = []
    for (let i = 0; i < 7; i++) {
      const date = new Date(start)
      date.setDate(start.getDate() + i)
      dates.push(formatDate(date))
    }
    return dates
  })
</script>

<main class="container mx-auto p-6 max-w-2xl">
  <!-- Header -->
  <div class="flex items-center gap-4 mb-6">
    <a
      href="/"
      class="p-2 rounded-lg bg-surface-raised hover:bg-accent/20 transition-colors"
      aria-label="Back to tracker"
    >
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </a>
    <h1 class="text-2xl font-black text-accent">History</h1>
  </div>

  <!-- Week Navigation -->
  <div class="flex items-center justify-between bg-surface-raised rounded-xl p-4 mb-6">
    <button
      class="p-2 rounded-lg hover:bg-surface transition-colors"
      onclick={() => (weekOffset -= 1)}
      aria-label="Previous week"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </button>

    <div class="text-center">
      <div class="font-semibold">{currentRange()}</div>
      <div class="text-sm text-on-surface-muted">
        Total: {formatTimeHuman(weekTotal())}
      </div>
    </div>

    <button
      class="p-2 rounded-lg hover:bg-surface transition-colors disabled:opacity-30"
      onclick={() => (weekOffset += 1)}
      disabled={weekOffset >= 0}
      aria-label="Next week"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>
  </div>

  <!-- Days List -->
  {#if isLoading}
    <div class="text-center py-8 text-on-surface-muted">Loading...</div>
  {:else}
    <div class="space-y-4">
      {#each weekDates() as dateStr}
        {@const dayTasks = tasksByDate.get(dateStr) || []}
        {@const dayTotal = getDayTotal(dayTasks)}
        {@const isToday = dateStr === new Date().toISOString().split("T")[0]}

        <div class="bg-surface-raised rounded-xl overflow-hidden">
          <!-- Day Header -->
          <div
            class="flex items-center justify-between px-4 py-3 border-b border-on-surface/10 {isToday
              ? 'bg-accent/10'
              : ''}"
          >
            <div class="font-medium {isToday ? 'text-accent' : ''}">
              {formatDateDisplay(dateStr)}
              {#if isToday}
                <span class="text-xs ml-2 text-accent">(Today)</span>
              {/if}
            </div>
            <div class="font-mono text-on-surface-muted">
              {dayTotal > 0 ? formatTimeHuman(dayTotal) : "-"}
            </div>
          </div>

          <!-- Tasks for this day -->
          {#if dayTasks.length > 0}
            <div class="divide-y divide-on-surface/5">
              {#each dayTasks as task}
                <div class="flex items-center justify-between px-4 py-2">
                  <div class="text-sm truncate flex-1 mr-4">{task.name}</div>
                  <div class="font-mono text-sm text-on-surface-muted">
                    {formatTimeHuman(task.total_seconds)}
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="px-4 py-3 text-sm text-on-surface-muted">No tasks</div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</main>

