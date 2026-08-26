mod gutenberg;
mod parse;

use gutenberg::{fetch_gutenberg_metadata, fetch_gutenberg_text};
use parse::{analyze_text, AnalysisPayload, BookMetadata};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let book_id: u32 = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(209); // Default: Book #209 ("The Turn of the Screw")

    println!("Fetching Metadata & Text for Book ID #{}...", book_id);

    // 1. Fetch metadata (Title, Author, Years) via Gutendex API
    let metadata = fetch_gutenberg_metadata(book_id).unwrap_or_else(|_| BookMetadata {
        gutenberg_id: book_id,
        title: "Unknown Title".to_string(),
        author: "Unknown Author".to_string(),
        author_birth_year: None,
        author_death_year: None,
    });

    // 2. Fetch raw text via Gutenberg CDN
    let raw_text = fetch_gutenberg_text(book_id).unwrap_or_else(|err| {
        eprintln!("Failed to fetch text: {}", err);
        std::process::exit(1);
    });

    // 3. Run metric analysis
    let metrics = analyze_text(&raw_text);

    // 4. Combine into final flattened payload
    let payload = AnalysisPayload { metadata, metrics };

    let json_output = serde_json::to_string_pretty(&payload).expect("Failed to serialize");

    println!("\n=== Stylistic Analysis Output ===");
    println!("{}", json_output);
}