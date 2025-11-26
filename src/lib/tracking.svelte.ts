import { getContext, setContext, onDestroy } from "svelte"
import { type Task, addTimeToTask } from "./tasks"

const TRACKING_KEY = Symbol("tracking")

export type TrackingContext = {
  get currentTask(): Task | null
  get elapsedSeconds(): number
  get isTracking(): boolean
  startTracking: (task: Task) => void
  stopTracking: () => Promise<void>
}

/**
 * Create and provide the tracking context (call in layout)
 */
export function createTrackingContext() {
  let currentTask = $state<Task | null>(null)
  let elapsedSeconds = $state(0)
  let startTime: number | null = null
  let intervalId: ReturnType<typeof setInterval> | null = null

  // Derived state
  let isTracking = $derived(currentTask !== null)

  /**
   * Start tracking time on a task
   */
  function startTracking(task: Task) {
    // If already tracking something else, stop it first
    if (currentTask && currentTask.id !== task.id) {
      stopTrackingSync()
    }

    // If clicking the same task that's already tracking, do nothing
    if (currentTask?.id === task.id) {
      return
    }

    currentTask = task
    elapsedSeconds = 0
    startTime = Date.now()

    // Update elapsed time every second
    intervalId = setInterval(() => {
      if (startTime) {
        elapsedSeconds = Math.floor((Date.now() - startTime) / 1000)
      }
    }, 1000)
  }

  /**
   * Stop tracking (sync version for internal use)
   */
  function stopTrackingSync(): number {
    if (intervalId) {
      clearInterval(intervalId)
      intervalId = null
    }

    const elapsed = elapsedSeconds
    currentTask = null
    elapsedSeconds = 0
    startTime = null

    return elapsed
  }

  /**
   * Stop tracking and save elapsed time to database
   */
  async function stopTracking(): Promise<void> {
    if (!currentTask) return

    const taskId = currentTask.id
    const elapsed = stopTrackingSync()

    // Save elapsed time to database
    if (elapsed > 0) {
      await addTimeToTask(taskId, elapsed)
    }
  }

  /**
   * Cleanup on destroy
   */
  function cleanup() {
    if (intervalId) {
      clearInterval(intervalId)
    }
  }

  const ctx: TrackingContext = {
    get currentTask() {
      return currentTask
    },
    get elapsedSeconds() {
      return elapsedSeconds
    },
    get isTracking() {
      return isTracking
    },
    startTracking,
    stopTracking,
  }

  setContext(TRACKING_KEY, ctx)

  return { cleanup, stopTracking }
}

/**
 * Get the tracking context (call in any child component)
 */
export function useTracking(): TrackingContext {
  return getContext<TrackingContext>(TRACKING_KEY)
}
