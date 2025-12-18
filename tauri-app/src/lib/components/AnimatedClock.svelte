<script lang="ts">
  let { class: className = "w-6 h-6" }: { class?: string } = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  class={className}
>
  <!-- Clock face -->
  <circle cx="12" cy="12" r="10" />
  <!-- Static hour hand (shorter, pointing up) -->
  <line x1="12" y1="12" x2="12" y2="8" />
  <!-- Rotating minute/second hand (longer) -->
  <line x1="12" y1="12" x2="12" y2="6">
    <animateTransform
      attributeName="transform"
      attributeType="XML"
      type="rotate"
      from="0 12 12"
      to="360 12 12"
      dur="14s"
      repeatCount="indefinite"
    />
  </line>
</svg>
