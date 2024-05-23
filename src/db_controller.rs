use rusqlite::{params, Connection, Result, OptionalExtension};
use std::collections::HashMap;

pub struct Entry {
    pub name: String,
    pub value: i32,
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(database_path: &str) -> Result<Self> {
        let connection = Connection::open(database_path)?;
        Ok(Database { connection })
    }

    pub fn initialise_database(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS entries (\
                key TEXT PRIMARY KEY,\
                value INTEGER\
                )",
            (),
        )?;
        let mut statement = self.connection.prepare("SELECT COUNT(*) FROM entries")?;
        let count: i32 = statement.query_row([], |row| row.get(0))?;

        if count == 0 {
            self.fill_db()?;
        }

        Ok(())
    }

    fn fill_db(&self) -> Result<()> {
        let mut hashmap: HashMap<String, i32> = HashMap::new();
        hashmap.insert("apple".to_string(), 10);
        hashmap.insert("banana".to_string(), 20);
        hashmap.insert("orange".to_string(), 15);

        for (key, value) in hashmap {
            self.connection.execute(
                "INSERT INTO entries (key, value) VALUES (?1, ?2)",
                params![key, value],
            )?;
        }
        Ok(())
    }

    pub fn get_all_entries(&self) -> Result<HashMap<String, i32>> {
        let mut statement = self.connection.prepare("SELECT key, value FROM entries")?;
        let iterator = statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        })?;

        let mut map = HashMap::new();
        for entry in iterator {
            let (key, value): (String, i32) = entry?;
            map.insert(key, value);
        }
        Ok(map)
    }

    pub fn get_entry(&self, name: &str) -> Result<Option<Entry>> {
        let mut stmt = self.connection.prepare("\
        Select name, price FROM groceries WHERE name = ?1")?;
        let entry = stmt.query_row(params![name], |row| {
            Ok(Entry {
                name: row.get(0)?,
                value: row.get(1)?,
            })
        }).optional()?;

        Ok(entry)
    }

    pub fn add_entry(&self, key: &str, value: i32) -> Result<()> {
        self.connection.execute(
            "INSERT INTO entries (key, value) VALUES (?1, ?2)\
            ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn remove_entry(&self, key: &str) -> Result<usize> {
        let deleted = self.connection.execute(
            "DELETE FROM entries WHERE name = ?1",
            params![key],
        )?;
        Ok(deleted)
    }
}
