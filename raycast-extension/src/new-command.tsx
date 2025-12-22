import { List, Icon, showToast, Toast, environment } from "@raycast/api";
import { useState, useEffect } from "react";
import { homedir } from "os";
import path from "path";
import Database from "better-sqlite3";

type Task = {
  id: number;
  name: string;
  total_seconds: number;
  created_at: string;
};

export default function Command() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function fetchTasks() {
      let db: Database.Database | null = null;
      try {
        const dbName = environment.isDevelopment ? "timetracker-dev.db" : "timetracker.db";
        const dbPath = path.join(
          homedir(),
          `Library/Application Support/com.jonaspalmsorensen.time-tracker/${dbName}`
        );

        // Open in Read-Only mode to avoid locks and corruption
        db = new Database(dbPath, { readonly: true, fileMustExist: true });

        const query = `
          SELECT id, name, total_seconds, created_at
          FROM tasks
          WHERE date(created_at) = date('now', 'localtime')
          ORDER BY created_at DESC;
        `;

        const stmt = db.prepare(query);
        const result = stmt.all() as Task[];
        setTasks(result);

      } catch (error) {
        console.error("Failed to fetch tasks:", error);
        await showToast({
          style: Toast.Style.Failure,
          title: "Failed to fetch tasks",
          message: String(error),
        });
      } finally {
        if (db) {
          db.close();
        }
        setIsLoading(false);
      }
    }

    fetchTasks();
  }, []);

  return (
    <List isLoading={isLoading} searchBarPlaceholder="Search today's tasks...">
      {tasks.length === 0 && !isLoading ? (
        <List.EmptyView icon={Icon.CheckCircle} title="No tasks found for today" />
      ) : (
        tasks.map((task) => (
          <List.Item
            key={task.id}
            icon={Icon.Circle}
            title={task.name}
            subtitle={formatDuration(task.total_seconds)}
            accessories={[{ text: new Date(task.created_at).toLocaleTimeString() }]}
          />
        ))
      )}
    </List>
  );
}

function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;

  const parts = [];
  if (h > 0) parts.push(`${h}h`);
  if (m > 0 || h > 0) parts.push(`${m}m`);
  parts.push(`${s}s`);

  return parts.length > 0 ? parts.join(" ") : "0s";
}
