"use client";

import { useState } from "react";

export default function Home() {
  const [value, setValue] = useState<string>("");

  async function getLatestDataTopic(key: string) {
    return await (
      await fetch(`/api/database/get_entree/`, {
        method: "POST",
        body: JSON.stringify({ topic: key }),
      })
    ).json();
  }

  async function cleanDatabase() {
    await fetch(`/api/database/clean_database`, { method: "DELETE" });
  }

  return (
    <main className="flex flex-col gap-y-20">
      <button
        className="w-60 h-40 bg-blue-500"
        onClick={async () => {
          setValue(JSON.stringify(await getLatestDataTopic("/data/double")));
        }}
      >
        Get Topic Data
      </button>
      <button
        className="w-60 h-40 bg-red-500"
        onClick={async () => await cleanDatabase()}
      >
        Clean local table data.
      </button>
      <div className="flex flex-col gap-y-10">
        <a>Json Value Returned:</a>
        <p>{value}</p>
      </div>
    </main>
  );
}
