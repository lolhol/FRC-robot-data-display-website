# How to use

## How to make a demo that **WORKS** and is tested by me...

1. Copy this code into your robot code (Robot.java for me). This will update the value every 1 second.

```java
@Override
public void robotInit() {
    NetworkTableInstance inst = NetworkTableInstance.create();
    inst.startServer("networktables.json", "127.0.0.1", 6000, 5000);

    NetworkTable table = inst.getTable("data");
    table.getEntry("double").setDouble(1.0);

    new Thread(() -> {
        while (true) {
            table.getEntry("double").setDouble(Math.random());
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
    }).start();
}
```

2. Run the "Run.bash" script in the root folder and accept anything it asks you. That will install (theoretically) everything that is needed and will run the actual backend and front end code (yes they are separate).
3. Go to localhost:3000 and you should be able to click the buttons with a response.

---

For a complete API documentation, please see the [API Documentation](BackendAPISupportLists.md)
