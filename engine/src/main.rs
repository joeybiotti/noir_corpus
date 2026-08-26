mod gutenberg;
mod parse;

use gutenberg::{fetch_gutenberg_metadata, fetch_gutenberg_text};
use parse::{analyze_text, AnalysisPayload, BookMetadata};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse all positional arguments as u32 IDs.
    // If no IDs passed, default to a classic literature batch.
    let book_ids: Vec<u32> = if args.len() > 1 {
        args[1..]
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    } else {
        vec![
            209,  // The Turn of the Screw (Henry James)
            863,  // The Mysterious Affair at Styles (Agatha Christie)
            2852, // The Hound of the Baskervilles (Arthur Conan Doyle)
        ]
    };

    println!("=== Processing Corpus Batch: {:?} ===\n", book_ids);

    let mut corpus_results: Vec<AnalysisPayload> = Vec::new();

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

        corpus_results.push(AnalysisPayload { metadata, metrics });
    }

    let json_output = serde_json::to_string_pretty(&corpus_results)
        .expect("Failed to serialize batch output");

    println!("\n=== Corpus Batch Analysis Output ===");
    println!("{}", json_output);
}