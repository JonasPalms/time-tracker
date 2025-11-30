import { getDb } from "./db"

export interface Task {
  id: number
  name: string
  total_seconds: number
  created_at: string
}

/**
 * Get today's date in YYYY-MM-DD format (using local timezone)
 */
export function getTodayDate(): string {
  const now = new Date()
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, "0")
  const day = String(now.getDate()).padStart(2, "0")
  return `${year}-${month}-${day}`
}

/**
 * Get all tasks for today (based on created_at timestamp)
 * Uses SQLite's date() function to filter in the database
 */
export async function getTodaysTasks(): Promise<Task[]> {
  const db = await getDb()
  const today = getTodayDate()
  // Use SQLite's date() function to extract date from created_at and filter in SQL
  return await db.select<Task[]>(
    "SELECT * FROM tasks WHERE date(created_at) = ? ORDER BY created_at DESC",
    [today]
  )
}

/**
 * Create a new task (created_at is set automatically by SQLite using localtime)
 */
export async function createTask(name: string): Promise<Task> {
  const db = await getDb()

  // Insert with created_at set automatically by SQLite using localtime
  const result = await db.execute("INSERT INTO tasks (name, total_seconds) VALUES (?, 0)", [name])

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
 * Get tasks for a date range (for history view, based on created_at)
 * Uses SQLite's date() function to filter in the database
 */
export async function getTasksInRange(startDate: string, endDate: string): Promise<Task[]> {
  const db = await getDb()
  // Use SQLite's date() function to extract date from created_at and filter in SQL
  return await db.select<Task[]>(
    "SELECT * FROM tasks WHERE date(created_at) >= ? AND date(created_at) <= ? ORDER BY created_at DESC",
    [startDate, endDate]
  )
}

/**
 * Update a task's name
 */
export async function updateTaskName(taskId: number, newName: string): Promise<void> {
  const db = await getDb()
  await db.execute("UPDATE tasks SET name = ? WHERE id = ?", [newName, taskId])
}

/**
 * Delete a task
 */
export async function deleteTask(taskId: number): Promise<void> {
  const db = await getDb()
  await db.execute("DELETE FROM tasks WHERE id = ?", [taskId])
}
