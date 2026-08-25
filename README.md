# noir_corpus

An end-to-end analytical corpus engine and headless BI workspace for 20th-century hardboiled, noir, and classic fiction text analysis.

## Architecture

* **`engine/`**: High-performance text processing, token metric calculation, and DuckDB export written in Rust.
* **`dashboard/`**: Code-first headless BI dashboards built with Evidence.dev and SQL.

## Tech Stack

* **Language**: Rust
* **Storage / Database**: DuckDB & Parquet
* **Visualization Layer**: Evidence.dev (Markdown + SQL)

## Getting Started

### Prerequisites

* Rust 1.98+
* Node.js 18+

### Setup

1. **Build Rust Engine**
   ```bash
   cd engine
   cargo build