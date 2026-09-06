<script lang="ts">
  import Icon from "./Icon.svelte";
  import { page } from "$app/state";

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

  const isActive = $derived(href ? page.url.pathname === href : false);
</script>

{#if href}
  <a
    {href}
    class="flex items-center gap-2.5 px-2.5 py-2 rounded-lg transition-colors focus-visible:bg-black/5 focus-visible:text-on-surface dark:focus-visible:bg-white/10
      {isActive
        ? 'bg-black/5 text-on-surface dark:bg-white/10'
        : 'text-on-surface-muted hover:bg-black/5 hover:text-on-surface dark:hover:bg-white/10'}"
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
    class="flex items-center gap-2.5 px-2.5 py-2 rounded-lg transition-colors w-full focus-visible:bg-black/5 focus-visible:text-on-surface dark:focus-visible:bg-white/10
      text-on-surface-muted hover:bg-black/5 hover:text-on-surface dark:hover:bg-white/10"
    title={collapsed ? label : undefined}
  >
    <Icon name={icon} class="w-5 h-5 shrink-0" />
    {#if !collapsed}
      <span class="text-sm font-medium">{label}</span>
    {/if}
  </button>
{/if}
