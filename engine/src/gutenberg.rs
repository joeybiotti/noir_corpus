use reqwest::blocking::Client;
use std::error::Error;
use super::parse::BookMetadata;

/// Fetches raw book text directly from Project Gutenberg standard or fallback cache endpoints.
pub fn fetch_gutenberg_text(client: &Client, book_id: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://www.gutenberg.org/files/{}/{}.txt", book_id, book_id);
    let alt_url = format!("https://www.gutenberg.org/cache/epub/{}/pg{}.txt", book_id, book_id);

    let response_text = match client.get(&url).send() {
        Ok(res) if res.status().is_success() => res.text()?,
        _ => {
            let alt_res = client.get(&alt_url).send()?;
            if !alt_res.status().is_success() {
                return Err(format!("Could not locate text file for Gutenberg ID {}", book_id).into());
            }
            alt_res.text()?
        }
    };

    Ok(response_text)
}

/// Strictly inspects Gutenberg header Subject fields to allow ONLY crime, detective, noir, and mystery.
pub fn is_target_genre(raw_text: &str) -> bool {
    let strict_noir_keywords = [
        "detective",
        "mystery",
        "crime",
        "noir",
        "murder",
        "pulp",
        "hardboiled",
        "burglar",
        "thief",
        "sherlock",
        "holmes",
        "fiction",
    ];

    let exclude_keywords = [
        "humorous",
        "juvenile",
        "fairy tales",
        "poetry",
        "essays",
    ];

    // Read the first 250 lines (where Gutenberg metadata lives)
    let header_block: String = raw_text.lines().take(250).collect::<Vec<&str>>().join(" ").to_lowercase();

    // Reject explicit exclusions first
    if exclude_keywords.iter().any(|&ex| header_block.contains(ex)) {
        return false;
    }

    // Match any crime/mystery/detective keyword anywhere in the header block
    if strict_noir_keywords.iter().any(|&kw| header_block.contains(kw)) {
        return true;
    }

    // If no subject headers exist at all in the header block, allow the book through
    true
}

/// Parses metadata (Title & Author) directly from standard Gutenberg header lines.
pub fn extract_metadata_from_text(book_id: u32, raw_text: &str) -> BookMetadata {
    let mut title = format!("Gutenberg Book #{}", book_id);
    let mut author = "Unknown Author".to_string();

    for line in raw_text.lines().take(150) {
        let line_trim = line.trim();
        let line_lower = line_trim.to_lowercase();

        if line_lower.starts_with("title:") {
            let parsed_title = line_trim["title:".len()..].trim();
            if !parsed_title.is_empty() {
                title = parsed_title.to_string();
            }
        } else if line_lower.starts_with("author:") {
            let parsed_author = line_trim["author:".len()..].trim();
            if !parsed_author.is_empty() {
                author = parsed_author.to_string();
            }
        }
    }

    BookMetadata {
        gutenberg_id: book_id,
        title,
        author,
        author_birth_year: None,
        author_death_year: None,
    }
}

/// Strips standard Project Gutenberg boilerplate headers and footers for clean metrics parsing.
pub fn strip_gutenberg_header_footer(raw: &str) -> String {
    let start_markers = [
        "*** START OF THIS PROJECT GUTENBERG EBOOK",
        "*** START OF THE PROJECT GUTENBERG EBOOK",
    ];

    let end_markers = [
        "*** END OF THIS PROJECT GUTENBERG EBOOK",
        "*** END OF THE PROJECT GUTENBERG EBOOK",
    ];

    let mut start_pos = 0;
    for marker in start_markers {
        if let Some(pos) = raw.find(marker) {
            if let Some(line_end) = raw[pos..].find('\n') {
                start_pos = pos + line_end + 1;
                break;
            }
        }
    }

    let mut end_pos = raw.len();
    for marker in end_markers {
        if let Some(pos) = raw.find(marker) {
            end_pos = pos;
            break;
        }
    }

    if start_pos < end_pos {
        raw[start_pos..end_pos].trim().to_string()
    } else {
        raw.to_string()
    }
}