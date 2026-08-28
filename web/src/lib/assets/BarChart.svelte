<script lang="ts">
  let { data }: { data: { title: string; word_count: number }[] } = $props();

  // Calculate maximum word count for scaling bars relative to SVG height
  let maxCount = $derived(Math.max(...data.map((d) => d.word_count), 1));
</script>

<div class="bg-white border rounded-lg p-6 shadow-sm">
  <h3 class="text-sm font-medium text-gray-500 mb-4">Word Count Comparison</h3>

  <svg viewBox="0 0 500 200" class="w-full h-48 overflow-visible">
    {#each data as point, i}
      {@const barHeight = (point.word_count / maxCount) * 160}
      {@const x = i * (500 / data.length) + 10}
      {@const barWidth = 500 / data.length - 20}

      <!-- Bar -->
      <rect
        {x}
        y={180 - barHeight}
        width={barWidth}
        height={barHeight}
        rx="4"
        class="fill-orange-500 hover:fill-orange-600 transition-colors cursor-pointer"
      >
        <title>{point.title}: {point.word_count.toLocaleString()} words</title>
      </rect>

      <!-- Label -->
      <text x={x + barWidth / 2} y="195" text-anchor="middle" class="text-[10px] fill-gray-500 font-sans">
        {point.title.length > 10 ? point.title.slice(0, 10) + '...' : point.title}
      </text>
    {/each}
  </svg>
</div>
