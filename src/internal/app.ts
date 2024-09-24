import WebSocket from "ws";

let client: WebSocket | undefined = undefined;

export function getClient() {
  console.log("getClient");
  if (client === undefined) {
  }

  return client;
}
