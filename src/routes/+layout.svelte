<script lang="ts">
  import "../app.css"
  import { createThemeContext } from "$lib/theme.svelte"
  import { onMount } from "svelte"

  let { children } = $props()

  // Create and provide theme context to all children
  const { init } = createThemeContext()

  onMount(() => {
    init()
  })
</script>

<div class="min-h-screen bg-surface text-on-surface">
  {@render children()}
</div>
