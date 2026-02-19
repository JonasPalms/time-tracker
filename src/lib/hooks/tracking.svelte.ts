import { type Task, addTimeToTask } from "$lib/services/tasks";

// Module-level reactive state - shared globally
let currentTask = $state<Task | null>(null);
let elapsedSeconds = $state(0);
let startTime: number | null = null;
let intervalId: ReturnType<typeof setInterval> | null = null;

// Derived state
let isTracking = $derived(currentTask !== null);

/**
 * Start tracking time on a task
 */
function startTracking(task: Task) {
  // If already tracking something else, stop it first and save
  if (currentTask && currentTask.id !== task.id) {
    stopTrackingSync();
  }

  // If clicking the same task that's already tracking, do nothing
  if (currentTask?.id === task.id) {
    return;
  }

  currentTask = task;
  elapsedSeconds = 0;
  startTime = Date.now();

  // Update elapsed time every second
  intervalId = setInterval(() => {
    if (startTime) {
      elapsedSeconds = Math.floor((Date.now() - startTime) / 1000);
    }
  }, 1000);
}

/**
 * Stop tracking (sync version for internal use)
 */
function stopTrackingSync(): number {
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }

  const elapsed = elapsedSeconds;
  currentTask = null;
  elapsedSeconds = 0;
  startTime = null;

  return elapsed;
}

/**
 * Stop tracking and save elapsed time to database
 */
async function stopTracking(): Promise<void> {
  if (!currentTask) return;

  const taskId = currentTask.id;
  const elapsed = stopTrackingSync();

  // Save elapsed time to database
  if (elapsed > 0) {
    await addTimeToTask(taskId, elapsed);
  }
}

/**
 * Cleanup interval
 */
function cleanup() {
  if (intervalId) {
    clearInterval(intervalId);
  }
}

// Export a function that returns getters to maintain reactivity
export function useTracking() {
  return {
    get currentTask() {
      return currentTask;
    },
    get elapsedSeconds() {
      return elapsedSeconds;
    },
    get isTracking() {
      return isTracking;
    },
    startTracking,
    stopTracking,
    cleanup,
  };
}
