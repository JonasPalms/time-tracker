import { getContext, setContext } from "svelte";
import { getTimeAdjustInterval, setTimeAdjustInterval } from "./settings";

const TIME_ADJUST_KEY = Symbol("timeAdjust");

export type TimeAdjustContext = {
  get intervalMinutes(): number;
  setIntervalMinutes: (minutes: number) => Promise<void>;
};

/**
 * Create and provide the time adjust context (call in layout)
 */
export function createTimeAdjustContext() {
  let intervalMinutes = $state(5); // Default

  async function init() {
    intervalMinutes = await getTimeAdjustInterval();
  }

  async function setInterval(minutes: number) {
    intervalMinutes = minutes;
    await setTimeAdjustInterval(minutes);
  }

  const ctx: TimeAdjustContext = {
    get intervalMinutes() {
      return intervalMinutes;
    },
    setIntervalMinutes: setInterval,
  };

  setContext(TIME_ADJUST_KEY, ctx);

  return { init };
}

/**
 * Get the time adjust context (call in any child component)
 */
export function useTimeAdjust(): TimeAdjustContext {
  return getContext<TimeAdjustContext>(TIME_ADJUST_KEY);
}
