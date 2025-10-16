# CircleOS Core Daemon (MVP)

Bolt is the core daemon for CircleOS (MVP). It manages services, plugins, IPC, telemetry and provides a secure HTTP API for other CircleOS components.

## Features (MVP)
- Service registry: register, start, stop, list services.
- Plugin manager: load external executable plugins and call them via JSON-RPC (stdin/stdout).
- HTTP API (Axum): /health, /services, /service/start, /service/stop, /plugins, /plugin/load, /plugin/call
- Simple token-based auth middleware
- Structured JSON logging via `tracing`

## Quickstart (local)
1. Build:
```bash
cargo build --release
