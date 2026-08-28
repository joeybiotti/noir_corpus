mod db;
mod gutenberg;
mod parse;

use db::Database;
use gutenberg::{
    extract_metadata_from_text, fetch_gutenberg_text, is_target_genre,
    strip_gutenberg_header_footer,
};
use parse::{analyze_text, AnalysisPayload};
use reqwest::blocking::Client;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default to a curated seed list of detective/mystery/crime IDs if no CLI args are passed
    let book_ids: Vec<u32> = if args.len() > 1 {
        args[1..]
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    } else {
        vec![
            // Arthur Conan Doyle (Sherlock Holmes)
            108, 1661, 2097, // Edgar Allan Poe (C. Auguste Dupin / Early Detective)
            967, 2147, 829,
            // Wilkie Collins & Mary Roberts Rinehart (Early Crime / Detective)
            155, 2852, 400, 2238, 7020, // Maurice Leblanc (Arsène Lupin)
            4017, 4026, // Anna Katharine Green
            273, 534, // E. W. Hornung (Raffles)
            483, 735, // Gaston Leroux
            834, 1751,
        ]
    };

    let client = Client::builder()
        .user_agent("noir_corpus_analyzer/1.0 (contact@example.com)")
        .timeout(Duration::from_secs(15))
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to create HTTP client: {}", err);
            std::process::exit(1);
        });

    // Initialize DuckDB connection (can use :memory: or local DB)
    let db = Database::new(":memory:").unwrap_or_else(|err| {
        eprintln!("Failed to initialize DuckDB: {}", err);
        std::process::exit(1);
    });

    println!(
        "=== Processing Corpus Batch into Parquet/DuckDB: {} Candidates ===\n",
        book_ids.len()
    );

    for book_id in book_ids {
        println!("--> Processing Book ID #{}...", book_id);
        sleep(Duration::from_millis(300));

        let raw_text = match fetch_gutenberg_text(&client, book_id) {
            Ok(text) => text,
            Err(err) => {
                eprintln!("    [!] Skipping Book #{}: {}", book_id, err);
                continue;
            }
        };

        // 1. Filter out books that don't match target genres
        if !is_target_genre(&raw_text) {
            println!(
                "    [-] Skipping Book #{}: Outside mystery/crime genre target",
                book_id
            );
            continue;
        }

        // 2. Extract header metadata
        let metadata = extract_metadata_from_text(book_id, &raw_text);

        // 3. Clean and run text metrics analysis
        let cleaned_text = strip_gutenberg_header_footer(&raw_text);
        let metrics = analyze_text(&cleaned_text);

        let payload = AnalysisPayload { metadata, metrics };

        // 4. Save to DuckDB table
        if let Err(err) = db.save_payload(&payload) {
            eprintln!("    [!] Error saving Book #{}: {}", book_id, err);
        } else {
            println!(
                "    [✓] Persisted '{}' by {} to DuckDB",
                payload.metadata.title, payload.metadata.author
            );
        }
    }

    // 5. Export table to Parquet file for Evidence (bypasses DuckDB file lock issues)
    println!("\n--> Exporting corpus metrics to Parquet...");
    if let Err(err) = db.export_to_parquet("noir_corpus.parquet") {
        eprintln!("    [!] Error exporting to Parquet: {}", err);
    } else {
        println!("    [✓] Successfully exported data to 'noir_corpus.parquet'");
    }

    println!("\n=== Done! Output written to noir_corpus.parquet ===");
}
