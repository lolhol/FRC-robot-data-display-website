"use client";

import { useEffect, useState } from "react";

import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { MotorOutputChart } from "./components/MotorOutputChart";
import { NetworkTableBridge } from "types_plugin_frc_nw_table";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

export default function Home() {
  const [value, setValue] = useState<{ timestamp: number; output: number }[]>(
    []
  );
  const [startPolling, setStart] = useState<boolean>(false);

  useEffect(() => {
    if (!startPolling) return;

    const interval = setInterval(async () => {
      const response = await NetworkTableBridge.getEntryAndClean({
        topic: "motor_speed",
      });

      if (!response.Err && response.Ok) {
        const ok = response.Ok;
        console.log(
          JSON.stringify(response) +
            " ! " +
            parseFloat(ok.value) +
            " ! " +
            ok.timestamp
        );
        setValue((prevValue) => [
          ...prevValue,
          {
            timestamp: ok.timestamp,
            output: parseFloat(ok.value),
          },
        ]);
      }
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  }, [startPolling]);

  async function makeGetMotorOutputRequest() {
    const response = await fetch("/api/database/get-entry-and-clean", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ topic: "motor_speed" }),
    });

    return await response.json();
  }

  return (
    <main className="flex flex-col gap-y-20">
      <button
        className="w-60 h-40 bg-blue-500 m-5"
        onClick={async () => {
          setStart(true);
        }}
      >
        Start Motor Output Getter
      </button>
      <div className="bg-gray-700 w-3/4 mb-10 ml-5">
        <MotorOutputChart data={value} />
      </div>
    </main>
  );
}
