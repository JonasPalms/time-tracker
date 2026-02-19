const STORAGE_KEY = "sidebar-width";
const STORAGE_KEY_HIDDEN = "sidebar-hidden";
const STORAGE_KEY_PREVIOUS = "sidebar-previous-width";
const DEFAULT_WIDTH = 200;
const MIN_WIDTH = 60;
const MAX_WIDTH = 300;
const COLLAPSE_THRESHOLD = 100;

// Module-level reactive state - shared globally
let width = $state(DEFAULT_WIDTH);
let hidden = $state(false);
let previousWidth = $state(DEFAULT_WIDTH);

function init() {
  if (typeof localStorage !== "undefined") {
    const storedHidden = localStorage.getItem(STORAGE_KEY_HIDDEN);
    if (storedHidden === "true") {
      hidden = true;
    }

    const storedPrevious = localStorage.getItem(STORAGE_KEY_PREVIOUS);
    if (storedPrevious) {
      const parsed = parseInt(storedPrevious, 10);
      if (!isNaN(parsed)) {
        previousWidth = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, parsed));
      }
    }

    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = parseInt(stored, 10);
      if (!isNaN(parsed)) {
        width = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, parsed));
        if (!hidden) {
          previousWidth = width;
        }
      }
    }
  }
}

function setWidth(value: number) {
  // Snap to collapsed if below threshold
  if (value < COLLAPSE_THRESHOLD) {
    width = MIN_WIDTH;
  } else {
    width = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, value));
  }

  // Update previousWidth when not hidden
  if (!hidden) {
    previousWidth = width;
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(STORAGE_KEY_PREVIOUS, String(previousWidth));
    }
  }

  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, String(width));
  }
}

function toggle() {
  hidden = !hidden;

  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY_HIDDEN, String(hidden));
  }
}

// Export a function that returns getters to maintain reactivity
export function useSidebar() {
  return {
    get width() {
      return width;
    },
    get collapsed() {
      return width <= MIN_WIDTH;
    },
    get hidden() {
      return hidden;
    },
    get previousWidth() {
      return previousWidth;
    },
    get minWidth() {
      return MIN_WIDTH;
    },
    get maxWidth() {
      return MAX_WIDTH;
    },
    init,
    setWidth,
    toggle,
  };
}
