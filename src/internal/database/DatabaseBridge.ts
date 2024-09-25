import Database from "better-sqlite3";

export class DatabaseBridge {
  database;

  constructor() {
    this.database = new Database("db/database.db");
    this.database
      .prepare("CREATE TABLE IF NOT EXISTS entries (key TEXT, value TEXT)")
      .run();
  }

  public getEntry(key: string) {
    try {
      const res: { value: string } = this.database
        .prepare("SELECT value FROM entries WHERE key = ?")
        .get(key) as { value: string };

      if (res) {
        this.database.prepare("DELETE FROM entries WHERE key = ?").run(key);

        return res.value;
      } else {
        console.warn(`No entry found for key: ${key}`);
        return null;
      }
    } catch (err) {
      console.error("Error fetching entry:", err);
      return null;
    }
  }

  public addToEntry(key: string, value: string) {
    try {
      const current: { value: string } = this.database
        .prepare("SELECT value FROM entries WHERE key = ?")
        .get(key) as { value: string };

      if (current) {
        const newValue = current.value + value;
        this.database
          .prepare("UPDATE entries SET value = ? WHERE key = ?")
          .run(newValue, key);
      } else {
        this.database
          .prepare("INSERT INTO entries (key, value) VALUES (?, ?)")
          .run(key, value);
      }
    } catch (err) {
      console.error("Error updating entry:", err);
    }
  }
}
