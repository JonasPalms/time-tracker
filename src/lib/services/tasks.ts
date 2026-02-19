import { invoke } from "@tauri-apps/api/core";

export interface Task {
  id: number;
  name: string;
  total_seconds: number;
  created_at: string;
  note: string | null;
}

/**
 * Get today's date in YYYY-MM-DD format (using local timezone)
 */
export function getTodayDate(): string {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, "0");
  const day = String(now.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

/**
 * Get all tasks for a specific date (based on created_at timestamp)
 * @param date - Date in YYYY-MM-DD format
 */
export async function getTasksForDate(date: string): Promise<Task[]> {
  return invoke<Task[]>("get_tasks_for_date", { date });
}

/**
 * Get all tasks for today (convenience function)
 */
export async function getTodaysTasks(): Promise<Task[]> {
  return invoke<Task[]>("get_todays_tasks");
}

/**
 * Create a new task
 * @param name - Task name
 * @param date - Date in YYYY-MM-DD format. Task will be created with this date at midnight.
 * @param initialSeconds - Optional initial time in seconds (default: 0)
 */
export async function createTask(
  name: string,
  date: string,
  initialSeconds: number = 0
): Promise<Task> {
  return invoke<Task>("create_task", {
    name,
    date,
    initialSeconds: initialSeconds > 0 ? initialSeconds : null,
  });
}

/**
 * Update a task's total time
 */
export async function updateTaskTime(taskId: number, totalSeconds: number): Promise<void> {
  return invoke("update_task_time", { taskId, totalSeconds });
}

/**
 * Add seconds to a task's total time
 */
export async function addTimeToTask(taskId: number, secondsToAdd: number): Promise<void> {
  return invoke("add_time_to_task", { taskId, secondsToAdd });
}

/**
 * Adjust a task's time (can be positive or negative)
 */
export async function adjustTaskTime(taskId: number, secondsToAdjust: number): Promise<void> {
  return invoke("adjust_task_time", { taskId, secondsToAdjust });
}

/**
 * Get tasks for a date range (for history view, based on created_at)
 */
export async function getTasksInRange(startDate: string, endDate: string): Promise<Task[]> {
  return invoke<Task[]>("get_tasks_in_range", { startDate, endDate });
}

/**
 * Update a task's name
 */
export async function updateTaskName(taskId: number, newName: string): Promise<void> {
  return invoke("update_task_name", { taskId, newName });
}

/**
 * Delete a task
 */
export async function deleteTask(taskId: number): Promise<void> {
  return invoke("delete_task", { taskId });
}

/**
 * Get unique task names for autocomplete suggestions
 * Returns distinct task names ordered by most recently used
 */
export async function getUniqueTaskNames(): Promise<string[]> {
  return invoke<string[]>("get_unique_task_names");
}

/**
 * Get a single task by ID
 */
export async function getTaskById(taskId: number): Promise<Task | null> {
  return invoke<Task | null>("get_task_by_id", { taskId });
}

/**
 * Update a task's note
 */
export async function updateTaskNote(taskId: number, note: string | null): Promise<void> {
  return invoke("update_task_note", { taskId, note });
}

/**
 * Update a task's date
 * @param taskId - Task ID
 * @param newDate - Date in YYYY-MM-DD format
 */
export async function updateTaskDate(taskId: number, newDate: string): Promise<void> {
  return invoke("update_task_date", { taskId, newDate });
}
