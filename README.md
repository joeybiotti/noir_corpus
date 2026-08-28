# noir_corpus

An end-to-end text analytics engine and interactive browser workspace for 20th-century hardboiled, noir, and classic fiction.

The system ingests raw Project Gutenberg texts, cleans boilerplate, computes token-level and stylistic metrics in Rust, exports a single Parquet file, and serves interactive analytics directly in the browser via DuckDB WebAssembly and Svelte 5.

## Architecture & Data Flow

```text
  +-----------------------+
  |  Project Gutenberg    |
  |  Raw .txt Downloads   |
  +-----------+-----------+
              |
              v
  +-----------------------+
  |  Rust Engine          |
  |  - Strip Boilerplate  |
  |  - Tokenize Words     |
  |  - Lexical Metrics    |
  |  - Dialogue Analysis  |
  +-----------+-----------+
              |
              v
  +-----------------------+
  |  Parquet Output       |
  |  noir_corpus.parquet  |
  +-----------+-----------+
              |
              v
  +-----------------------+
  |  Svelte 5 + WASM      |
  |  - Web Worker Loading |
  |  - In-Browser SQL     |
  |  - Dynamic SVG Charts |
  +-----------------------+
```


## Project Structure

- engine/
  High-performance Rust engine handling raw text ingestion, metric calculations, and Parquet serialization.

- dashboard/
  Client-side web application built with Svelte 5 and DuckDB WebAssembly for serverless browser analytics.

## Tech Stack

- Engine: Rust 1.85+, DuckDB crate, Apache Arrow / Parquet
- Data Format: Apache Parquet (noir_corpus.parquet)
- Frontend Framework: Svelte 5 ($state, $derived, $effect)
- Browser SQL Engine: @duckdb/duckdb-wasm
- Styling: Native CSS Custom Properties (zero-purge light/dark theme switching)

## Parquet Schema Reference

The Rust engine exports a flat table (noir_corpus.parquet) with the following attributes:

- gutenberg_id (VARCHAR): Project Gutenberg catalog identifier
- title (VARCHAR): Work title
- author (VARCHAR): Author name
- word_count (BIGINT): Total word count
- unique_words (BIGINT): Total distinct vocabulary count
- lexical_density (DOUBLE): Ratio of unique words to total word count
- dialogue_ratio (DOUBLE): Proportion of text enclosed in quotation marks
- avg_sentence_length (DOUBLE): Average word count per sentence

## Setup & Local Development

### Prerequisites

- Rust 1.85+ (with cargo)
- Node.js 18+ (or pnpm / npm)

### Step 1: Run the Pipeline (Engine)

Build and execute the Rust ingestion engine to produce the Parquet file:

cd engine
cargo run --release

Move the output file into the dashboard public asset directory:

cp data/noir_corpus.parquet ../dashboard/public/noir_corpus.parquet

### Step 2: Run the Dashboard

Navigate to the frontend directory, install dependencies, and launch Vite:

cd ../dashboard
npm install
npm run dev

Open http://localhost:5173. DuckDB WASM will load the Parquet file into memory and initialize the interactive views.

### Step 3: Production Build

Generate the static production build for web hosting:

cd dashboard
npm run build
npm run preview
