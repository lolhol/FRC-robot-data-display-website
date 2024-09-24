import { NextRequest } from "next/server";
import { getClient } from "../../../internal/app";
import WebSocket from "ws";

export async function POST(req: NextRequest) {
  const ws = new WebSocket("ws://127.0.0.1:8081", {
    perMessageDeflate: false,
  });
  ws.on("open", function open() {
    ws.send("something");
  });

  return new Response("Hello, Next.js!");
}
