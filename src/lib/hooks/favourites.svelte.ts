import {
  getFavourites,
  createFavourite,
  deleteFavourite,
  type Favourite,
} from "$lib/services/favourites";

// Module-level reactive state - shared globally
let favourites = $state<Favourite[]>([]);
let isLoading = $state(false);

async function reload() {
  isLoading = true;
  favourites = await getFavourites();
  isLoading = false;
}

async function add(name: string, durationSeconds: number) {
  await createFavourite(name, durationSeconds);
  await reload();
}

async function remove(id: number) {
  await deleteFavourite(id);
  await reload();
}

// Export a function that returns getters to maintain reactivity
export function useFavourites() {
  return {
    get favourites() {
      return favourites;
    },
    get isLoading() {
      return isLoading;
    },
    reload,
    add,
    remove,
  };
}
