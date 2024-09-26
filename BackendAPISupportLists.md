# API Endpoints Documentation

## Database Operations

### /api/database/clean-whole-database

- **Method:** DELETE
- **Description:** Cleans the entire database based on a configured time value.

---

### /api/database/clear-database

- **Method:** DELETE
- **Description:** Deletes all data from the database.

---

### /api/database/get-entree

- **Method:** POST
- **Request Fields:**

  ```json
  {
    "topic": "String"
  }
  ```

- **Response:**

  - Returns the latest value for the provided topic.

- **Error Response:**
  ```json
  {
    "topic": "ERROR",
    "value": "ERROR",
    "timestamp": -1
  }
  ```

---

### /api/database/get-entries

- **Method:** POST
- **Request Fields:**

  ```json
  {
    "topic": "String",
    "limit": "Int",
    "time_since_last_update": "Int (optional)"
  }
  ```

- **Response:**

  - Returns the latest entries for the provided topic.

- **Error Response:**
  ```json
  {
    "topic": "ERROR",
    "value": "ERROR",
    "timestamp": -1
  }
  ```

---

### /api/database/get-entree-and-clean

- **Method:** POST
- **Request Fields:**

  ```json
  {
    "topic": "String"
  }
  ```

- **Response:**

  - Returns the latest value for the provided topic and cleans the associated entries from the database.

- **Error Response:**

  ```json
  {
    "topic": "ERROR",
    "value": "ERROR",
    "timestamp": -1
  }
  ```

---
