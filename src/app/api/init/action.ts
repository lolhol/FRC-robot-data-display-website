"use server";

import {
  getClient,
  getDatabaseBridge,
  isError,
  networkTableClient,
  startClient,
} from "@/internal/app";

export async function getLatestData(key: string) {
  if (!networkTableClient.isConnected()) {
    await networkTableClient.connect("127.0.0.1", 5000);
  }

  console.log("Fetching data from NetworkTables");

  const value = networkTableClient.getEntry(key as string);

  console.log("Data fetched:", value);

  return value?.value;
}
