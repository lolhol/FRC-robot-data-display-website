use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;
use structs::table_entree::TableEntree;

pub mod structs;

#[derive(Debug)]
pub struct SQLiteDatabase {
    connection: Connection,
    last_update: u32,
    min_time_between_cleans: u32,
}

impl SQLiteDatabase {
    pub fn new(file: &str, min_time_between_cleans: u32) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(file)?;
        let _ = connection.execute(
            "CREATE TABLE IF NOT EXISTS data (topic TEXT, value TEXT, timestamp INTEGER)",
            [],
        );

        Ok(SQLiteDatabase {
            connection,
            last_update: 0,
            min_time_between_cleans: min_time_between_cleans,
        })
    }

    pub fn get_value(&self, topic: &str) -> Result<TableEntree, rusqlite::Error> {
        println!("Getting value for topic: {}", topic);
        self.connection.query_row(
            "SELECT value, timestamp FROM data WHERE topic = ?1",
            [topic],
            |row| {
                Ok(TableEntree {
                    topic: topic.to_string(), // `topic` is provided as input
                    value: row.get(0)?,       // `value` is in the first column of the result
                    timestamp: row.get(1)?,   // `timestamp` is in the second column of the result
                })
            },
        )
    }

    pub fn add_value(&mut self, data: TableEntree) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "INSERT INTO data (topic, value, timestamp) VALUES (?, ?, ?)",
            [data.topic, data.value, data.timestamp.clone().to_string()],
        )?;

        self.last_update = data.timestamp.clone();

        Ok(())
    }

    pub fn add_value_cleaning(&mut self, data: TableEntree) -> Result<(), rusqlite::Error> {
        self.clean_database_time(self.min_time_between_cleans);
        self.add_value(data);

        Ok(())
    }

    pub fn clean_database(&self) -> Result<(), rusqlite::Error> {
        self.clean_database_time(self.min_time_between_cleans)
    }

    fn clean_database_time(&self, min_time_since_last_update: u32) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "DELETE FROM data WHERE timestamp < ?",
            [(self.last_update - min_time_since_last_update).to_string()],
        )?;

        Ok(())
    }
}
