"use client";

import React, { useEffect } from "react";
import { Line } from "react-chartjs-2";
import {
  Chart,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";

Chart.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

export function MotorOutputChart(props: {
  data: {
    output: number;
    timestamp: number;
  }[];
}) {
  const [data, setData] = React.useState({
    labels: props.data.map((data) => data.timestamp),
    datasets: [
      {
        label: "Motor Output",
        data: props.data.map((data) => data.output),
        fill: false,
        borderColor: "rgba(75,192,192,1)",
        tension: 0.1,
      },
    ],
  });

  useEffect(() => {
    setData({
      labels: props.data.map((data) => data.timestamp),
      datasets: [
        {
          label: "Motor Output",
          data: props.data.map((data) => data.output),
          fill: false,
          borderColor: "rgba(75,192,192,1)",
          tension: 0.1,
        },
      ],
    });
  }, [props.data]);

  const options = {
    responsive: true,
    plugins: {
      legend: {
        position: "top" as const,
      },
      title: {
        display: true,
        text: "Motor Output Over Time",
      },
    },
    scales: {
      x: {
        title: {
          display: true,
          text: "Time",
        },
      },
      y: {
        title: {
          display: true,
          text: "Motor Output",
        },
        min: -1,
        max: 1,
      },
    },
  };

  console.log(data);

  return <Line width={500} height={300} data={data} options={options} />;
}
