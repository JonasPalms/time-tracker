import { invoke } from "@tauri-apps/api/core";

export interface Favourite {
  id: number;
  name: string;
  duration_seconds: number;
}

export async function createFavourite(
  name: string,
  durationSeconds: number
): Promise<number | undefined> {
  return invoke<number>("create_favourite", { name, durationSeconds });
}

export async function getFavourites(): Promise<Favourite[]> {
  return invoke<Favourite[]>("get_favourites");
}

export async function deleteFavourite(id: number): Promise<void> {
  return invoke("delete_favourite", { id });
}
