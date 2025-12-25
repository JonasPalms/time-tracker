import { getDb } from "./db";

export interface Favourite {
  id: number;
  name: string;
  duration_seconds: number;
}

export async function createFavourite(
  name: string,
  durationSeconds: number
): Promise<number | undefined> {
  const db = await getDb();
  const result = await db.execute(
    `
      INSERT INTO favourites (name, duration_seconds)
      VALUES (?, ?)
    `,
    [name, durationSeconds]
  );
  return result.lastInsertId;
}

export async function getFavourites(): Promise<Favourite[]> {
  const db = await getDb();
  const result = await db.select<Favourite[]>(
    `
      SELECT id, name, duration_seconds
      FROM favourites
    `
  );

  return result;
}

export async function deleteFavourite(id: number): Promise<void> {
  const db = await getDb();
  await db.execute(
    `
      DELETE FROM favourites
      WHERE id = ?
    `,
    [id]
  );
}
