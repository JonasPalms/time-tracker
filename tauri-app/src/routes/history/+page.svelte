<script lang="ts">
  import { goto } from "$app/navigation";
  import { getTasksInRange, type Task } from "$lib/services/tasks";
  import { formatTimeHuman } from "$lib/utils/time";
  import Icon from "$lib/components/Icon.svelte";
  import { slide } from "svelte/transition";

  // State
  let weekOffset = $state(0); // 0 = current week, -1 = last week, etc.
  let tasksByDate = $state<Map<string, Task[]>>(new Map());
  let isLoading = $state(true);
  let openDays = $state<Set<string>>(new Set());

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

  async function loadWeekTasks() {
    isLoading = true;
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
    openDays = new Set(grouped.keys());
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

  // Load tasks when week changes
  $effect(() => {
    weekOffset; // Dependency
    loadWeekTasks();
  });

  // Calculate total seconds for a day
  function getDayTotal(tasks: Task[]): number {
    return tasks.reduce((sum, task) => sum + task.total_seconds, 0);
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
  <!-- Header -->
  <section class="shrink-0 px-app pt-4">
    <h1 class="text-3xl font-black text-accent mb-4">History</h1>

    <!-- Week Navigation -->
    <div class="flex items-center justify-between p-4 mb-4">
      <button
        class="p-2 rounded-lg hover:bg-surface transition-colors"
        onclick={() => (weekOffset -= 1)}
        aria-label="Previous week"
      >
        <Icon name="chevron-left" class="size-6" />
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
        <Icon name="chevron-right" class="size-6" />
      </button>
    </div>
  </section>

  <!-- Days List -->
  <section class="flex-1 overflow-y-auto py-4">
    <div class="px-app">
      {#if isLoading}
        <div class="text-center py-8 text-on-surface-muted">Loading...</div>
      {:else}
        <div>
          {#each weekDates() as dateStr}
            {@const dayTasks = tasksByDate.get(dateStr) || []}
            {@const dayTotal = getDayTotal(dayTasks)}
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
                <span class="font-mono">
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
                    <button
                      class="w-full flex items-center justify-between px-2 py-3 rounded-xl transition-colors hover:bg-surface-raised text-left"
                      onclick={() => goto(`/task/${task.id}`)}
                    >
                      <div class="truncate flex-1 mr-4">{task.name}</div>
                      <div class="font-mono text-on-surface-muted">
                        {formatTimeHuman(task.total_seconds)}
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

