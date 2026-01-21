const STORAGE_KEY = "sidebar-width";
const DEFAULT_WIDTH = 200;
const MIN_WIDTH = 60;
const MAX_WIDTH = 300;
const COLLAPSE_THRESHOLD = 100;

// Module-level reactive state - shared globally
let width = $state(DEFAULT_WIDTH);

function init() {
  if (typeof localStorage !== "undefined") {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = parseInt(stored, 10);
      if (!isNaN(parsed)) {
        width = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, parsed));
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

  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, String(width));
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
    get minWidth() {
      return MIN_WIDTH;
    },
    get maxWidth() {
      return MAX_WIDTH;
    },
    init,
    setWidth,
  };
}
