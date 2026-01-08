# Rayon + Tokio Integration in Converge

This document explains how **Rayon** (CPU-bound parallelism) and **Tokio** (async I/O) work together in Converge, and the recommended architecture for the HTTP server layer.

---

## Current State: Rayon for Agent Execution

### Parallel Agent Execution (CPU-Bound)

**Current Implementation:**
```rust
// converge-core/src/engine.rs
fn execute_agents(
    &self,
    context: &Context,
    eligible: &[AgentId],
) -> Vec<(AgentId, AgentEffect)> {
    eligible
        .par_iter()  // ← Rayon parallel iterator
        .map(|&id| {
            let agent = &self.agents[id.0 as usize];
            let effect = agent.execute(context);  // CPU-bound work
            (id, effect)
        })
        .collect()
}
```

**What Rayon Does:**
- Executes agents in **parallel across CPU cores**
- Uses thread pool for CPU-bound work
- Agents read `&Context` (immutable) concurrently
- Each agent's `execute()` runs on a worker thread

**Characteristics:**
- ✅ **CPU-bound parallelism** (agent computation)
- ✅ **Synchronous** (blocking threads, not async)
- ✅ **Deterministic** (results collected in order)
- ✅ **Thread-safe** (agents only read immutable context)

---

## Future State: Tokio for HTTP Server

### Async I/O Layer (Network-Bound)

**Planned Implementation:**
```rust
// converge-runtime/src/main.rs (new crate)
use axum::{Router, Json};
use tokio::task;

async fn handle_job(Json(root_intent): Json<RootIntent>) -> Result<Json<ConvergeResult>> {
    // Spawn blocking task for CPU-bound engine work
    let result = task::spawn_blocking(move || {
        let mut engine = Engine::new();
        // Register agents...
        engine.run(context)  // ← Rayon runs here (blocking)
    })
    .await?;
    
    Ok(Json(result))
}
```

**What Tokio Does:**
- Handles **async I/O** (HTTP requests, LLM API calls)
- Manages concurrent connections
- Non-blocking network operations
- Efficient resource utilization

**Characteristics:**
- ✅ **I/O-bound concurrency** (HTTP, APIs)
- ✅ **Asynchronous** (non-blocking)
- ✅ **Scalable** (thousands of concurrent connections)

---

## How They Work Together

### Two-Layer Concurrency Model

```
┌─────────────────────────────────────────────────┐
│  Tokio Runtime (Async I/O)                      │
│  ┌───────────────────────────────────────────┐ │
│  │  HTTP Request 1                             │ │
│  │  ┌───────────────────────────────────────┐ │ │
│  │  │  spawn_blocking → Rayon Thread Pool  │ │ │
│  │  │  ┌──────────┐  ┌──────────┐          │ │ │
│  │  │  │ Agent 1  │  │ Agent 2  │  ...     │ │ │
│  │  │  │ (CPU)    │  │ (CPU)    │          │ │ │
│  │  │  └──────────┘  └──────────┘          │ │ │
│  │  └───────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────┘ │
│  ┌───────────────────────────────────────────┐ │
│  │  HTTP Request 2 (concurrent)              │ │
│  │  ┌───────────────────────────────────────┐ │ │
│  │  │  spawn_blocking → Rayon Thread Pool  │ │ │
│  │  │  ┌──────────┐  ┌──────────┐          │ │ │
│  │  │  │ Agent 1  │  │ Agent 2  │  ...     │ │ │
│  │  │  │ (CPU)    │  │ (CPU)    │          │ │ │
│  │  │  └──────────┘  └──────────┘          │ │ │
│  │  └───────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

### Execution Flow

1. **HTTP Request Arrives** (Tokio)
   - Axum handler receives request
   - Extracts `RootIntent` from JSON

2. **Spawn Blocking Task** (Tokio → Rayon)
   - `tokio::task::spawn_blocking()` moves CPU-bound work off async runtime
   - Prevents blocking Tokio threads
   - Rayon thread pool handles agent execution

3. **Parallel Agent Execution** (Rayon)
   - Agents execute in parallel across CPU cores
   - Each agent reads `&Context` (immutable)
   - Effects collected deterministically

4. **Merge & Return** (Tokio)
   - Blocking task completes
   - Result serialized to JSON
   - HTTP response sent

### Why This Works

**Separation of Concerns:**
- **Tokio**: Handles I/O (network, file system, timers)
- **Rayon**: Handles CPU-bound computation (agent execution)

**No Conflict:**
- Rayon runs **inside** `spawn_blocking()` (off async runtime)
- Tokio doesn't interfere with Rayon's thread pool
- Each layer uses appropriate concurrency model

**Performance:**
- Tokio handles thousands of concurrent connections
- Rayon utilizes all CPU cores for agent execution
- No blocking of async runtime by CPU work

---

## Recommended Architecture

### Separate Crate: `converge-runtime`

**Structure:**
```
converge-core/          # Library (pure, no HTTP)
├── src/
│   ├── engine.rs      # Uses rayon for agent execution
│   ├── context.rs
│   └── ...
└── Cargo.toml         # rayon dependency

converge-runtime/       # HTTP Server (new crate)
├── src/
│   ├── main.rs        # Axum server, tokio runtime
│   ├── handlers.rs    # HTTP handlers
│   ├── server.rs      # Server setup
│   └── ...
└── Cargo.toml         # axum, tokio dependencies
    [dependencies]
    converge-core = { path = "../converge-core" }
