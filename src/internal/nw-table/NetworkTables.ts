import WebSocket from "ws";
import { EventEmitter } from "events";

interface NetworkTableEntry {
  key: string;
  value: any;
  type: string;
}

class NetworkTablesInstance {
  private static _defaultInstance: NetworkTablesInstance | null = null;
  private ws: WebSocket | null = null;
  private tables: Map<string, NetworkTableEntry> = new Map();
  private entryListeners: Map<string, Set<Function>> = new Map();
  private connListeners: Set<Function> = new Set();
  private emitter: EventEmitter = new EventEmitter();

  // Get default instance (singleton)
  public static getDefault(): NetworkTablesInstance {
    if (!this._defaultInstance) {
      this._defaultInstance = new NetworkTablesInstance();
    }
    return this._defaultInstance;
  }

  // Create a new instance
  public static create(): NetworkTablesInstance {
    return new NetworkTablesInstance();
  }

  // Connect to a NetworkTables server
  public connect(address: string, port: number): Promise<void> {
    return new Promise((resolve, reject) => {
      this.ws = new WebSocket(`ws://${address}:${port}/nt/1`);

      this.ws.on("open", () => {
        console.log("Connected to NetworkTables");
        resolve();
      });

      this.ws.on("message", (data) => {
        console.log("Message from server:", data.toString());
        const message = JSON.parse(data.toString());
        if (message.type === "entry") {
          this.handleEntryMessage(message);
        }
        this.emitter.emit("message", message);
      });

      this.ws.on("error", (error) => {
        console.error("WebSocket error:", error);
        reject(error);
      });

      this.ws.on("close", () => {
        console.log("Disconnected from NetworkTables");
      });
    });
  }

  // Handle incoming entry messages
  private handleEntryMessage(message: any): void {
    console.log("Received entry message:", message);
    const { key, value } = message;
    this.tables.set(key, { key, value, type: typeof value });

    const listeners = this.entryListeners.get(key);
    if (listeners) {
      listeners.forEach((listener) => listener(key, value));
    }
  }

  // Get a specific entry
  public getEntry(key: string): NetworkTableEntry | undefined {
    return this.tables.get(key);
  }

  // Get all entries that match a prefix
  public getEntries(prefix: string): NetworkTableEntry[] {
    const entries: NetworkTableEntry[] = [];
    for (const [key, entry] of this.tables.entries()) {
      if (key.startsWith(prefix)) {
        entries.push(entry);
      }
    }
    return entries;
  }

  // Add an entry listener
  public addEntryListener(
    key: string,
    listener: (key: string, value: any) => void,
    immediateNotify: boolean = false
  ): void {
    if (!this.entryListeners.has(key)) {
      this.entryListeners.set(key, new Set());
    }

    const listeners = this.entryListeners.get(key)!;
    listeners.add(listener);

    if (immediateNotify) {
      const entry = this.getEntry(key);
      if (entry) {
        listener(entry.key, entry.value);
      }
    }
  }

  // Remove an entry listener
  public removeEntryListener(key: string, listener: Function): void {
    const listeners = this.entryListeners.get(key);
    if (listeners) {
      listeners.delete(listener);
    }
  }

  // Add a connection listener
  public addConnectionListener(listener: (connected: boolean) => void): void {
    this.connListeners.add(listener);
  }

  // Remove a connection listener
  public removeConnectionListener(listener: Function): void {
    this.connListeners.delete(listener);
  }

  // Set a value in the network table
  public setValue(key: string, value: any): void {
    if (this.ws) {
      const message = { key, value, type: typeof value };
      this.ws.send(JSON.stringify(message));
      this.tables.set(key, { key, value, type: typeof value });
    } else {
      console.error("Not connected to NetworkTables");
    }
  }

  // Get the list of current connections
  public getConnections(): string[] {
    // Placeholder for connection info. Add logic to return actual connections if necessary.
    return [];
  }

  // Check if the client is connected
  public isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  // Disconnect from the NetworkTables server
  public disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}

export default NetworkTablesInstance;
