import Database from "@tauri-apps/plugin-sql";

let db: Database | null = null;

/**
 * Get the database name from environment variable or use default.
 * Defaults to "timetracker-dev.db" for development.
 */
function getDbName(): string {
  // Use environment variable if set, otherwise default to dev database
  const dbName = import.meta.env.VITE_DB_NAME || "timetracker-dev.db";
  return `sqlite:${dbName}`;
}

/**
 * Get or create the database connection.
 * Uses SQLite stored in the app's data directory.
 */
export async function getDb(): Promise<Database> {
  if (!db) {
    const dbPath = getDbName();
    db = await Database.load(dbPath);
    await runMigrations(db);
  }

  return db;
}

/**
 * Run database migrations.
 * Creates tables if they don't exist.
 */
async function runMigrations(database: Database): Promise<void> {
  // Create tasks table
  // Use localtime for created_at to match user's timezone
  await database.execute(`
    CREATE TABLE IF NOT EXISTS tasks (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      total_seconds INTEGER DEFAULT 0,
      created_at TEXT DEFAULT (datetime('now', 'localtime'))
    )
  `);

  await database.execute(`
    CREATE TABLE IF NOT EXISTS favourites (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      duration_seconds INTEGER NOT NULL,
      created_at TEXT DEFAULT (datetime('now', 'localtime'))
    )
  `);

  // Create index for faster date lookups on created_at
  await database.execute(`
    CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at)
  `);
}
