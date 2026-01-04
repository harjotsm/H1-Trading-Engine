# H1 Trading Engine

A high-performance trading matching engine built from scratch in Rust, designed to understand the core mechanics of how
modern cryptocurrency exchanges handle order execution and price discovery.

## 🎯 Overview

This project implements a custom order matching engine with the following capabilities:

- **Order Books** - Manages buy (bid) and sell (ask) orders for trading pairs using efficient data structures
- **Limit Orders** - Execute only at a specified price or better, with automatic matching against the orderbook
- **Price-Time Priority** - Orders are matched based on price priority first, then time priority (FIFO)
- **Real-time Matching** - Instant order matching with detailed trade event reporting
- **Performance Optimized** - Built with Rust's zero-cost abstractions and benchmarked with Criterion

## 🏗️ Architecture

### Core Components

#### MatchingEngine (`src/matching_engine/engine.rs`)

The main engine that manages multiple trading pairs and routes orders to the appropriate orderbook.

```rust
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}
```

#### Orderbook (`src/matching_engine/orderbook.rs`)

The heart of the matching logic, maintaining separate bid and ask limit queues using BTreeMaps for efficient price-level
lookups.

```rust
pub struct Orderbook {
    asks: BTreeMap<u64, Limit>,    // Sell orders (ascending price)
    bids: BTreeMap<u64, Limit>,    // Buy orders (descending price)
    next_order_id: u64,
}
```

#### Limit

A price level containing a queue of orders at that specific price point.

```rust
pub struct Limit {
    price: u64,
    orders: VecDeque<Order>,
}
```

### Order Matching Algorithm

1. **Taker Order Arrives** - A new limit order enters the system
2. **Price Matching** - The engine checks if there are matching orders on the opposite side
    - For buy orders: match against asks with price ≤ bid price
    - For sell orders: match against bids with price ≥ ask price
3. **Execution** - Fill orders in price-time priority until:
    - The taker order is fully filled, or
    - No more matching orders exist
4. **Resting Order** - If quantity remains unfilled, add to the orderbook as a maker order

### Event System

The engine produces detailed match events:

- `MatchEvent::Trade` - A trade occurred between maker and taker
- `MatchEvent::Maker` - Remaining quantity added to the orderbook

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (using edition 2024)
- Cargo

### Running the Demo

```bash
cargo run
```

This executes a sample trading scenario with multiple orders and displays trade executions.

### Running Benchmarks

```bash
cargo bench
```

Performance benchmarks test orderbook throughput with 1000 random orders. Results are available in
`target/criterion/report/index.html`.

### Building for Release

```bash
cargo build --release
```

## 📊 Current Status

✅ **Phase 1: Core Matching Engine** - COMPLETE

- [x] Orderbook implementation with BTreeMap
- [x] Limit order matching logic
- [x] Price-time priority algorithm
- [x] Event-driven architecture
- [x] Basic trading pair support
- [x] Performance benchmarking suite

## 🗺️ Roadmap - Next Steps

### 🚧 Phase 2: Asynchronous TCP Server

Build a production-grade TCP server to host the matching engine and handle concurrent client connections.

**Key Challenges:**

- **Async Runtime** - Integrate `tokio` for high-performance async I/O
- **TCP Protocol** - Implement a custom protocol layer since TCP transmits raw bytes
    - Use JSON for message format (`serde_json`)
    - Implement length-delimited framing to handle message boundaries
- **Thread Safety** - Make the matching engine thread-safe for concurrent access
    - Wrap engine in `Arc<Mutex<MatchingEngine>>` or explore lock-free alternatives
- **Connection Management** - Handle multiple simultaneous client connections
- **Error Handling** - Graceful handling of network errors and disconnections

**Technical Stack:**

- `tokio` - Async runtime
- `serde` + `serde_json` - JSON serialization
- `tokio-util` - Codec utilities for framing

### ⏳ Phase 3: CLI Trading Client (REPL)

Build an interactive command-line client that allows users to trade without restarting the application.

**Features:**

- Interactive REPL (Read-Eval-Print Loop)
- Natural command syntax: `buy 100 BTC @ 50000` or `sell 50 BTC @ 51000`
- Real-time response display
- Connection to the TCP server
- Command history and autocomplete

**Technical Stack:**

- `tokio` - Network communication
- `rustyline` - Advanced line editing and history
- `clap` or custom parser - Command parsing

**Example Session:**

```
> connect localhost:8080
Connected to trading engine
> buy 1.5 BTC @ 50000
✓ Order placed: ID #1234
⚡ Partial fill: 1.0 BTC @ 50000
📝 Remaining: 0.5 BTC on book
> orderbook BTC_USD
...
```

### ⏳ Phase 4: Snapshotting & Persistence

Implement state persistence to survive server crashes and enable recovery.

**Requirements:**

- Periodic snapshots of orderbook state
- Write-ahead logging (WAL) for crash recovery
- Efficient serialization of orderbook data structures
- Automatic recovery on server restart

**Technical Stack:**

- `serde` - Serialization/Deserialization
- File-based or database-backed storage
- Snapshot scheduling with configurable intervals

**Implementation Ideas:**

- Snapshot format: JSON or binary (bincode)
- Incremental snapshots to reduce I/O overhead
- Replay mechanism for orders between snapshots

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## 📈 Performance

The current implementation focuses on correctness and readability. Future optimizations may include:

- Lock-free data structures
- Memory pooling for order allocation
- SIMD optimizations for matching logic
- Custom allocators

## 🛠️ Technology Stack

- **Language**: Rust (edition 2024)
- **Dependencies**:
    - `rand` - Random number generation for testing
    - `criterion` - Benchmarking framework
