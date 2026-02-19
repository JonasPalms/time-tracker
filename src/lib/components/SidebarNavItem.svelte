<script lang="ts">
  import Icon from "./Icon.svelte";
  import { page } from "$app/stores";

  type IconName = "home" | "clock" | "settings";

  let {
    href,
    icon,
    label,
    collapsed = false,
    onclick,
  }: {
    href?: string;
    icon: IconName;
    label: string;
    collapsed?: boolean;
    onclick?: () => void;
  } = $props();

  const isActive = $derived(href ? $page.url.pathname === href : false);
</script>

{#if href}
  <a
    {href}
    class="flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors
      {isActive ? 'bg-surface-raised text-on-surface' : 'text-on-surface-muted hover:bg-surface-raised hover:text-on-surface'}"
    title={collapsed ? label : undefined}
  >
    <Icon name={icon} class="w-5 h-5 shrink-0" />
    {#if !collapsed}
      <span class="text-sm font-medium">{label}</span>
    {/if}
  </a>
{:else}
  <button
    {onclick}
    class="flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors w-full
      text-on-surface-muted hover:bg-surface-raised hover:text-on-surface"
    title={collapsed ? label : undefined}
  >
    <Icon name={icon} class="w-5 h-5 shrink-0" />
    {#if !collapsed}
      <span class="text-sm font-medium">{label}</span>
    {/if}
  </button>
{/if}
