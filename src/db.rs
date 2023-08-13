use rusqlite::{params, Connection, Result};

pub fn get_user_password(username: &str) -> Result<String> {
    let conn = Connection::open("your_database_path.sqlite3")?;
    let mut stmt = conn.prepare("SELECT password FROM user WHERE username = ?1")?;
    let stored_password: String = stmt.query_row(params![username], |row| row.get(0))?;
    Ok(stored_password)
}

// ... другие функции для работы с БД
pub fn save_message_hash(from: &str, to: &str, ipfs_hash: &str, conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO messages (from_user, to_user, ipfs_hash) VALUES (?1, ?2, ?3)",
        params![from, to, ipfs_hash],
    )?;
    Ok(())
}
