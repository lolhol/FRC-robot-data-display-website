# API Endpoints Documentation

## Database Operations

### `/api/database/clean-whole-database`

- **Method**: `DELETE`
- **Description**: This endpoint cleans all entries in the SQLite database. It is primarily intended for maintenance purposes. If the operation fails due to internal database issues (such as a poisoned lock), an error is returned.

- **Responses**:

  - **Success**:

    - Returns a JSON object with a `DatabaseCleaningSuccess` status indicating the database has been successfully cleaned.
    - Example response:

      ```json
      {
        "Ok": "DatabaseCleaningSuccess"
      }
      ```

  - **Error**:

    - Returns an error if the database is poisoned or otherwise inaccessible.
    - Example response:

    ```json
    {
      "Err": {
        "DatabasePoisonedError": -1
      }
    }
    ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  await fetch("/api/database/clean-whole-database", {
    method: "DELETE",
  })
    .then((response) => response.json())
    .then((data) => console.log(data))
    .catch((error) => console.error("Error:", error));
  ```

- **Error Handling**:
  - **`DatabasePoisonedError(-1)`**: This error is returned when the `Mutex` guarding the database has been poisoned, usually because of a panic during a previous operation. It prevents further access to the database until resolved.
- **Success Response**:
  - **`DatabaseCleaningSuccess`**: This status is returned when the entire database has been successfully cleaned.

---

### `/api/database/clear-database`

- **Method**: `DELETE`
- **Description**: This endpoint clears all entries of the database. You can wipe the entire database with this method. If there is an issue with accessing the database due to internal errors, an error response is returned.

- **Responses**:

  - **Success**:

    - Returns a JSON object with a `DatabaseClearingSuccess` status indicating that the targeted portion of the database has been successfully cleared.
    - Example response:

      ```json
      {
        "Ok": "DatabaseClearingSuccess"
      }
      ```

  - **Error**:

    - Returns an error if the database is poisoned or locked.
    - Example response:

      ```json
      {
        "Err": {
          "DatabasePoisonedError": -1
        }
      }
      ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  await fetch("/api/database/clear-database", {
    method: "DELETE",
  })
    .then((response) => response.json())
    .then((data) => console.log(data))
    .catch((error) => console.error("Error:", error));
  ```

- **Error Handling**:
  - **`DatabasePoisonedError(-1)`**: Returned when the database lock is poisoned, preventing access to the database due to a previous panic or critical error.
- **Success Response**:
  - **`DatabaseClearingSuccess`**: This status is returned when a portion or specific entries in the database have been cleared successfully, without affecting the entire database.

---

### `/api/database/get-entry`

- **Method**: `POST`
- **Description**: This endpoint retrieves an entry from the SQLite database based on the provided `topic`. If the entry exists, the corresponding data is returned. If the entry does not exist or if an internal error occurs (such as a database lock failure), an appropriate response is returned.

- **Request Body**:

  - The request should contain a JSON object with a `topic` field specifying the topic to search for in the database.
  - Example request body:

    ```json
    {
      "topic": "example_topic"
    }
    ```

- **Responses**:

  - **Success**:

    - If the `topic` exists in the database, the response will contain the corresponding data.
    - If the `topic` does not exist, `None` will be returned.
    - Example response (entry found):

      ```json
      {
        "Ok": {
          "id": 1,
          "value": "example_value"
        }
      }
      ```

    - Example response (entry not found):

      ```json
      {
        "Ok": null
      }
      ```

  - **Error**:

    - If the database lock is poisoned or any other internal error occurs, an error is returned.
    - Example error response:

      ```json
      {
        "Err": {
          "DatabasePoisonedError": -1
        }
      }
      ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  await fetch("/api/database/get-entry", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      topic: "example_topic",
    }),
  })
    .then((response) => response.json())
    .then((data) => {
      if (data.Ok) {
        console.log("Entry found:", data.Ok);
      } else {
        console.log("Entry not found");
      }
    })
    .catch((error) => console.error("Error:", error));
  ```

- **Error Handling**:

  - **`DatabasePoisonedError(-1)`**: This error occurs if the database lock is poisoned, typically due to a previous panic that prevents safe access to the database.

- **Success Response**:
  - **`Some(TableEntree)`**: Returned when the `topic` exists in the database and its corresponding data is found.
  - **`None`**: Returned when the `topic` does not exist in the database.

---

### `/api/database/get-entries`

- **Method**: `POST`
- **Description**: This endpoint retrieves multiple entries from the SQLite database based on the provided `topic`. The number of entries retrieved is determined by the `amount` field, and an optional `time_since_last_update` filter can be applied to limit the results to entries updated after a specific timestamp. If the `amount` is missing or invalid, or if the database is poisoned due to a previous operation failure, an appropriate error is returned.

