import { getClient } from "./internal/app";
import WebSocket from "ws";

export function register() {
  let is_error = true;

  console.log("WebSocket client created");

  setInterval(() => {
    if (is_error) {
      console.log("Re-trying WebSocket connection...");
      let client = new WebSocket(`ws://127.0.0.1:8000`);

      client.on("open", () => {
        console.log("WebSocket connection established");
        is_error = false;
        client?.send("Hello Server");
      });

      client.on("message", (message) => {
        console.log("Message from server:", message.toString());
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
