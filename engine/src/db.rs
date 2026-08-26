use crate::parse::AnalysisPayload;
use duckdb::{params, Connection, Result};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens or creates a DuckDB database file at the given path.
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Initializes the corpus_metrics table if it does not already exist.
    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS corpus_metrics (
                gutenberg_id UINTEGER PRIMARY KEY,
                title VARCHAR,
                author VARCHAR,
                author_birth_year INTEGER,
                author_death_year INTEGER,
                word_count UINTEGER,
                unique_word_count UINTEGER,
                sentence_count UINTEGER,
                avg_sentence_length DOUBLE,
                lexical_density DOUBLE,
                dialogue_ratio DOUBLE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
            [],
        )?;
        Ok(())
    }

    /// Upserts an AnalysisPayload row into the database.
    pub fn save_payload(&self, payload: &AnalysisPayload) -> Result<()> {
        self.conn.execute(
            "INSERT INTO corpus_metrics (
                gutenberg_id,
                title,
                author,
                author_birth_year,
                author_death_year,
                word_count,
                unique_word_count,
                sentence_count,
                avg_sentence_length,
                lexical_density,
                dialogue_ratio
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (gutenberg_id) DO UPDATE SET
                title = EXCLUDED.title,
                author = EXCLUDED.author,
                author_birth_year = EXCLUDED.author_birth_year,
                author_death_year = EXCLUDED.author_death_year,
                word_count = EXCLUDED.word_count,
                unique_word_count = EXCLUDED.unique_word_count,
                sentence_count = EXCLUDED.sentence_count,
                avg_sentence_length = EXCLUDED.avg_sentence_length,
                lexical_density = EXCLUDED.lexical_density,
                dialogue_ratio = EXCLUDED.dialogue_ratio;",
            params![
                payload.metadata.gutenberg_id,
                payload.metadata.title,
                payload.metadata.author,
                payload.metadata.author_birth_year,
                payload.metadata.author_death_year,
                payload.metrics.word_count as u32,
                payload.metrics.unique_word_count as u32,
                payload.metrics.sentence_count as u32,
                payload.metrics.avg_sentence_length,
                payload.metrics.lexical_density,
                payload.metrics.dialogue_ratio,
            ],
        )?;
        Ok(())
    }
}