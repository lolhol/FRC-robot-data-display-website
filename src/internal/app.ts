import WebSocket from "ws";
import { DatabaseBridge } from "./database/DatabaseBridge";
import NetworkTableClient from "./nw-table/NetworkTables";

let client: WebSocket | undefined = undefined;
let databaseBridge: DatabaseBridge | undefined = undefined;
let is_error = false;
let init = true;

export const networkTableClient = new NetworkTableClient();

export function getDatabaseBridge() {
  if (!databaseBridge) {
    databaseBridge = new DatabaseBridge();
  }

  return databaseBridge;
}

export function isError() {
  return is_error;
}

export function getClient() {
  return client;
}

export function startClient() {
  console.log("WebSocket client created");

  setInterval(() => {
    if (is_error || init) {
      init = false;
      console.log("Re-trying WebSocket connection...");
      client = new WebSocket(`ws://127.0.0.1:${process.env.SERVER_PORT}`);

      client.on("open", () => {
        console.log("WebSocket connection established");
        is_error = false;
        client?.send("Hello Server");
      });

      client.on("message", (message) => {
        console.log("Message from server:", message.toString());
        let data = JSON.parse(message.toString());
        getDatabaseBridge().addToEntry(data.topic_name, data.data);
      });

      client.on("error", (error) => {
        is_error = true;
        console.error("WebSocket error:", error);
      });

      client.on("close", () => {
        is_error = true;
      });
    }
  }, 1000);
}
