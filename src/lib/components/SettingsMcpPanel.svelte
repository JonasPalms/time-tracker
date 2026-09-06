<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import {
    formatMcpSnippet,
    getMcpBinaryPath,
    MCP_CLIENTS,
    type McpClient,
  } from "$lib/services/mcp";
  import { Check, Copy } from "@lucide/svelte";

  let client = $state<McpClient>("cursor");
  let command = $state<string | null>(null);
  let error = $state<string | null>(null);
  let copied = $state(false);
  let copyReset: ReturnType<typeof setTimeout> | undefined;

  const snippet = $derived(command ? formatMcpSnippet(client, command) : "");

  $effect(() => {
    let cancelled = false;

    void getMcpBinaryPath()
      .then((path) => {
        if (cancelled) return;
        command = path;
        error = null;
      })
      .catch((reason: unknown) => {
        if (cancelled) return;
        command = null;
        error = reason instanceof Error ? reason.message : String(reason);
      });

    return () => {
      cancelled = true;
      if (copyReset) clearTimeout(copyReset);
    };
  });

  function selectClient(next: McpClient) {
    client = next;
    copied = false;
  }

  async function copySnippet() {
    if (!snippet) return;

    await navigator.clipboard.writeText(snippet);
    copied = true;

    if (copyReset) clearTimeout(copyReset);
    copyReset = setTimeout(() => {
      copied = false;
    }, 1500);
  }
</script>

<div class="min-w-0 space-y-4">
  <p class="text-sm text-on-surface-muted">
    Copy the snippet for your client and paste it there. TimeTracker does not install or manage the
    connection.
  </p>

  <div
    class="flex flex-wrap gap-1 rounded-lg bg-surface-raised p-1"
    role="tablist"
    aria-label="MCP client"
  >
    {#each MCP_CLIENTS as option (option.id)}
      <button
        type="button"
        role="tab"
        aria-selected={client === option.id}
        class="flex-1 rounded-md px-3 py-1.5 text-sm font-medium transition-colors {client ===
        option.id
          ? 'bg-surface text-on-surface shadow-xs'
          : 'text-on-surface-muted hover:text-on-surface'}"
        onclick={() => selectClient(option.id)}
      >
        {option.label}
      </button>
    {/each}
  </div>

  {#if error}
    <p class="text-sm text-on-surface-muted">{error}</p>
  {:else if command}
    <p class="text-sm text-on-surface-muted">
      {MCP_CLIENTS.find((option) => option.id === client)?.hint}
    </p>

    <div class="min-w-0 overflow-hidden rounded-lg bg-surface-raised">
      <pre
        class="max-h-56 min-w-0 max-w-full cursor-text overflow-auto p-4 text-xs leading-5 break-all whitespace-pre-wrap select-text text-on-surface selection:bg-accent/40 selection:text-on-surface"
      >{snippet}</pre>
    </div>

    <Button variant="outline" onclick={copySnippet}>
      {#if copied}
        <Check class="size-4" />
        Copied
      {:else}
        <Copy class="size-4" />
        Copy snippet
      {/if}
    </Button>
  {/if}
</div>
