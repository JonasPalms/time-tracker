import {
  type Task,
  getActiveTracking,
  startTracking as startTrackingSession,
  stopTracking as stopTrackingSession,
  type ActiveTracking,
} from "$lib/services/tasks";

let currentTask = $state<Task | null>(null);
let startedAtMs = $state<number | null>(null);
let elapsedSeconds = $state(0);
let intervalId: ReturnType<typeof setInterval> | null = null;

let isTracking = $derived(currentTask !== null);

function parseLocalDateTime(value: string): number {
  const [datePart, timePart = "00:00:00"] = value.split(" ");
  const [year, month, day] = datePart.split("-").map(Number);
  const [hours, minutes, seconds] = timePart.split(":").map(Number);
  return new Date(year, month - 1, day, hours, minutes, seconds || 0).getTime();
}

function tick() {
  if (startedAtMs === null) {
    elapsedSeconds = 0;
    return;
  }
  elapsedSeconds = Math.max(0, Math.floor((Date.now() - startedAtMs) / 1000));
}

function ensureInterval() {
  if (intervalId) return;
  intervalId = setInterval(tick, 1000);
}

function clearIntervalOnly() {
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
}

function applySession(session: ActiveTracking | null) {
  if (!session) {
    clearIntervalOnly();
    currentTask = null;
    startedAtMs = null;
    elapsedSeconds = 0;
    return;
  }

  const nextStartedAt = parseLocalDateTime(session.started_at);
  currentTask = session.task;
  if (startedAtMs !== nextStartedAt) {
    startedAtMs = nextStartedAt;
  }
  tick();
  ensureInterval();
}

async function refresh() {
  applySession(await getActiveTracking());
}

async function startTracking(task: Task) {
  if (currentTask?.id === task.id) return;
  applySession(await startTrackingSession(task.id));
}

async function stopTracking(): Promise<void> {
  if (!currentTask) return;
  await stopTrackingSession();
  applySession(null);
}

function cleanup() {
  clearIntervalOnly();
}

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
    refresh,
    cleanup,
  };
}
