<script lang="ts">
  import { Calendar as CalendarPrimitive } from "bits-ui";
  import type { DateValue } from "@internationalized/date";
  import ChevronLeftIcon from "@lucide/svelte/icons/chevron-left";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import { cn } from "$lib/utils.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";

  interface Props {
    value?: DateValue;
    onValueChange?: (value: DateValue | undefined) => void;
    placeholder?: DateValue;
    class?: string;
    weekdayFormat?: "narrow" | "short" | "long";
    initialFocus?: boolean;
  }

  let {
    value = $bindable(),
    onValueChange,
    placeholder = $bindable(),
    class: className,
    weekdayFormat = "short",
    initialFocus = false,
  }: Props = $props();
</script>

<CalendarPrimitive.Root
  type="single"
  bind:value
  bind:placeholder
  {onValueChange}
  {weekdayFormat}
  {initialFocus}
  class={cn("p-3", className)}
>
  {#snippet children({ months, weekdays })}
    <CalendarPrimitive.Header class="relative flex w-full items-center justify-between pt-1">
      <CalendarPrimitive.PrevButton
        class={cn(
          buttonVariants({ variant: "outline" }),
          "size-7 bg-transparent p-0 opacity-50 hover:opacity-100"
        )}
      >
        <ChevronLeftIcon class="size-4" />
      </CalendarPrimitive.PrevButton>
      <CalendarPrimitive.Heading class="text-sm font-medium" />
      <CalendarPrimitive.NextButton
        class={cn(
          buttonVariants({ variant: "outline" }),
          "size-7 bg-transparent p-0 opacity-50 hover:opacity-100"
        )}
      >
        <ChevronRightIcon class="size-4" />
      </CalendarPrimitive.NextButton>
    </CalendarPrimitive.Header>
    <div class="flex flex-col gap-4 pt-4 sm:flex-row sm:gap-2">
      {#each months as month (month.value.toString())}
        <CalendarPrimitive.Grid class="w-full border-collapse space-y-1">
          <CalendarPrimitive.GridHead>
            <CalendarPrimitive.GridRow class="flex">
              {#each weekdays as weekday}
                <CalendarPrimitive.HeadCell
                  class="w-8 rounded-md text-[0.8rem] font-normal text-muted-foreground"
                >
                  {weekday.slice(0, 2)}
                </CalendarPrimitive.HeadCell>
              {/each}
            </CalendarPrimitive.GridRow>
          </CalendarPrimitive.GridHead>
          <CalendarPrimitive.GridBody class="[&>tr]:mt-2">
            {#each month.weeks as weekDates}
              <CalendarPrimitive.GridRow class="flex">
                {#each weekDates as date}
                  <CalendarPrimitive.Cell {date} month={month.value} class="relative p-0 text-center text-sm">
                    <CalendarPrimitive.Day
                      class={cn(
                        buttonVariants({ variant: "ghost" }),
                        "size-8 p-0 font-normal",
                        "[&[data-today]:not([data-selected])]:bg-accent [&[data-today]:not([data-selected])]:text-accent-foreground",
                        "[&[data-selected]]:bg-primary [&[data-selected]]:text-primary-foreground [&[data-selected]]:opacity-100 [&[data-selected]]:hover:bg-primary [&[data-selected]]:hover:text-primary-foreground [&[data-selected]]:focus:bg-primary [&[data-selected]]:focus:text-primary-foreground",
                        "[&[data-disabled]]:text-muted-foreground [&[data-disabled]]:opacity-50",
                        "[&[data-outside-month]]:text-muted-foreground [&[data-outside-month]]:opacity-50 [&[data-outside-month][data-selected]]:bg-accent/50 [&[data-outside-month][data-selected]]:text-muted-foreground [&[data-outside-month][data-selected]]:opacity-30",
                        "[&[data-unavailable]]:text-destructive-foreground [&[data-unavailable]]:line-through"
                      )}
                    />
                  </CalendarPrimitive.Cell>
                {/each}
              </CalendarPrimitive.GridRow>
            {/each}
          </CalendarPrimitive.GridBody>
        </CalendarPrimitive.Grid>
      {/each}
    </div>
  {/snippet}
</CalendarPrimitive.Root>
