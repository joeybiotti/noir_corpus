use reqwest::blocking::Client;
use std::error::Error;

pub fn fetch_gutenberg_text(book_id: u32) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://www.gutenberg.org/files/{}/{}.txt",
        book_id, book_id
    );

    // Fallback URL
    let alt_url = format!(
        "https://www.gutenberg.org/cache/epub/{}/pg{}.txt",
        book_id, book_id
    );

    let client = Client::builder().user_agent("prose_analyzer/1.0").build()?;

    println!("Fetching Book ID # {} from Project Gutenberg", book_id);

    let response = match client.get(&url).send() {
        Ok(res) if res.status().is_success() => res.text()?,
        _ => client.get(&alt_url).send()?.text()?,
    };

    let cleaned_text = strip_gutenberg_header_footer(&response);
    Ok(cleaned_text)
}

/// Strips standard Project Gutenberg boilerplate headers and footers.
fn strip_gutenberg_header_footer(raw: &str) -> String {
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
