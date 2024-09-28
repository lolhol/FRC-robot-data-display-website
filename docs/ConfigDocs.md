# Configuration Docs

## Two Configs - server and client

---

### SERVER_PORT

This project creates a backend server with **rust** that runs concurrently with the frontend client and server (launched by default by **NextJS**). Essentially, this is the port that the server will run on. Additionally, to retranslate the api requests to the **rust** server instead of the **NextJS** server, we need to know the local port that the server is running on.

Essentially, we created a proxy server in **rust** that will run on that port and then retranslate all the api requests going to the default server of **NextJS** to the **rust** server.

---

### NETWORK_TABLE_IP

The ip that the network table is running on. We need this because the network table can run on the robot that we are connected to - thus making it's ip different from localhost (127.0.0.1).

---

### NETWORK_TABLE_PORT

The port that the network table is running on. Essentially the port that the **rust** server will use to connect to the network server.

---

### TIME_BETWEEN_RECONNECT_ATTEMPTS

The amount of time between reconnect attempts. Essentially the amount of time that the **rust** server will wait before trying to reconnect to the network table. This can happen if the network table is not running yet.

---

### DATABASE_PATH

The path to the database file. This is the file that contains the data that the **rust** server will store in the local database. This file is created by the **SQLiteDatabase** class.

Essentially, we store tmp data that comes from the robot in this database and then retrieve it when client API HTTP requests come.

---

### DATABASE_MIN_TIME_AFTER_UPDATE

This accounts for the cleaning feature. Essentially, when you **clean** the database, it will remove all the entries who's time is smaller than
`current_time - DATABASE_MIN_TIME_AFTER_UPDATE`.

---
