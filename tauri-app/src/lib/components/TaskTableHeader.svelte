<script lang="ts">
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ChevronUpIcon from "@lucide/svelte/icons/chevron-up";

  type SortBy = "name" | "time" | null;
  type SortDirection = "asc" | "desc";

  let {
    sortBy,
    sortDirection,
    onToggleSort,
  }: {
    sortBy: SortBy;
    sortDirection: SortDirection;
    onToggleSort: (column: "name" | "time") => void;
  } = $props();

  function getAriaSort(column: "name" | "time"): "none" | "ascending" | "descending" {
    if (sortBy !== column) return "none";
    return sortDirection === "asc" ? "ascending" : "descending";
  }
</script>

<thead>
  <tr>
    <th class="w-14 py-2 pl-2 pr-1 text-left text-xs font-semibold uppercase tracking-wide text-on-surface-muted">
      <span class="sr-only">Actions</span>
    </th>
    <th
      class="py-2 pr-3 text-left text-xs font-semibold uppercase tracking-wide text-on-surface-muted"
      aria-sort={getAriaSort("name")}
    >
      <button
        class="inline-flex items-center gap-2 hover:text-on-surface transition-colors"
        onclick={() => onToggleSort("name")}
      >
        <span>Name</span>
        {#if sortBy === "name"}
          {#if sortDirection === "asc"}
            <ChevronUpIcon class="w-3 h-3 text-accent" />
          {:else}
            <ChevronDownIcon class="w-3 h-3 text-accent" />
          {/if}
        {/if}
      </button>
    </th>
    <th
      class="w-33 py-2 px-3 text-left text-xs font-semibold uppercase tracking-wide text-on-surface-muted"
      aria-sort={getAriaSort("time")}
    >
      <button
        class="inline-flex items-center gap-2 hover:text-on-surface transition-colors"
        onclick={() => onToggleSort("time")}
      >
        <span class="ml-[3px]">Duration</span>
        {#if sortBy === "time"}
          {#if sortDirection === "asc"}
            <ChevronUpIcon class="w-3 h-3 text-accent" />
          {:else}
            <ChevronDownIcon class="w-3 h-3 text-accent" />
          {/if}
        {/if}
      </button>
    </th>
    <th class="w-12 py-2 pl-1 pr-2 text-right text-xs font-semibold uppercase tracking-wide text-on-surface-muted">
      <span class="sr-only">More actions</span>
    </th>
  </tr>
</thead>
