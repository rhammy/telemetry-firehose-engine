## 🛠 Project Requirements

### 1. The Data Protocol (Binary Specification)

Since you are in semiconductors, ignore JSON. Real-time tools use binary formats for speed and space.

* **Requirement:** Define a fixed-size binary packet structure (e.g., 32 or 64 bytes).
* **Fields:** Must include a `Timestamp` (nanoseconds), a `SensorID` (u32), a `ReadingType` (Enum), and the `Value` (f64).
* **Constraint:** Use **Zero-Copy parsing**. Your engine should be able to look at a slice of bytes (`&[u8]`) and "see" your struct without copying the data into a new memory location.

### 2. The Ingest Layer (Async Networking)

This is your "Front-End" that talks to the hardware (or simulator).

* **Requirement:** Create a UDP listener using an **Asynchronous Runtime**.
* **Capacity:** It must handle a "burst" of packets without dropping them. Look into setting the OS UDP buffer size.
* **Constraint:** The listener task must do **zero heavy lifting**. Its only job is to receive the packet, validate the checksum, and send it down the pipeline.

### 3. The Dispatcher (The Memory Bridge)

This is where most beginners fail and where you will learn the most about Rust's ownership.

* **Requirement:** Move packets from the "Async Networking" task to a "Processing Thread Pool."
* **Mechanism:** Use a **Bounded Channel**. If the processing engine is too slow, the channel fills up, creating "backpressure" that tells the networking layer to slow down (or log a warning).
* **Constraint:** Implement this using an **MPSC (Multi-Producer, Single-Consumer)** or a **Broadcast** pattern.

### 4. The Analytics Engine (Multi-Core Processing)

This is the "Back-End" where the heavy math happens.

* **Requirement:** Calculate a **Rolling Window Average** and **Standard Deviation** for 1,000+ unique sensors simultaneously.
* **Concurrency:** Use a dedicated thread pool (separate from the async executor) so that heavy CPU math doesn't "starve" the networking tasks of CPU time.
* **Constraint:** Use **Data Parallelism**. If you have 8 cores, all 8 should be saturated when a massive burst of data arrives.

---

## 🚀 Where to Start (The "No-AI" Roadmap)

### Step 1: Design the "Packet" (Day 1)

Don't write a single line of networking code yet. Define your data structure in a library file.

* **Research:** Look up `#[repr(C)]` and how it affects memory layout.
* **Goal:** Write a test that takes a `[u8; 64]` array and successfully casts/interprets it as your `SensorPacket` struct.

### Step 2: Build the "Generator" (Day 2)

Write a separate, tiny CLI tool that just loops and sends these binary packets over UDP as fast as possible. This is your "Simulated Wafer Machine."

* **Research:** `std::net::UdpSocket`. Keep it synchronous and simple.

### Step 3: The Async "Sink" (Day 3)

Now, create your actual engine. Try to get it to just *count* the packets it receives using an async listener.

* **Research:** **Tokio**. Specifically, `tokio::net::UdpSocket`.
* **The Struggle:** You will need to learn how to keep a "counter" across different async tasks. Do not ask AI for the answer; look up **`Arc<AtomicU64>`**.

### Step 4: The Hand-off (Day 4+)

This is the big one. Try to send that packet through a channel to a second task.

* **Research:** `tokio::sync::mpsc`.
* **The Struggle:** You will run into "Lifetime" errors. This is the "Welcome to Rust" moment. Read the compiler error, type `rustc --explain E0XXX` in your terminal, and fix it manually.

---

### Why this works

By starting with the **Packet Definition**, you fix the "schema" of your world. By building the **Generator**, you create a way to test your work. By the time you get to the **Multi-Core Analytics**, you will have such a strong grasp of how data flows through memory that the "complex" math will feel easy.

**Would you like me to find the specific documentation for "Zero-Copy" crates like `zerocopy` or `bytemuck` so you can see how to handle binary data safely?**