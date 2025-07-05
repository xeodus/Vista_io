use chrono::Utc;
use rusqlite::Connection;


pub fn init_db() -> Result<Connection, anyhow::Error> {
    let conn = Connection::open("vista.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT,
            prompt TEXT,
            response TEXT,
            pii_matches TEXT,
            model TEXT
            )",
            [],
    )?;
    Ok(conn)
}

pub fn log_interaction(
    conn: &Connection,
    prompt: &str,
    responses: &str,
    matches: &str,
    models: &str
) -> Result<(), anyhow::Error>
{
    let ts = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO logs (timestamp, prompt, responses, pii_matches, models) VALUES (?1, ?2, ?3, ?4, ?5)",
        [ts.as_str(), prompt, responses, matches, models]
    )?;

    Ok(())
}