```

### Why Separate Crates?

**Benefits:**
1. **Clean Separation**
   - `converge-core` stays pure library (no HTTP dependencies)
   - Can be used in CLI tools, tests, other runtimes
   - Server concerns isolated

2. **Dependency Management**
   - `converge-core`: Only needs `rayon` for parallelism
   - `converge-runtime`: Adds `axum`, `tokio` for HTTP
   - No unnecessary dependencies in core

3. **Testability**
   - Core can be tested without HTTP stack
   - Server can be tested with mock core
   - Clear boundaries

4. **Flexibility**
   - Could add `converge-cli` (CLI tool using core)
   - Could add `converge-grpc` (gRPC server)
   - Core remains reusable

5. **Build Performance**
   - Faster incremental builds (core changes don't rebuild server)
   - Can publish core separately

### Alternative: Workspace Structure

If you prefer a workspace:

```
Cargo.toml              # Workspace root
├── converge-core/      # Library
└── converge-runtime/   # HTTP Server
```

**Workspace `Cargo.toml`:**
```toml
[workspace]
members = ["converge-core", "converge-runtime"]
resolver = "2"
```

---

## Implementation Pattern

### Handler with Blocking Task

```rust
// converge-runtime/src/handlers.rs
use axum::{Json, extract::State};
use tokio::task;
use converge_core::{Engine, Context, RootIntent, ConvergeResult};

pub async fn handle_job(
    State(agent_registry): State<AgentRegistry>,
    Json(root_intent): Json<RootIntent>,
) -> Result<Json<ConvergeResult>, Error> {
    // Move data into blocking task
    let registry = agent_registry.clone();
    
    // Spawn blocking task (moves to Rayon thread pool)
    let result = task::spawn_blocking(move || {
        let mut engine = Engine::new();
        
        // Register agents from registry
        for agent in registry.agents() {
            engine.register(agent);
        }
        
        // Run engine (uses Rayon internally for agent execution)
        let context = Context::from_root_intent(root_intent);
        engine.run(context)
    })
    .await  // Wait for blocking task
    .map_err(|e| Error::TaskJoin(e))?  // Handle join error
    .map_err(|e| Error::Converge(e))?;  // Handle engine error
    
    Ok(Json(result))
}
```

### Server Setup

```rust
// converge-runtime/src/main.rs
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize agent registry
    let registry = AgentRegistry::new();
    
    // Build router
    let app = Router::new()
        .route("/api/v1/jobs", post(handle_job))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(registry);
    
    // Start server
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

---

## Key Points

### 1. Rayon is Already Parallel ✅

**Current state:**
- Agents execute in parallel using `rayon::par_iter()`
- This is **CPU-bound parallelism** (uses all CPU cores)
- Works great for agent computation

**No changes needed** to agent execution model.

### 2. Tokio Adds Async I/O Layer

**Future state:**
- Tokio handles HTTP requests (async I/O)
- Uses `spawn_blocking()` to offload CPU work to Rayon
- Enables concurrent handling of multiple jobs

### 3. They Complement Each Other

**Rayon:**
- Parallel agent execution within a job
- CPU-bound work
- Synchronous (blocking threads)

**Tokio:**
- Concurrent job handling
- I/O-bound work (HTTP, APIs)
- Asynchronous (non-blocking)

### 4. Recommended Structure

**Separate crate: `converge-runtime`**
- Keeps `converge-core` pure library
- Clean separation of concerns
- Better testability and flexibility

---

## Migration Path

### Step 1: Create `converge-runtime` Crate

```bash
cd converge-core/..
cargo new --bin converge-runtime
```

### Step 2: Add Dependencies

```toml
# converge-runtime/Cargo.toml
[dependencies]
converge-core = { path = "../converge-core" }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Step 3: Implement Handlers

- Create `src/handlers.rs` with `handle_job()`
- Use `spawn_blocking()` for engine execution
- Return JSON responses

### Step 4: Update Justfile

```just
# Justfile
run-server:
    cd converge-runtime && cargo run

test-all:
    cd converge-core && cargo test
    cd converge-runtime && cargo test
```

---

## Summary

**Current State:**
- ✅ Rayon provides parallel agent execution (CPU-bound)
- ✅ Works perfectly for agent computation
- ✅ No HTTP server yet

**Future State:**
- ✅ Tokio handles HTTP requests (async I/O)
- ✅ `spawn_blocking()` bridges Tokio → Rayon
- ✅ Separate `converge-runtime` crate for HTTP server

**Architecture:**
- `converge-core`: Pure library, uses Rayon
- `converge-runtime`: HTTP server, uses Tokio + Axum
- Clean separation, better testability

**No Conflicts:**
- Rayon and Tokio work together seamlessly
- Each handles appropriate concurrency model
- `spawn_blocking()` is the bridge

---

## References

- [Rust Memory Model](../04-reference-comparisons/RUST_MEMORY_MODEL.md)
- [Engine Execution Model](../architecture/ENGINE_EXECUTION_MODEL.md)
- [Technology Stack](../deployment/TECHNOLOGY_STACK.md)
- [Deployment Guide](./DEPLOYMENT.md)

