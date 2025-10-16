use anyhow::Result;
use rusqlite::{params, Connection};
use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

/// Initialize the SQLite DB and create users table if missing.
pub fn init_db(path: &std::path::Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", &"WAL")?;
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        "#,
    )?;
    Ok(conn)
}

/// Insert a new user; returns inserted user id
pub fn insert_user(conn: &Connection, username: &str, password_hash: &str) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
        params![username, password_hash, now],
    )?;
    let id = conn.last_insert_rowid();
    Ok(id)
}

/// Get user by username
pub fn get_user_by_username(conn: &Connection, username: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, created_at FROM users WHERE username = ?1 LIMIT 1",
    )?;
    let mut rows = stmt.query(params![username])?;
    if let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let username: String = row.get(1)?;
        let password_hash: String = row.get(2)?;
        let created_at_str: String = row.get(3)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        Ok(Some(User {
            id,
            username,
            password_hash,
            created_at,
        }))
    } else {
        Ok(None)
    }
}
