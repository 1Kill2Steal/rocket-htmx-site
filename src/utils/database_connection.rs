// /src/utils/database_connection.rs

// Database

use rusqlite::{Connection, Result, Error};
use crate::User;

pub fn establish_connection() -> Result<Connection, Error> {
    // RELATIVE TO THE CRATE ROOT!!
    let db_connection = Connection::open("src/database/users.db")?;

    // To ensure the schema is set.
    let _ = db_connection
        .execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            password_salt TEXT NOT NULL
        )",
        [],
    )?;

    Ok(db_connection)
}

// Function to find a user by username in the database
pub fn find_user_by_username(conn_result: &Result<Connection, Error>, username_input: &str) -> Option<User> {
    let username = username_input.to_lowercase();

    let conn = match conn_result.as_ref() {
        Ok(conn) => conn,
        Err(_) => return None,
    };

    let mut stmt = match conn.prepare("SELECT id, username, password_hash FROM users WHERE username = ?1") {
        Ok(stmt) => stmt,
        Err(_) => return None,
    };

    let mut rows = match stmt.query([username]) {
        Ok(rows) => rows,
        Err(_) => return None,
    };

    let row = match rows.next() {
        Ok(Some(row)) => row,
        Ok(None) => return None, // No rows found
        Err(_) => return None,   // Return None on error
    };

    match (row.get(1), row.get(2)) {
        (Ok(username), Ok(password)) => {
            Some(User {
                username,
                password,
            })
        }
        _ => None,
    }

}
