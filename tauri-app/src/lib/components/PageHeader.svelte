<script lang="ts">
  import type { Snippet } from "svelte";
  import { cn } from "$lib/utils";

  interface Props {
    children: Snippet;
    border?: boolean;
  }

  let { children, border = true }: Props = $props();
</script>

<header class={cn("shrink-0 px-app pt-4", border && "border-b border-on-surface/10")}>
  {@render children()}
</header>
