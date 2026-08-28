<script lang="ts">
	import { onMount } from 'svelte';
	import * as duckdb from '@duckdb/duckdb-wasm';
	import duckdb_wasm from '@duckdb/duckdb-wasm/dist/duckdb-mvp.wasm?url';
	import mvp_worker from '@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js?url';

	let loading = $state(true);
	let isDark = $state(true);
	let metrics: any[] = $state([]);

	let maxWordCount = $derived(
		metrics.length > 0 ? Math.max(...metrics.map((m) => m.word_count)) : 1
	);

	function toggleTheme() {
		isDark = !isDark;
	}

	onMount(async () => {
		const worker = new Worker(mvp_worker);
		const logger = new duckdb.ConsoleLogger();
		const db = new duckdb.AsyncDuckDB(logger, worker);
		await db.instantiate(duckdb_wasm);

		const conn = await db.connect();

		const res = await fetch('/noir_corpus.parquet');
		const buffer = await res.arrayBuffer();
		await db.registerFileBuffer('noir_corpus.parquet', new Uint8Array(buffer));

		const result = await conn.query(`
			SELECT 
				gutenberg_id,
				title,
				author,
				word_count,
				unique_words,
				ROUND(lexical_density, 3) AS lexical_density,
				ROUND(dialogue_ratio, 3) AS dialogue_ratio,
				ROUND(avg_sentence_length, 1) AS avg_sentence_len
			FROM 'noir_corpus.parquet'
			ORDER BY word_count DESC
		`);

		metrics = result.toArray().map((r) => r.toJSON());
		loading = false;
	});
</script>

<div class="app-root" class:theme-dark={isDark} class:theme-light={!isDark}>
	<main class="p-4 max-w-5xl mx-auto font-sans text-xs">
		<header class="mb-4 border-b pb-2 flex items-center justify-between header-border">
			<div>
				<h1 class="text-lg font-bold tracking-tight">Noir Corpus Analytics</h1>
				<p class="text-[11px] mt-0.5 subtitle-color">
					Client-side SQL execution over Parquet via DuckDB WebAssembly
				</p>
			</div>

			<button
				type="button"
				onclick={toggleTheme}
				class="px-2.5 py-1 text-[11px] font-mono rounded border transition-colors cursor-pointer toggle-btn"
			>
				{isDark ? '☀️ Light Mode' : '🌙 Dark Mode'}
			</button>
		</header>

		{#if loading}
			<div class="flex items-center space-x-2 text-xs subtitle-color">
				<div class="w-3 h-3 rounded-full border-2 border-orange-500 border-t-transparent animate-spin"></div>
				<span>Initializing DuckDB WASM & processing corpus...</span>
			</div>
		{:else}
			<div class="mb-6 border rounded p-4 shadow-sm card-bg">
				<h3 class="text-xs font-semibold mb-2 heading-color">
					Word Count Comparison
				</h3>

				<svg viewBox="0 0 500 120" class="w-full h-28 overflow-visible">
					{#each metrics as point, i}
						{@const barHeight = (point.word_count / maxWordCount) * 110}
						{@const x = i * (500 / metrics.length) + 4}
						{@const barWidth = Math.max(500 / metrics.length - 8, 6)}

						<rect
							{x}
							y={118 - barHeight}
							width={barWidth}
							height={barHeight}
							rx="2"
							class="fill-orange-500 hover:fill-orange-400 transition-colors cursor-pointer"
						>
							<title>{point.title} ({point.author}): {point.word_count.toLocaleString()} words</title>
						</rect>
					{/each}
				</svg>
			</div>

			<div class="overflow-x-auto border rounded table-border">
				<table class="w-full text-left border-collapse text-xs">
					<thead>
						<tr class="border-b table-header">
							<th class="p-1.5 font-semibold text-[11px]">ID</th>
							<th class="p-1.5 font-semibold text-[11px]">Title</th>
							<th class="p-1.5 font-semibold text-[11px]">Author</th>
							<th class="p-1.5 font-semibold text-[11px]">Word Count</th>
							<th class="p-1.5 font-semibold text-[11px]">Dialogue Ratio</th>
							<th class="p-1.5 font-semibold text-[11px]">Avg Sentence Len</th>
						</tr>
					</thead>
					<tbody>
						{#each metrics as row}
							<tr class="border-b table-row transition-colors">
								<td class="p-1.5 font-mono text-[10px] muted-color">{row.gutenberg_id}</td>
								<td class="p-1.5 font-medium">{row.title}</td>
								<td class="p-1.5 subtitle-color">{row.author}</td>
								<td class="p-1.5 font-mono text-[11px]">{row.word_count.toLocaleString()}</td>
								<td class="p-1.5 font-mono text-[11px]">{(row.dialogue_ratio * 100).toFixed(1)}%</td>
								<td class="p-1.5 font-mono text-[11px]">{row.avg_sentence_len}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</main>
</div>

<style>
	/* Dark Theme Variables */
	.theme-dark {
		background-color: #020617;
		color: #f8fafc;
	}
	.theme-dark .header-border { border-color: #1e293b; }
	.theme-dark .subtitle-color { color: #94a3b8; }
	.theme-dark .heading-color { color: #cbd5e1; }
	.theme-dark .muted-color { color: #64748b; }
	.theme-dark .card-bg { background-color: #0f172a; border-color: #1e293b; }
	.theme-dark .table-border { border-color: #1e293b; }
	.theme-dark .table-header { background-color: #0f172a; border-color: #1e293b; color: #cbd5e1; }
	.theme-dark .table-row { border-color: #1e293b; }
	.theme-dark .table-row:hover { background-color: #0f172a; }
	.theme-dark .toggle-btn { background-color: #1e293b; color: #e2e8f0; border-color: #334155; }
	.theme-dark .toggle-btn:hover { background-color: #334155; }

	/* Light Theme Variables */
	.theme-light {
		background-color: #f8fafc;
		color: #0f172a;
	}
	.theme-light .header-border { border-color: #e2e8f0; }
	.theme-light .subtitle-color { color: #64748b; }
	.theme-light .heading-color { color: #334155; }
	.theme-light .muted-color { color: #94a3b8; }
	.theme-light .card-bg { background-color: #ffffff; border-color: #e2e8f0; }
	.theme-light .table-border { border-color: #e2e8f0; }
	.theme-light .table-header { background-color: #f1f5f9; border-color: #e2e8f0; color: #334155; }
	.theme-light .table-row { border-color: #f1f5f9; }
	.theme-light .table-row:hover { background-color: #f8fafc; }
	.theme-light .toggle-btn { background-color: #ffffff; color: #334155; border-color: #cbd5e1; }
	.theme-light .toggle-btn:hover { background-color: #f1f5f9; }

	.app-root {
		min-height: 100vh;
		transition: background-color 0.2s ease, color 0.2s ease;
	}
</style>