- **Request Body**:

  - The request should contain a JSON object with the following fields:
    - `topic`: (String) The topic to search for in the database.
    - `amount`: (Integer) The number of entries to retrieve.
    - `time_since_last_update`: (Optional Integer) A timestamp to filter entries based on their last update time.
  - Example request body:
    ```json
    {
      "topic": "example_topic",
      "amount": 5,
      "time_since_last_update": 1628475600 (optional)
    }
    ```

- **Responses**:

  - **Success**:

    - Returns a JSON array of `TableEntree` objects based on the `topic`, `amount`, and optional `time_since_last_update` filter.
    - Example response:

      ```json
      {
        "Ok": [
          {
            "id": 1,
            "value": "example_value_1"
          },
          {
            "id": 2,
            "value": "example_value_2"
          }
        ]
      }
      ```

    - If no entries are found, an empty array is returned:

      ```json
      {
        "Ok": []
      }
      ```

  - **Error**:

    - **`DatabasePoisonedError(-1)`**: Returned if the database lock is poisoned.
    - **`DatabaseInvalidAmountError(-1)`**: Returned if the `amount` field is missing or invalid in the request.

    - Example error response (database poisoned):

      ```json
      {
        "Err": {
          "DatabasePoisonedError": -1
        }
      }
      ```

    - Example error response (invalid amount):

      ```json
      {
        "Err": {
          "DatabaseInvalidAmountError": -1
        }
      }
      ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  await fetch("/api/database/get-entries", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      topic: "example_topic",
      amount: 5,
      time_since_last_update: 1628475600,
    }),
  })
    .then((response) => response.json())
    .then((data) => {
      if (data.Ok) {
        console.log("Entries found:", data.Ok);
      } else {
        console.log("No entries found or error occurred.");
      }
    })
    .catch((error) => console.error("Error:", error));
  ```

- **Error Handling**:

  - **`DatabasePoisonedError(-1)`**: This error occurs when the internal database lock is poisoned, typically due to a previous panic or critical failure.
  - **`DatabaseInvalidAmountError(-1)`**: This error is returned if the `amount` field is not provided or is invalid.

- **Success Response**:
  - A JSON array of `TableEntree` objects is returned, representing the entries that match the given `topic` and optional `time_since_last_update` filter.
  - An empty array is returned if no matching entries are found.

---

### `/api/database/get-entry-and-clean`

- **Method**: `POST`
- **Description**: This endpoint retrieves a single entry from the SQLite database based on the provided `topic`, and then cleans the database by removing old or unwanted entries. The entry matching the given `topic` is returned (if found), and the database is cleaned in the process. If the database lock is poisoned, an error is returned.

- **Request Body**:

  - The request should contain a JSON object with the following fields:
    - `topic`: (String) The topic to search for in the database.
  - Example request body:

    ```json
    {
      "topic": "example_topic"
    }
    ```

- **Responses**:

  - **Success**:

    - If the `topic` exists in the database, the entry is returned and the database is cleaned.
    - If no entry is found, `None` is returned, but the cleaning operation is still performed.
    - Example response (entry found):

      ```json
      {
        "Ok": {
          "id": 1,
          "value": "example_value"
        }
      }
      ```

    - Example response (entry not found):

      ```json
      {
        "Ok": null
      }
      ```

  - **Error**:

    - **`DatabasePoisonedError(-1)`**: Returned if the database lock is poisoned, meaning it cannot be accessed due to a previous error or panic.

    - Example error response:

      ```json
      {
        "Err": {
          "DatabasePoisonedError": -1
        }
      }
      ```

- **Code Example** (JavaScript/TypeScript):

  ```js
  await fetch("/api/database/get-entry-and-clean", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      topic: "example_topic",
    }),
  })
    .then((response) => response.json())
    .then((data) => {
      if (data.Ok) {
        console.log("Entry found and database cleaned:", data.Ok);
      } else {
        console.log("No entry found, but database cleaned.");
      }
    })
    .catch((error) => console.error("Error:", error));
  ```

- **Error Handling**:

  - **`DatabasePoisonedError(-1)`**: This error is returned when the database lock is poisoned, preventing safe access to the database. The error indicates that a previous panic occurred while accessing the database.

- **Success Response**:
  - **`Some(TableEntree)`**: Returned when the `topic` exists in the database and the corresponding entry is retrieved, followed by a successful database cleanup.
  - **`None`**: Returned when no entry is found for the given `topic`, but the database is still cleaned successfully.

---
