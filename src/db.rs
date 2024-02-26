use rusqlite::{Connection, Error, Result};
use bcrypt::{hash, DEFAULT_COST};

pub async fn new_user(name: &str, password: &str) -> Result<()> {
    let conn = Connection::open("test.db")?;

    // check if the table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;

    let hash = hash(password, DEFAULT_COST);

    match hash {
        Ok(hash) => {
            conn.execute(
                "INSERT INTO users (name, password) VALUES (?1, ?2)",
                &[name, &hash],
            )?;
        }
        Err(_) => {
            return Err(Error::InvalidParameterName("Invalid password".to_string()));
        }
    }

    Ok(())
}

pub async fn verify_user(name: &str, password: &str) -> Result<bool> {
    let conn = Connection::open("test.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT password FROM users WHERE name = ?1")?;
    let mut rows = stmt.query(&[name])?;

    if let Some(row) = rows.next()? {
        let hash: String = row.get(0)?;
        match bcrypt::verify(password, &hash) {
            Ok(true) => Ok(true),
            _ => Ok(false),
        }
    } else {
        Ok(false)
    }
}