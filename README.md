# HyperBus 🚀

**HyperBus** is a lightweight, file-backed message bus built in **Rust**.  
It’s inspired by systems like Apache Kafka and NATS — but built from scratch as part of my journey to learn Rust and systems programming.

## 💡 Overview
HyperBus allows producers to publish messages to **topics**, and consumers to subscribe and receive them in real time over **TCP**.

Unlike typical databases or brokers, HyperBus uses simple **append-only log files** per topic for persistence — making it a fun way to explore how commit logs, message queues, and async networking work under the hood.

## ⚙️ Goals
- Learn **Rust async**, **I/O**, and **concurrency** with `tokio`
- Implement a simple **TCP-based protocol**
- Store messages as **file-based commit logs**
- Support **topics**, **publish/subscribe**, and **acknowledgements**

## 🧩 Example Protocol (WIP)
```

PUB updates {"msg":"system rebooted"}
OK 1

SUB updates
MSG 1 {"msg":"system rebooted"}

```

## 🗂️ Project Structure
```

src/
├── main.rs          # Entry point & server setup
├── server.rs        # TCP listener
├── connection.rs    # Command parsing
├── broker.rs        # Topic registry & message routing
├── storage.rs       # File-based log storage
└── message.rs       # Message structure & serialization

```

## 🧠 Status
Early prototype — work in progress.  
The goal isn’t production readiness, but learning and exploring how message brokers work from the ground up.

## 🧰 Tech Stack
- 🦀 Rust  
- ⚡ Tokio (async runtime)  
- 🧾 Serde + JSON  
- 📁 File-based commit logs  