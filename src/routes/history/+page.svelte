<script lang="ts">
  import AnimatedClock from "$lib/components/AnimatedClock.svelte";
  import EditTaskDialog from "$lib/components/EditTaskDialog.svelte";
  import { tasksRefreshGeneration } from "$lib/hooks/tasks-refresh.svelte";
  import { useTracking } from "$lib/hooks/tracking.svelte";
  import { getTasksInRange, type Task } from "$lib/services/tasks";
  import { formatTimeHuman } from "$lib/utils/time";
  import Icon from "$lib/components/Icon.svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import { slide } from "svelte/transition";

  const tracking = useTracking();

  // State
  let weekOffset = $state(0); // 0 = current week, -1 = last week, etc.
  let tasksByDate = $state<Map<string, Task[]>>(new Map());
  let isLoading = $state(true);
  let openDays = $state<Set<string>>(new Set());
  let editTaskId = $state<number | null>(null);
  let editDialogOpen = $state(false);

  // Get week start (Monday) and end (Sunday) for a given offset
  function getWeekRange(offset: number): { start: Date; end: Date } {
    const now = new Date();
    const dayOfWeek = now.getDay();
    // Adjust so Monday is 0
    const mondayOffset = dayOfWeek === 0 ? -6 : 1 - dayOfWeek;

    const monday = new Date(now);
    monday.setDate(now.getDate() + mondayOffset + offset * 7);
    monday.setHours(0, 0, 0, 0);

    const sunday = new Date(monday);
    sunday.setDate(monday.getDate() + 6);
    sunday.setHours(23, 59, 59, 999);

    return { start: monday, end: sunday };
  }

  function formatDate(date: Date): string {
    // Format as YYYY-MM-DD using local timezone (not UTC)
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    return `${year}-${month}-${day}`;
  }

  function formatDateDisplay(dateStr: string): string {
    const date = new Date(dateStr + "T00:00:00");
    return date.toLocaleDateString("en-US", {
      weekday: "short",
      month: "short",
      day: "numeric",
    });
  }

  function formatWeekRange(start: Date, end: Date): string {
    const startStr = start.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
    });
    const endStr = end.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
    return `${startStr} - ${endStr}`;
  }

  async function loadWeekTasks(options: { silent?: boolean } = {}) {
    if (!options.silent) isLoading = true;
    const { start, end } = getWeekRange(weekOffset);

    const tasks = await getTasksInRange(formatDate(start), formatDate(end));

    // Group tasks by date (extract from created_at)
    const grouped = new Map<string, Task[]>();
    for (const task of tasks) {
      // Extract date from SQLite datetime format (YYYY-MM-DD HH:MM:SS)
      const taskDate = task.created_at.substring(0, 10);
      const existing = grouped.get(taskDate) || [];
      grouped.set(taskDate, [...existing, task]);
    }

    tasksByDate = grouped;
    if (!options.silent) {
      openDays = new Set(grouped.keys());
    }
    isLoading = false;
  }

  function handleToggle(dateStr: string) {
    if (openDays.has(dateStr)) {
      openDays.delete(dateStr);
      openDays = new Set(openDays);
    } else {
      openDays = new Set(openDays.add(dateStr));
    }
  }

  function handleEdit(taskId: number) {
    editTaskId = taskId;
    editDialogOpen = true;
  }

  // Load tasks when week changes
  $effect(() => {
    weekOffset;
    void loadWeekTasks();
  });

  $effect(() => {
    if (tasksRefreshGeneration() === 0) return;
    void loadWeekTasks({ silent: true });
  });

  function taskDisplaySeconds(task: Task): number {
    if (tracking.currentTask?.id === task.id) {
      return task.total_seconds + tracking.elapsedSeconds;
    }
    return task.total_seconds;
  }

  function getDayTotal(tasks: Task[]): number {
    return tasks.reduce((sum, task) => sum + taskDisplaySeconds(task), 0);
  }

  // Calculate week total
  let weekTotal = $derived(() => {
    let total = 0;
    for (const tasks of tasksByDate.values()) {
      total += getDayTotal(tasks);
    }
    return total;
  });

  // Get current week range for display
  let currentRange = $derived(() => {
    const { start, end } = getWeekRange(weekOffset);
    return formatWeekRange(start, end);
  });

  // Get all dates in the week for display (even if no tasks)
  let weekDates = $derived(() => {
    const { start } = getWeekRange(weekOffset);
    const dates: string[] = [];
    for (let i = 0; i < 7; i++) {
      const date = new Date(start);
      date.setDate(start.getDate() + i);
      dates.push(formatDate(date));
    }
    return dates;
  });
