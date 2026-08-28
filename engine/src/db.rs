use crate::parse::AnalysisPayload;
use duckdb::{params, Connection, Result};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS corpus_metrics (
                gutenberg_id INTEGER PRIMARY KEY,
                title VARCHAR,
                author VARCHAR,
                word_count INTEGER,
                unique_words INTEGER,
                sentence_count INTEGER,
                lexical_density DOUBLE,
                dialogue_ratio DOUBLE,
                avg_sentence_length DOUBLE
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn save_payload(&self, payload: &AnalysisPayload) -> Result<()> {
        self.conn.execute(
            "INSERT INTO corpus_metrics (
                gutenberg_id, title, author, word_count, unique_words,
                sentence_count, lexical_density, dialogue_ratio, avg_sentence_length
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (gutenberg_id) DO UPDATE SET
                title = EXCLUDED.title,
                author = EXCLUDED.author,
                word_count = EXCLUDED.word_count,
                unique_words = EXCLUDED.unique_words,
                sentence_count = EXCLUDED.sentence_count,
                lexical_density = EXCLUDED.lexical_density,
                dialogue_ratio = EXCLUDED.dialogue_ratio,
                avg_sentence_length = EXCLUDED.avg_sentence_length;",
            params![
                &payload.metadata.gutenberg_id,
                &payload.metadata.title,
                &payload.metadata.author,
                &(payload.metrics.word_count as i64),
                &(payload.metrics.unique_word_count as i64),
                &(payload.metrics.sentence_count as i64),
                &payload.metrics.lexical_density,
                &payload.metrics.dialogue_ratio,
                &payload.metrics.avg_sentence_length,
            ],
        )?;

        Ok(())
    }

    pub fn export_to_parquet(&self, parquet_path: &str) -> Result<()> {
        let sql = format!(
            "COPY corpus_metrics TO '{}' (FORMAT PARQUET);",
            parquet_path
        );
        self.conn.execute(&sql, [])?;
        Ok(())
    }
}