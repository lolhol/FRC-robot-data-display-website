# Methodology

## Launch Process

### Overview

```mermaid
graph LR;
    A[Run.bash] --> B[Get Operation System];
    B --> C[MacOS];
    B --> D[Windows];
    B --> E[Linux];
    C --> |for mac| F[Build and Run Project];
    D --> |for windows| F[Build and Run Project];
    E --> |for linux| F[Build and Run Project];
```

### Launch Process MacOS

```mermaid
graph LR;
    C[Install Node.js] --> E[Install Cargo];
    E --> F[Install Frontend deps];
    F --> G[Build Backend];
    G --> H[Launch Frontend and Backend];
```

### Launch Process Linux

At the moment not supported

### Launch Process Windows

At the moment not supported

## Backend Process

```mermaid
graph LR;
    R[Robot] --> |data| T[Add to Network Table];

    A[Start] --> B[Start Network Table Server];
    B --> C[Start Backend Server];

    D[Network Table Server] --> E[Check Connection to Robot Network Table];
    E --> F[Process];
    F --> |Can't connect| E;
    F --> G[Subscribe to Updates];

    DB[Internal Database];

    H[On Update] --> I[Add to local DB] --> |add| DB;

    K[Client] --> |req| J[On API Request];
    DB <--> |get| J;
```
