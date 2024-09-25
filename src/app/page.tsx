"use client";

import Image from "next/image";
import { useEffect, useRef, useState } from "react";
import { getLatestData } from "./api/init/action";

export default function Home() {
  const [value, setValue] = useState<string>("");

  useEffect(() => {
    let intervalLocal = setInterval(async () => {
      setValue((await getLatestData("double")) ?? "No data");
    }, 1000);

    return () => {
      clearInterval(intervalLocal);
    };
  }, []);

  return <main>{value}</main>;
}
