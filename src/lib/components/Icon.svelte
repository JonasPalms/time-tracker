<script lang="ts">
  const icons = import.meta.glob("$lib/icons/*.svg", {
    eager: true,
    query: "?raw",
    import: "default",
  }) as Record<string, string>;

  // Extract icon names from paths for the type
  type IconName =
    | "play"
    | "pause"
    | "stop"
    | "settings"
    | "clock"
    | "chevron-left"
    | "chevron-right"
    | "x"
    | "trash"
    | "home";

  let { name, class: className = "w-5 h-5" }: { name: IconName; class?: string } = $props();

  // Get the SVG content and inject the class
  const svgContent = $derived(() => {
    const path = `/src/lib/icons/${name}.svg`;
    const raw = icons[path];
    if (!raw) return "";
    // Inject the class into the SVG tag
    return raw.replace("<svg", `<svg class="${className}"`);
  });
</script>

{@html svgContent()}
