/**
 * Format seconds as HH:MM:SS (always includes hours with leading zero)
 */
export function formatTime(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  const pad = (n: number) => n.toString().padStart(2, "0");
  return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`;
}

/**
 * Format seconds as human readable (e.g., "2h 30m")
 */
export function formatTimeHuman(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);

  if (hours > 0 && minutes > 0) {
    return `${hours}h ${minutes}m`;
  } else if (hours > 0) {
    return `${hours}h`;
  } else if (minutes > 0) {
    return `${minutes}m`;
  } else {
    return `${totalSeconds}s`;
  }
}

/**
 * Get current date formatted as "Wed 26 Nov"
 */
export function getCurrentDate(): string {
  return new Date().toLocaleDateString("en-US", {
    weekday: "short",
    day: "numeric",
    month: "short",
  });
}

/**
 * Format a date string (YYYY-MM-DD) for display (e.g., "Wed 26 Nov")
 */
export function formatDateForDisplay(dateStr: string): string {
  const date = new Date(dateStr + "T00:00:00");
  return date.toLocaleDateString("en-US", {
    weekday: "short",
    day: "numeric",
    month: "short",
  });
}

/**
 * Convert a Date object to YYYY-MM-DD format string
 */
export function getDateString(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

/**
 * Add or subtract days from a date string (YYYY-MM-DD)
 */
export function addDays(dateStr: string, days: number): string {
  const date = new Date(dateStr + "T00:00:00");
  date.setDate(date.getDate() + days);
  return getDateString(date);
}

/**
 * Convert time string (HH:MM:SS or MM:SS) to total seconds
 * @param timeStr - Time string in format "HH:MM:SS" or "MM:SS"
 * @returns Total seconds, or null if invalid format
 */
export function parseTimeString(timeStr: string): number | null {
  const trimmed = timeStr.trim();
  if (!trimmed) return null;

  // Match HH:MM:SS or MM:SS format
  const match = trimmed.match(/^(?:(\d+):)?(\d+):(\d+)$/);
  if (!match) return null;

  const hours = match[1] ? parseInt(match[1], 10) : 0;
  const minutes = parseInt(match[2], 10);
  const seconds = parseInt(match[3], 10);

  if (minutes >= 60 || seconds >= 60 || minutes < 0 || seconds < 0 || hours < 0) {
    return null;
  }

  return hours * 3600 + minutes * 60 + seconds;
}
