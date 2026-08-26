mod db;
mod gutenberg;
mod parse;

use db::Database;
use gutenberg::{fetch_gutenberg_metadata, fetch_gutenberg_text};
use parse::{analyze_text, AnalysisPayload, BookMetadata};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let book_ids: Vec<u32> = if args.len() > 1 {
        args[1..]
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    } else {
        vec![209, 863, 2852]
    };

    // 1. Initialize local DuckDB connection
    let db = Database::new("noir_corpus.duckdb").unwrap_or_else(|err| {
        eprintln!("Failed to initialize DuckDB: {}", err);
        std::process::exit(1);
    });

    println!("=== Processing Corpus Batch into DuckDB: {:?} ===\n", book_ids);

    for book_id in book_ids {
        println!("--> Fetching & analyzing Book ID #{}...", book_id);

        let metadata = fetch_gutenberg_metadata(book_id).unwrap_or_else(|_| BookMetadata {
            gutenberg_id: book_id,
            title: "Unknown Title".to_string(),
            author: "Unknown Author".to_string(),
            author_birth_year: None,
            author_death_year: None,
        });

        let raw_text = match fetch_gutenberg_text(book_id) {
            Ok(text) => text,
            Err(err) => {
                eprintln!("    [!] Skipping Book #{}: {}", book_id, err);
                continue;
            }
        };

        let metrics = analyze_text(&raw_text);
        let payload = AnalysisPayload { metadata, metrics };

        // 2. Persist directly to DuckDB
        if let Err(err) = db.save_payload(&payload) {
            eprintln!("    [!] Error saving Book #{} to DB: {}", book_id, err);
        } else {
            println!("    [✓] Persisted '{}' to DuckDB", payload.metadata.title);
        }
    }

    println!("\n=== Done! Records written to noir_corpus.duckdb ===");
}