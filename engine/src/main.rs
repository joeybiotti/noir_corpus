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
            108,   // The Return of Sherlock Holmes - Arthur Conan Doyle
            2097,  // The Sign of the Four - Arthur Conan Doyle
            1661,  // The Adventures of Sherlock Holmes - Arthur Conan Doyle
            2852,  // The Circular Staircase - Mary Roberts Rinehart
            155,   // The Moonstone - Wilkie Collins
            1188,  // The Mystery of Edwin Drood - Charles Dickens
            967,   // The Murders in the Rue Morgue - Edgar Allan Poe
            2147,  // The Works of Edgar Allan Poe - Vol 2 (Raven/Poe stories)
            7020,  // The Red House Mystery - A. A. Milne
            400,   // The Innocence of Father Brown - G. K. Chesterton
            2238,  // The Wisdom of Father Brown - G. K. Chesterton
            834,   // The Mystery of the Yellow Room - Gaston Leroux
            174,   // The Picture of Dorian Gray - Oscar Wilde
            5200,  // The Metamorphosis - Franz Kafka
            829,   // The Golden Bug - Edgar Allan Poe
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

    let db = Database::new("noir_corpus.duckdb").unwrap_or_else(|err| {
        eprintln!("Failed to initialize DuckDB: {}", err);
        std::process::exit(1);
    });

    println!(
        "=== Processing Corpus Batch into DuckDB: {} Candidates ===\n",
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
            println!("    [-] Skipping Book #{}: Outside mystery/fiction genre target", book_id);
            continue;
        }

        // 2. Extract header metadata
        let metadata = extract_metadata_from_text(book_id, &raw_text);

        // 3. Clean and run text metrics analysis
        let cleaned_text = strip_gutenberg_header_footer(&raw_text);
        let metrics = analyze_text(&cleaned_text);

        let payload = AnalysisPayload { metadata, metrics };

        // 4. Save to DuckDB
        if let Err(err) = db.save_payload(&payload) {
            eprintln!("    [!] Error saving Book #{}: {}", book_id, err);
        } else {
            println!(
                "    [✓] Persisted '{}' by {} to DuckDB",
                payload.metadata.title, payload.metadata.author
            );
        }
    }

    println!("\n=== Done! Records written to noir_corpus.duckdb ===");
}