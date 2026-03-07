<script lang="ts">
  import { type DateValue, getLocalTimeZone } from "@internationalized/date";
  import CalendarIcon from "@lucide/svelte/icons/calendar";
  import { cn } from "$lib/utils.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Calendar } from "$lib/components/ui/calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";

  interface Props {
    value?: DateValue;
    onValueChange?: (value: DateValue | undefined) => void;
    placeholder?: string;
    class?: string;
  }

  let {
    value = $bindable(),
    onValueChange,
    placeholder = "Pick a date",
    class: className,
  }: Props = $props();

  let open = $state(false);

  function formatDate(date: DateValue): string {
    const jsDate = date.toDate(getLocalTimeZone());
    return jsDate.toLocaleDateString("en-US", {
      weekday: "short",
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function handleValueChange(newValue: DateValue | undefined) {
    value = newValue;
    onValueChange?.(newValue);
    if (newValue) {
      open = false;
    }
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        variant="outline"
        class={cn(
          "w-full justify-start text-left font-normal",
          !value && "text-muted-foreground",
          className
        )}
        {...props}
      >
        <CalendarIcon class="mr-2 size-4" />
        {value ? formatDate(value) : placeholder}
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-auto p-0" align="start">
    <Calendar {value} onValueChange={handleValueChange} initialFocus />
  </Popover.Content>
</Popover.Root>
