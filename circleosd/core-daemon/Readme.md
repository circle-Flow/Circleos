
# 🌀 CircleOSD – Core Daemon, Service Registry & Auth System (Rust)

CircleOSD is a **modular, Rust-based system daemon** that acts as the *core* of a next-generation operating environment.  
It provides:

- ⚙️ **Service registry & process management**
- 🔐 **Authentication service (users, sessions)**
- 🔌 **Plugin manager** (native or WASM-based)
- 💬 **JSON RPC socket interface**
- 🧩 A clean foundation for building OS-level services, shells, games, and extensions

---

## 🧱 Architecture Overview

CircleOSD is structured as a collection of cooperating Rust crates, with the `core-daemon` at the center.


### Core responsibilities

| Component | Description |
|------------|-------------|
| **core-daemon** | The system heart — handles service management, authentication, plugins, and RPC interface. |
| **service_registry** | Manages service definitions, status, and lifecycle. |
| **auth_service** | Stores users, passwords, and session tokens (Argon2-based). |
| **plugin_manager** | Loads and verifies plugins (native or sandboxed). |
| **rpc** | JSON-RPC protocol over Unix socket `/tmp/circleosd.sock`. |
| **config** | Loads configuration from `/etc/circleosd.conf` (or defaults). |

---

## ⚡ Getting Started

### 1️⃣ Prerequisites

- Rust (≥ 1.70)
- Cargo
- Linux / macOS (Unix sockets supported)
- Optional: `socat` for quick RPC testing

### 2️⃣ Build

```bash
git clone https://github.com/yourname/circleosd.git
cd circleosd/core-daemon
cargo build --release

Run the Daemon
cargo run --release


You’ll see logs like:

Starting circleosd core daemon...
Listening for RPC on /tmp/circleosd.sock

💬 RPC Interface (JSON over Unix Socket)

CircleOSD uses a simple JSON-RPC style protocol over a local Unix socket (/tmp/circleosd.sock).

Each request must be a single line of JSON ending with \n.
Responses are JSON objects with ok, message, and optional data.

Example commands (using socat)
➕ Create a user
echo '{"action":"create_user","username":"alice","password":"secret"}' \
  | socat - UNIX-CONNECT:/tmp/circleosd.sock


Response:

{"ok":true,"message":"user created"}

🔑 Authenticate user
echo '{"action":"auth","username":"alice","password":"secret"}' \
  | socat - UNIX-CONNECT:/tmp/circleosd.sock


Response:

{"ok":true,"message":"authenticated"}

🧩 Register a service
echo '{"action":"register_service","name":"mydaemon","cmd":["/usr/bin/mydaemon"]}' \
  | socat - UNIX-CONNECT:/tmp/circleosd.sock

📜 List all services
echo '{"action":"list_services"}' | socat - UNIX-CONNECT:/tmp/circleosd.sock


Response:

{
  "ok": true,
  "data": [
    {"name":"mydaemon","cmd":["/usr/bin/mydaemon"],"running":false}
  ]
}

🚀 Start a service
echo '{"action":"start_service","name":"mydaemon"}' | socat - UNIX-CONNECT:/tmp/circleosd.sock

🧩 Load a plugin
echo '{"action":"load_plugin","path":"./plugins/example_plugin.wasm"}' \
  | socat - UNIX-CONNECT:/tmp/circleosd.sock

🔐 Authentication

CircleOSD uses Argon2 for password hashing and secure storage in memory.
Future versions will support:

Persistent user DB (SQLite or encrypted JSON)

Session tokens (JWT / short-lived tokens)

2FA via TOTP

⚙️ Configuration

Configuration defaults are defined in src/config.rs, but you can create /etc/circleosd/circleosd.conf like this:

[core]
socket = "/run/circleosd.sock"
log_level = "info"

[auth]
autologin = false
user_db = "/var/lib/circleosd/users.db"

[plugins]
paths = ["./plugins"]

📂 Project File Layout
core-daemon/

├── Cargo.toml

└── src/

    ├── main.rs               # Entry point

    ├── auth.rs               # Auth service (Argon2)

    ├── service_registry.rs   # Register/start/stop services

    ├── plugin.rs             # Plugin manager

    ├── rpc.rs                # RPC request handler

    └── config.rs             # Config loader

🧠 Development Notes

Written fully in async Rust using tokio.

IPC via Unix sockets for low-latency, secure communication.

Uses serde_json for lightweight JSON-RPC.

Modular: you can replace or extend any subsystem.

Perfect for building:

lightweight system daemons,

experimental OS prototypes,

embedded or game-oriented service shells.

🚧 Roadmap
Milestone	Description
✅ Core daemon skeleton	Basic service registry, plugin loader, and auth.
🔜 Persistent user DB	SQLite-based credential storage.
🔜 CLI tool (circlectl)	User and service management CLI.
🔜 Plugin sandboxing	Run plugins safely via WASM.
🔜 Greeter UI	Simple TUI/GUI login experience.
🔜 Package manager	For installing apps/games and system services.
🧩 Extending CircleOSD

Add your own plugins under plugins/:

cargo new --lib plugins/example_plugin


Implement a Rust library with functions you want the daemon to load,
or build a WASM plugin using wasmtime.

🕹️ Building Apps & Games

You can develop Rust apps or games that run within CircleOSD.
Recommended engines:

Bevy
 – ECS-based game engine.

wgpu
 – Modern graphics API.

macroquad
 – Simple 2D game framework.

Games can be installed under /opt/circleosd/apps/ and registered as services or plugins.

🧰 Tools & Commands (coming soon)
Command	Description
circlectl user add <name>	Add a new user
circlectl auth <name>	Authenticate a user
circlectl service start <svc>	Start a registered service
circlectl plugin load <path>	Load a plugin manually
circlectl status	Show running services
🧪 Testing

Run all unit tests:

cargo test


Run the daemon in debug mode with verbose logs:

RUST_LOG=debug cargo run

🧑‍💻 Contributing

Fork and clone the repo

Create a feature branch: git checkout -b feature/your-feature

Commit your changes

Open a pull request!

We welcome contributions to improve system stability, plugin API, authentication, or service management.

🛡️ License

Licensed under the MIT License — see LICENSE
.

🌟 Credits

Developed by you and contributors.
Inspired by Linux systemd, Windows service control, and Bevy's plugin architecture — but written entirely in modern Rust.

"CircleOSD is the heartbeat of a new OS — modular, fast, secure, and hackable."


--- 
That would make your repo fully buildable and demo-ready.
