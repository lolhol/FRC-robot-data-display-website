# Real-Time Robot Data Dashboard

![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)

This project enables developers to leverage the powerful combination of **Next.js**, **Rust**, and **WPILib** to visualize and monitor live robot data in real-time via a browser. By harnessing modern web development technologies like **HTML**, **CSS**, and **TypeScript**, this dashboard offers a robust, customizable solution for tracking robot performance and diagnostics.

This project stands out from other robot monitoring solutions by using **Next.js** to build a fast, responsive front-end and **Rust** for efficient, low-level back-end processing. This results in a streamlined experience that minimizes latency and enhances real-time data monitoring capabilities.

---

## How It Works

At the core of this project is WPILib's **NetworkTables** — a communication protocol commonly used in robotics (especially **FRC** robots) that allows different systems to share data in real time. This project retrieves robot data by connecting to **NetworkTables** via WebSockets and displaying it live on a web dashboard.

### Getting Started

To get started, you first need to set up the **NetworkTables** on your robot. The table acts as a shared space where the robot can publish data (such as motor speeds, sensor values, etc.), and the dashboard can subscribe to these updates and display them live.

### Robot Code Example

Below is an example of how you can configure your robot to publish motor speed data to the **NetworkTables**:

```java
new Thread(() -> {
    while (true) {
        try {
            NetworkTableEntry motorSpeedEntry = NetworkTableInstance.getDefault().getEntry("motor_speed");
            double simulatedSpeed = -1 + (2 * Math.random());
            motorSpeedEntry.setDouble(simulatedSpeed); // Update motor speed
            Thread.sleep(250); // Refresh every 250ms
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }
}).start();
```

In this example:

- **NetworkTableEntry**: A table entry that allows data to be exchanged.
- **motor_speed**: The key used to store and retrieve the motor speed in **NetworkTables**.
- The loop simulates random motor speeds and updates the table every 250 milliseconds.

### Accessing Data in the Frontend

The front-end application fetches data from **NetworkTables** by calling a backend API. The API returns the most recent value for a specified topic (e.g., motor speed). Below is an example of how you can make an API call to retrieve motor speed data using **TypeScript**:

```ts
async function makeGetMotorOutputRequest() {
  const response = await fetch("/api/database/get-entry", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ topic: "motor_speed" }),
  });

  const data = await response.json();
  return data;
}
```

In this code:

- **fetch**: Sends a request to the backend endpoint `/api/database/get-entry` to retrieve data.
- **POST**: The method used to send the request.
- **topic**: The key (`motor_speed`) that you want to query from the **NetworkTables**.

Once the data is retrieved, it can be displayed on the dashboard, offering real-time insights into robot performance.

---

## API Documentation

This project provides a well-defined API to interact with **NetworkTables** and other back-end services. To see a complete list of available endpoints and their usage, please refer to the [API Documentation](BackendAPISupportLists.md).

---

## Why Use This Project?

1. **Real-Time Monitoring**: Get live updates from the robot’s sensors, actuators, and other components with minimal latency.
2. **Flexible & Powerful**: Combines the power of **Next.js** with **Rust** for high performance and scalability.
3. **Developer Friendly**: Easy to extend and modify, making it suitable for a wide range of robotics applications.

---

## How to Set It Up

1. **Clone the Repository**: Clone this project to your local machine.
2. **Run the code**: IF YOU ARE ON **MAC**. _Support for linux / windows coming soon._

   ```bash
   bash Run.bash
   ```

3. **Configure NetworkTables**: Ensure your robot's code is set up to publish data to **NetworkTables** (as shown above).

4. **View the Dashboard**: Navigate to `http://localhost:3000` in your browser to start viewing live data from the robot.

---

## Contribution Guidelines

We welcome contributions! Feel free to submit a pull request or raise an issue if you encounter any problems or have suggestions for improvements. Please refer to our [Contribution Guide](CONTRIBUTING.md) for more details.

---
