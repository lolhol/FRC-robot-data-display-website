# API Endpoints Documentation

## Database Operations

### `/api/database/clean-whole-database`

- **Method**: `DELETE`
- **Description**: This endpoint removes all entries from the database based on a configured retention time. It is typically used for maintenance purposes to clean out outdated or irrelevant data.

- **Code Example** (JavaScript/TypeScript):
  ```js
  await fetch("/api/database/clean-whole-database", {
    method: "DELETE",
  })
    .then((response) => response.json())
    .then((data) => console.log(data))
    .catch((error) => console.error("Error:", error));
  ```

---

### `/api/database/clear-database`

- **Method**: `DELETE`
- **Description**: Completely wipes the entire database. Use this endpoint with caution as it **permanently** deletes all data.

- **Code Example** (JavaScript/TypeScript):
  ```js
  await fetch("/api/database/clear-database", {
    method: "DELETE",
  })
    .then((response) => response.json())
    .then((data) => console.log(data))
    .catch((error) => console.error("Error:", error));
  ```

---

### `/api/database/get-entry`

- **Method**: `POST`
- **Request Body**:

  ```json
  {
    "topic": "String"
  }
  ```

- **Description**: Retrieves the latest entry for a specified topic from the database.
- **Response**: Returns the most recent value for the specified `topic`.
- **Error Response**:

  ```json
  {
    "topic": "ERROR",
    "value": "ERROR",
    "timestamp": -1
  }
  ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  const response = await fetch("/api/database/get-entry", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ topic: "motor_output" }),
  });

  const data = await response.json();
  console.log(data);
  ```

---

### `/api/database/get-entries`

- **Method**: `POST`
- **Request Body**:

  ```json
  {
    "topic": "String",
    "limit": "Int",
    "time_since_last_update": "Int (optional)"
  }
  ```

- **Description**: Retrieves multiple entries for the provided topic, with optional limits on how many entries to return and how recently they were updated.
- **Response**: Returns a list of the latest entries for the given `topic`.
- **Error Response**:

  ```json
  {
    "topic": "ERROR",
    "value": "ERROR",
    "timestamp": -1
  }
  ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  const response = await fetch("/api/database/get-entries", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      topic: "motor_output",
      limit: 10,
      time_since_last_update: 3600, // optional
    }),
  });

  const data = await response.json();
  console.log(data);
  ```

---

### `/api/database/get-entry-and-clean`

- **Method**: `POST`
- **Request Body**:

  ```json
  {
    "topic": "String"
  }
  ```

- **Description**: Retrieves the latest entry for the specified `topic` and **cleans** the associated entries from the database.
- **Response**: Returns the latest value for the specified `topic` and removes the corresponding data.
- **Error Response**:

  ```json
  {
    "topic": "ERROR",
    "value": "ERROR",
    "timestamp": -1
  }
  ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  const response = await fetch("/api/database/get-entry-and-clean", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ topic: "motor_output" }),
  });

  const data = await response.json();
  console.log(data);
  ```
