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

    pub fn get_values(
        &self,
        topic: &str,
        min_time_since_last_update: u32,
        max_count: u32,
    ) -> Result<Vec<TableEntree>, rusqlite::Error> {
        let mut stmt = self.connection.prepare(
            "SELECT value, timestamp FROM data WHERE topic = ? AND timestamp >= ? ORDER BY timestamp DESC LIMIT ?"
        )?;

        let rows = stmt.query_map(
            [
                topic,
                &(self.last_update as i32 - min_time_since_last_update as i32).to_string(),
                &max_count.to_string(),
            ],
            |row| {
                Ok(TableEntree {
                    topic: topic.to_string(),
                    value: row.get(0)?,
                    timestamp: row.get(1)?,
                })
            },
        )?;

        Ok(rows.collect::<Result<Vec<TableEntree>, rusqlite::Error>>()?)
    }

    pub fn get_values_no_time(
        &self,
        topic: &str,
        max_count: u32,
    ) -> Result<Vec<TableEntree>, rusqlite::Error> {
        self.get_values(topic, self.last_update, max_count)
    }

    pub fn get_value(&self, topic: &str) -> Result<TableEntree, rusqlite::Error> {
        Ok(self
            .get_values(topic, self.last_update, 1)?
            .get(0)
            .unwrap_or(&TableEntree::get_error())
            .clone())
    }

    pub fn add_value(&mut self, data: TableEntree) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "INSERT INTO data (topic, value, timestamp) VALUES (?, ?, ?)",
            [data.topic, data.value, data.timestamp.clone().to_string()],
        )?;

        self.last_update = data.timestamp.clone();

        Ok(())
    }

    /*pub fn add_value_cleaning(&mut self, data: TableEntree) -> Result<(), rusqlite::Error> {
        let _ = self.clean_database_time(self.min_time_between_cleans);
        let _ = self.add_value(data);

        Ok(())
    }*/

    pub fn clean_database(&self) -> Result<(), rusqlite::Error> {
        self.clean_database_time(self.min_time_between_cleans)
    }

    pub fn length(&self) -> Result<u32, rusqlite::Error> {
        let mut binding = self.connection.prepare("SELECT COUNT(*) FROM data")?;
        let mut stmt = binding.query([])?;
        Ok(stmt.next()?.unwrap().get(0)?)
    }

    pub fn topic_length(&self, topic: &str) -> Result<u32, rusqlite::Error> {
        let mut binding = self
            .connection
            .prepare("SELECT COUNT(*) FROM data WHERE topic = ?")?;
        let mut stmt = binding.query([topic])?;
        Ok(stmt.next()?.unwrap().get(0)?)
    }

    pub fn clear_database(&self) -> Result<(), rusqlite::Error> {
        self.connection.execute("DELETE FROM data", [])?;
        Ok(())
    }

    fn clean_database_time(&self, min_time_since_last_update: u32) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "DELETE FROM data WHERE timestamp <= ?",
            [(self.last_update - min_time_since_last_update).to_string()],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod utils {
        use std::fs::File;

        use super::*;

        pub fn get_database(min_time_between_cleans: u32) -> SQLiteDatabase {
            if !std::path::Path::new("test.db").exists() {
                let _ = File::create("test.db"); // create an empty file
            }

            let db = SQLiteDatabase::new("test.db", min_time_between_cleans).unwrap();
            let _ = db.clear_database();
            db
        }

        pub fn put_data_in_database(
            mut database: SQLiteDatabase,
            data_amt: u32,
            time_step: u32,
        ) -> SQLiteDatabase {
            for i in 0..data_amt {
                let _ = database.add_value(TableEntree::new(
                    "test".to_string(),
                    "test".to_string(),
                    i * time_step,
                ));
            }

            database
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_clean_and_new() {
        let mut database = utils::get_database(2);
        database
            .add_value(TableEntree::new("test".to_string(), "test".to_string(), 1))
            .unwrap();
        drop(database);

        let database = utils::get_database(2);

        assert_eq!(database.length().unwrap(), 0);
    }

    #[test]
    #[serial_test::serial]
    fn test_topic_length() {
        let mut database = utils::put_data_in_database(utils::get_database(2), 5, 1);
        database
            .add_value(TableEntree::new("test1".to_string(), "test".to_string(), 1))
            .unwrap();
        assert_eq!(database.topic_length("test").unwrap(), 5);
    }

    #[test]
    #[serial_test::serial]
    fn test_length() {
        let database = utils::put_data_in_database(utils::get_database(2), 5, 1);
        assert_eq!(database.length().unwrap(), 5);
    }

    #[test]
    #[serial_test::serial]
    fn test_add_value() {
        let mut database = utils::get_database(2);
        database
            .add_value(TableEntree::new("test".to_string(), "test".to_string(), 1))
            .unwrap();

        assert!(database.get_value("test").is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_get_values() {
        let database = utils::put_data_in_database(utils::get_database(2), 5, 1);
        let values = database.get_values("test", 2, 5).unwrap();
        assert_eq!(values.len(), 3);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_values_no_time() {
        let database = utils::put_data_in_database(utils::get_database(2), 5, 1);
        let values = database.get_values_no_time("test", 5).unwrap();
        assert_eq!(values.len(), 5);
    }

    #[test]
    #[serial_test::serial]
    fn test_clean_database() {
        let database = utils::put_data_in_database(utils::get_database(2), 5, 1);
        database.clean_database().unwrap();

        assert_eq!(database.length().unwrap(), 2);
    }
}