</script>

<div class="h-full flex flex-col">
  <PageHeader>
    <div class="flex items-center gap-3">
      <button
        class="p-2 rounded-lg bg-surface-raised hover:bg-surface-hover transition-colors"
        onclick={() => (weekOffset -= 1)}
        aria-label="Previous week"
      >
        <Icon name="chevron-left" class="w-5 h-5" />
      </button>
      <button
        class="p-2 rounded-lg bg-surface-raised transition-colors enabled:hover:bg-surface-hover disabled:cursor-default disabled:opacity-30"
        onclick={() => (weekOffset += 1)}
        disabled={weekOffset >= 0}
        aria-label="Next week"
      >
        <Icon name="chevron-right" class="w-5 h-5" />
      </button>
      <h1 class="text-2xl ml-2 font-black text-pretty text-accent">
        {currentRange()}
      </h1>
      <span class="ml-auto text-2xl font-black text-on-surface">
        {formatTimeHuman(weekTotal())}
      </span>
    </div>
  </PageHeader>

  <!-- Days List -->
  <section class="flex-1 overflow-y-auto pt-2">
    <div class="px-app">
      {#if isLoading}
        <div class="text-center py-8 text-on-surface-muted">Loading…</div>
      {:else}
        <div>
          {#each weekDates() as dateStr}
            {@const dayTasks = tasksByDate.get(dateStr) || []}
            {@const dayTotal = getDayTotal(dayTasks)}
            {@const dayHasActive = dayTasks.some((task) => task.id === tracking.currentTask?.id)}
            {@const isToday = dateStr === new Date().toISOString().split("T")[0]}

            <div class="py-2">
              <button
                class="w-full flex items-center justify-between px-3 py-3 rounded-lg bg-surface-elevated"
                onclick={() => handleToggle(dateStr)}
                aria-expanded={openDays.has(dateStr)}
                aria-controls="day-{dateStr}"
              >
                <span class="font-medium {isToday ? 'text-accent' : ''}">
                  {formatDateDisplay(dateStr)}
                </span>
                <span class="flex items-center gap-2 font-mono {dayHasActive ? 'text-accent' : ''}">
                  {#if dayHasActive && !openDays.has(dateStr)}
                    <AnimatedClock class="w-4 h-4" />
                  {/if}
                  {dayTotal > 0 ? formatTimeHuman(dayTotal) : "-"}
                </span>
              </button>

              {#if dayTasks.length > 0 && openDays.has(dateStr)}
                <div
                  id="day-{dateStr}"
                  class="space-y-0.5 mt-2"
                  role="region"
                  transition:slide={{ duration: 200 }}
                >
                  {#each dayTasks as task}
                    {@const isActive = tracking.currentTask?.id === task.id}
                    <button
                      class="w-full flex items-center justify-between px-2 py-3 rounded-xl transition-colors hover:bg-surface-raised text-left {isActive
                        ? 'text-accent'
                        : ''}"
                      onclick={() => handleEdit(task.id)}
                      title={isActive ? "Currently tracking" : undefined}
                    >
                      <div class="min-w-0 flex-1 mr-4 flex items-center gap-2">
                        {#if isActive}
                          <AnimatedClock class="w-4 h-4 shrink-0" />
                        {/if}
                        <div class="truncate">{task.name}</div>
                      </div>
                      <div class="font-mono {isActive ? 'text-accent' : 'text-on-surface-muted'}">
                        {formatTimeHuman(taskDisplaySeconds(task))}
                      </div>
                    </button>
                  {/each}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </section>
</div>

<EditTaskDialog bind:open={editDialogOpen} taskId={editTaskId} onTaskChange={loadWeekTasks} />
