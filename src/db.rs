use rusqlite::{params, Connection, OptionalExtension};
use anyhow::{Result, Context};
use chrono::Utc;

#[derive(Debug)]
pub struct IssuedKey {
    pub id: i64,
    pub user: String,
    pub key: String,
    pub key_hash: String,
    pub active: bool,
    pub replaced_by: Option<i64>,
    pub created_at: String,
}

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)
        .with_context(|| format!("Failed to open database '{}'", path))?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS issued_keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user TEXT NOT NULL,
            key TEXT NOT NULL,
            key_hash TEXT NOT NULL,
            active BOOLEAN NOT NULL DEFAULT 1,
            replaced_by INTEGER,
            created_at TEXT NOT NULL
        );"
    ).context("Failed to initialize schema")?;

    Ok(conn)
}

pub fn issue_key(conn: &Connection, user: &str, key: &str, key_hash: &str) -> Result<i64> {
    let exists: Option<i64> = conn.query_row(
        "SELECT id FROM issued_keys WHERE user = ?1 AND active = 1 LIMIT 1",
        params![user],
        |row| row.get(0),
    ).optional().context("DB query failed")?;

    if exists.is_some() {
        anyhow::bail!("User '{}' already has an active key", user);
    }

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO issued_keys (user, key, key_hash, active, created_at)
         VALUES (?1, ?2, ?3, 1, ?4)",
        params![user, key, key_hash, now],
    ).context("Failed to insert issued key")?;

    Ok(conn.last_insert_rowid())
}

pub fn rotate_key(conn: &Connection, user: &str, new_key: &str, key_hash: &str) -> Result<i64> {
    let old_id: Option<i64> = conn.query_row(
        "SELECT id FROM issued_keys WHERE user = ?1 AND active = 1 LIMIT 1",
        params![user],
        |row| row.get(0),
    ).optional().context("DB query failed")?;

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO issued_keys (user, key, key_hash, active, created_at)
         VALUES (?1, ?2, ?3, 1, ?4)",
        params![user, new_key, key_hash, now],
    ).context("Failed to insert rotated key")?;

    let new_id = conn.last_insert_rowid();

    if let Some(old_id) = old_id {
        conn.execute(
            "UPDATE issued_keys SET active = 0, replaced_by = ?1 WHERE id = ?2",
            params![new_id, old_id],
        ).context("Failed to deactivate old key")?;
    }

    Ok(new_id)
}

pub fn get_active_key(conn: &Connection, user: &str) -> Result<String> {
    conn.query_row(
        "SELECT key FROM issued_keys WHERE user = ?1 AND active = 1 LIMIT 1",
        params![user],
        |row| row.get(0),
    )
        .optional()
        .context("Failed to query active key")?
        .ok_or_else(|| anyhow::anyhow!("No active key found for user '{}'", user))
}

pub fn list_keys(conn: &Connection, show_all: bool) -> Result<Vec<IssuedKey>> {
    let sql = if show_all {
        "SELECT id, user, key, key_hash, active, replaced_by, created_at FROM issued_keys ORDER BY id DESC"
    } else {
        "SELECT id, user, key, key_hash, active, replaced_by, created_at FROM issued_keys WHERE active = 1 ORDER BY id DESC"
    };

    let mut stmt = conn.prepare(sql).context("Failed to prepare journal query")?;
    let rows = stmt.query_map([], |row| {
        Ok(IssuedKey {
            id: row.get(0)?,
            user: row.get(1)?,
            key: row.get(2)?,
            key_hash: row.get(3)?,
            active: row.get(4)?,
            replaced_by: row.get(5)?,
            created_at: formatted_date(&row.get(6)?),
        })
    }).context("Failed to map journal rows")?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.context("Failed to read journal row")?);
    }
    Ok(results)
}

fn formatted_date(created_at: &String) -> String {
    let created_formatted = chrono::DateTime::parse_from_rfc3339(&created_at)
        .map(|dt| dt.format("%Y-%m-%d:%H:%M:%S").to_string())
        .unwrap_or_else(|_| created_at.clone());
    created_formatted
}
