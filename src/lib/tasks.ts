import { getDb } from "./db"

export interface Task {
  id: number
  name: string
  date: string
  total_seconds: number
  created_at: string
}

/**
 * Get today's date in YYYY-MM-DD format
 */
export function getTodayDate(): string {
  return new Date().toISOString().split("T")[0]
}

/**
 * Get all tasks for today
 */
export async function getTodaysTasks(): Promise<Task[]> {
  const db = await getDb()
  const today = getTodayDate()
  return await db.select<Task[]>("SELECT * FROM tasks WHERE date = ? ORDER BY created_at DESC", [
    today,
  ])
}

/**
 * Create a new task for today
 */
export async function createTask(name: string): Promise<Task> {
  const db = await getDb()
  const today = getTodayDate()

  const result = await db.execute(
    "INSERT INTO tasks (name, date, total_seconds) VALUES (?, ?, 0)",
    [name, today]
  )

  // Fetch the created task
  const tasks = await db.select<Task[]>("SELECT * FROM tasks WHERE id = ?", [result.lastInsertId])

  return tasks[0]
}

/**
 * Update a task's total time
 */
export async function updateTaskTime(taskId: number, totalSeconds: number): Promise<void> {
  const db = await getDb()
  await db.execute("UPDATE tasks SET total_seconds = ? WHERE id = ?", [totalSeconds, taskId])
}

/**
 * Add seconds to a task's total time
 */
export async function addTimeToTask(taskId: number, secondsToAdd: number): Promise<void> {
  const db = await getDb()
  await db.execute("UPDATE tasks SET total_seconds = total_seconds + ? WHERE id = ?", [
    secondsToAdd,
    taskId,
  ])
}

/**
 * Adjust a task's time (can be positive or negative)
 */
export async function adjustTaskTime(taskId: number, secondsToAdjust: number): Promise<void> {
  const db = await getDb()
  await db.execute("UPDATE tasks SET total_seconds = total_seconds + ? WHERE id = ?", [
    secondsToAdjust,
    taskId,
  ])
}

/**
 * Get tasks for a date range (for history view)
 */
export async function getTasksInRange(startDate: string, endDate: string): Promise<Task[]> {
  const db = await getDb()
  return await db.select<Task[]>(
    "SELECT * FROM tasks WHERE date >= ? AND date <= ? ORDER BY date DESC, created_at DESC",
    [startDate, endDate]
  )
}

/**
 * Delete a task
 */
export async function deleteTask(taskId: number): Promise<void> {
  const db = await getDb()
  await db.execute("DELETE FROM tasks WHERE id = ?", [taskId])
}
