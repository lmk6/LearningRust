use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

pub fn initialise_database(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS entries (\
                key TEXT PRIMARY KEY,\
                value INTEGER\
                )",
        (),
    )?;
    let mut statement = connection.prepare("SELECT COUNT(*) FROM entries")?;
    let count: i32 = statement.query_row([], |row| row.get(0))?;

    if count == 0 {
        fill_db(connection)?;
    }

    Ok(())
}

fn fill_db(connection: &Connection) -> Result<()> {
    let mut hashmap: HashMap<String, i32> = HashMap::new();
    hashmap.insert("apple".to_string(), 10);
    hashmap.insert("banana".to_string(), 20);
    hashmap.insert("orange".to_string(), 15);

    for (key, value) in hashmap {
        connection.execute(
            "INSERT INTO entries (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
    }
    Ok(())
}

pub fn load_hashmap_from_db(connection: &Connection, hash_map: &mut HashMap<String, i32>) -> Result<()> {
    let mut statement = connection.prepare("SELECT key, value FROM entries")?;
    let iterator = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
    })?;

    for entry in iterator {
        let (key, value): (String, i32) = entry?;
        hash_map.insert(key, value);
    }
    Ok(())
}

pub fn add_entry_to_db(connection: Connection, key: &str, value: i32) -> Result<()> {
    connection.execute(
        "INSERT INTO entries (key, value) VALUES (?1, ?2)\
            ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}
