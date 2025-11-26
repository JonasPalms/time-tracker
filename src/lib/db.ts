import Database from "@tauri-apps/plugin-sql"

let db: Database | null = null

/**
 * Get or create the database connection.
 * Uses SQLite stored in the app's data directory.
 */
export async function getDb(): Promise<Database> {
  if (!db) {
    // "sqlite:timetracker.db" creates the file in app data directory
    db = await Database.load("sqlite:timetracker.db")
    await runMigrations(db)
  }
  return db
}

/**
 * Run database migrations.
 * Creates tables if they don't exist.
 */
async function runMigrations(database: Database): Promise<void> {
  // Create tasks table
  await database.execute(`
    CREATE TABLE IF NOT EXISTS tasks (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      date TEXT NOT NULL,
      total_seconds INTEGER DEFAULT 0,
      created_at TEXT DEFAULT CURRENT_TIMESTAMP
    )
  `)

  // Create index for faster date lookups
  await database.execute(`
    CREATE INDEX IF NOT EXISTS idx_tasks_date ON tasks(date)
  `)
}
