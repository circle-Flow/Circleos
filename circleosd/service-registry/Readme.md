This README includes:

🧠 Concept overview and file tree (mind-map style)

⚙️ Build + Run instructions (with local setup and config)

📡 API & CLI examples

🔄 Integration details with other CircleOSD components

🧩 Best practices for service supervision

📄 service-registry/README.md
# 🧭 CircleOSD Service Registry

The **Service Registry** is the central orchestrator for **CircleOSD** — responsible for
managing, tracking, and supervising all running system services and daemons.

It acts like a lightweight **systemd** or **launchd** for CircleOSD microservices.

---

## 🧠 Concept Overview



circleosd/

├── service-registry/ # Central process supervisor

│ ├── src/

│ │ ├── main.rs # Boot entrypoint — starts registry daemon

│ │ ├── registry.rs # Core registry logic and APIs

│ │ ├── service.rs # Service definitions, state tracking

│ │ ├── process.rs # Process spawn/restart/monitor logic

│ │ └── health.rs # Health pings, service liveness checks

│ ├── Cargo.toml

│ └── README.md

│
├── auth-service/ # Auth microservice (user login)

├── plugin-manager/ # Dynamic plugin loader

├── core-daemon/ # Core orchestrator and IPC bridge

└── circlectl/ # CLI management tool


---

## ⚙️ Features

✅ Centralized service tracking and state management  
✅ Declarative `services.toml` configuration  
✅ Auto-restart of failed processes  
✅ Built-in health checks via IPC or heartbeat  
✅ JSON-RPC control interface (start, stop, list)  
✅ Optional persistent logs per service  

---

## 🧱 Architecture Diagram

```text
                   ┌──────────────────────────────┐
                   │          circlectl           │
                   │ (CLI control tool for admin) │
                   └─────────────┬────────────────┘
                                 │
                                 ▼
                ┌────────────────────────────────────┐
                │         Service Registry           │
                │   (supervises, monitors, restarts) │
                └────┬────────────┬────────────┬─────┘
                     │            │            │
     ┌───────────────┘            │            └──────────────┐
     ▼                            ▼                           ▼
auth-service               plugin-manager               core-daemon
(authentication)            (plugin system)            (system control)

⚙️ Configuration Layout

The service-registry reads its configuration from:

etc/
└── services.toml

Example services.toml
[[service]]
name = "auth-service"
cmd = ["./auth-service"]
restart = "Always"
health_check = "unix:/tmp/auth-service.sock"

[[service]]
name = "plugin-manager"
cmd = ["./plugin-manager"]
restart = "OnFailure"
health_check = "http://localhost:8000/health"

Supported restart policies
Policy	Description
Always	Restart whenever the process exits
OnFailure	Restart only on non-zero exit code
Never	Do not restart automatically
🛠️ Build & Run
🧩 Prerequisites

Rust 1.78+

Linux or macOS (Windows via WSL)

Config file: etc/services.toml

🏗️ Build
cd service-registry
cargo build --release

🚀 Run the Registry
cargo run --release


By default, it will:

Load configuration from ../etc/services.toml

Spawn each service as a supervised process

Create a Unix socket at /tmp/service-registry.sock for RPC commands

🧪 Commands & API

You can interact with the registry using circlectl (or any JSON-RPC client).

Example — List Services
echo '{"action":"list_services"}' | socat - UNIX-CONNECT:/tmp/service-registry.sock


Response:

{
  "ok": true,
  "data": [
    {"name":"auth-service","status":"Running"},
    {"name":"plugin-manager","status":"Stopped"}
  ]
}

Example — Restart a Service
echo '{"action":"restart","service":"auth-service"}' | socat - UNIX-CONNECT:/tmp/service-registry.sock

Example — Check Health
echo '{"action":"health"}' | socat - UNIX-CONNECT:/tmp/service-registry.sock

🧩 Integration with CircleOSD

The registry runs as the first system supervisor, spawning all essential microservices.

Component	Role
auth-service	User auth & session validation
plugin-manager	Loads plugins dynamically
core-daemon	Core system IPC and control
greeter	Login UI (calls auth-service)

The core-daemon or circlectl can communicate with it via /tmp/service-registry.sock.

🩺 Health Monitoring

The registry runs continuous health checks using configurable backends:

Type	Example	Description
unix:	unix:/tmp/auth-service.sock	Pings a local socket
http:	http://127.0.0.1:8080/health	Checks HTTP /health endpoint
cmd:	cmd:./scripts/check_auth.sh	Executes a custom command
🧠 Developer Mindmap

For clarity when navigating the codebase:

service-registry/
├── Cargo.toml
└── src/
    ├── main.rs          # Initializes tracing, loads config, starts registry
    ├── registry.rs      # In-memory registry of all running services
    ├── service.rs       # Service struct, status (Running, Failed, Restarting)
    ├── process.rs       # Process launcher + restart supervisor
    └── health.rs        # Health monitoring subsystem

🧰 Example Output (Logs)
[INFO] Service registry starting...
[INFO] Loaded 2 services from etc/services.toml
[INFO] Starting auth-service (PID 2341)
[INFO] Starting plugin-manager (PID 2342)
[INFO] Health OK: auth-service
[WARN] plugin-manager unhealthy, restarting...

🧾 License

This service is part of the CircleOSD Project.
Licensed under the MIT License.
See the root LICENSE file for details.

🔮 Future Roadmap

 Add persistent service state to var/registry.db

 Support dependency ordering (depends_on)

 Add metrics exporter (Prometheus)

 Support distributed service discovery (gRPC/Etcd)

 Integrate with circlectl status visualization

🧩 Troubleshooting
Issue	Fix
Socket already exists	Remove /tmp/service-registry.sock before starting
Permission denied	Ensure you have rights to /tmp or change socket path
Services not starting	Verify etc/services.toml paths are correct and binaries are built
No health data	Add health_check entries to each service in services.toml
📜 Quickstart Summary

Clone and Build

git clone https://github.com/<yourname>/circleosd.git
cd circleosd/service-registry
cargo build --release


Create config

mkdir -p ../etc
cp examples/services.toml ../etc/services.toml


Run

cargo run --release


Check running services

echo '{"action":"list_services"}' | socat - UNIX-CONNECT:/tmp/service-registry.sock


🧠 Tip:
For development, you can auto-reload configs:

cargo watch -x run

🪶 Maintainers
You (@adiytsuman24) — System Architect

CircleOSD Team — Platform maintainers
You (@yourgithub) — System Architect

CircleOSD Team — Platform maintainers
