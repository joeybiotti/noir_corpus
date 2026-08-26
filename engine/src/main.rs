mod gutenberg;
mod parse;

use gutenberg::fetch_gutenberg_text;
use parse::analyze_text;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String>=env::args().collect();
    let target = args.get(1).map(|s| s.as_str()).unwrap_or("208");

    let raw_text = if let Ok(book_id) = target.parse::<u32>(){
        fetch_gutenberg_text(book_id).unwrap_or_else(|err| {
            eprintln!("Failed to fetch Gutenberg book #{}: {}", book_id, err);
            std::process::exit(1);
        })
    }else {
        println!("Reading local file: {}", target);
        fs::read_to_string(Path::new(target)).unwrap_or_else(|err| {
            eprintln!("Failed to read file: {}", err);
            std::process::exit(1);
        })
    };

    let metrics = analyze_text(&raw_text);
    let json_output = serde_json::to_string_pretty(&metrics).expect("Failed to serialize metrics");

    println!("\n=== Stylistic Analysis Output ===");
    println!("{}", json_output);
}
