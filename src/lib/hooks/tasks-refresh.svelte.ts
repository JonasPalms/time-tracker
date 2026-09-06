import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

let generation = $state(0);
let started = false;
let unlistenEvent: (() => void) | undefined;
let unlistenFocus: (() => void) | undefined;

function bump() {
  generation += 1;
}

export function tasksRefreshGeneration(): number {
  return generation;
}

export async function startTasksRefresh(): Promise<void> {
  if (started) return;
  started = true;

  unlistenEvent = await listen("tasks-changed", bump);
  unlistenFocus = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
    if (focused) bump();
  });
}

export function stopTasksRefresh(): void {
  unlistenEvent?.();
  unlistenFocus?.();
  unlistenEvent = undefined;
  unlistenFocus = undefined;
  started = false;
}
