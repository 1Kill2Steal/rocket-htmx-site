// /src/utils/database_connection.rs

// Database

use rusqlite::{Connection, Result};

pub fn establish_connection() -> Result<Connection> {
    // /src/database/...
    let db_connection = Connection::open("../database/users.db")?;

    // To ensure the schema is set.
    let _ = create_user_table(&db_connection)?;

    Ok(db_connection)
}

// Not needed.
fn create_user_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}
