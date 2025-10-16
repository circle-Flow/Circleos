🌀 CircleOSD

An Experimental Modular Operating System Daemon Built in Rust

📖 Overview

CircleOSD (Circle Operating System Daemon) is a modular, service-oriented micro-OS kernel written in Rust.
It doesn’t replace Linux — it runs on top of it — but behaves like an OS core: booting, authenticating users, running services, and managing plugins and apps.

CircleOSD aims to demonstrate:

⚙️ A microkernel-like architecture in user space

🧩 A plugin-based system for extending functionality dynamically

🔐 A secure authentication service (Argon2 + sessions)

🧠 A service registry for supervision and discovery

🖥️ A CLI control tool (circlectl) for managing the entire system

🎮 An apps layer for games and experiments

It’s both a learning OS project and a real, extensible service platform.

🧱 Architecture
circleosd/

│
├── core-daemon/        # Central orchestrator (PID 1 of CircleOS)

├── auth-service/       # Handles users, login, sessions

├── service-registry/   # Tracks and supervises all running services

├── plugin-manager/     # Loads and isolates .so/.dll/.wasm plugins

├── circlectl/          # CLI interface to interact with CircleOSD

├── greeter/            # Optional TUI/CLI login screen

├── plugins/            # Example and external plugins

├── apps/               # Optional apps and game demos

├── etc/                # System configuration files

├── var/                # Runtime data (logs, sockets, sessions)

├── build/              # Compiled binaries

└── scripts/            # Helper scripts (build/run/install)


🧩 Each module is its own Rust crate, part of a unified Cargo workspace.

⚡ Features
Feature	Description
Boot Sequence	Mimics real OS startup: services, auth, plugins
Service Registry	Supervises registered services, monitors health
Authentication	Argon2 password hashing, session tokens
Plugin System	Load .so, .dll, or .wasm dynamically
RPC Communication	JSON-RPC over Unix sockets
CLI Control (circlectl)	Manage users, services, and plugins
Logging System	Persistent logs in var/log/
Config-Driven	All services read etc/*.toml configs
Extensible	Add your own apps and games easily
🪄 Boot Flow

When you power on CircleOSD:

The core daemon starts (core-daemon)

It runs a boot sequence:

Loads configuration (etc/circleosd.conf)

Starts the Service Registry

Starts the Auth Service

Starts the Plugin Manager

Opens the RPC socket (var/run/circleosd.sock)

Logs boot progress to var/log/circleosd.log

Waits for user login via:

circlectl user login

or greeter (text UI)

🧩 Components Overview:

Component	Description:

Core Daemon	The main orchestrator — listens for commands and routes RPC calls
Auth Service	Provides login/register APIs using SQLite + Argon2
Service Registry	Keeps track of all microservices and restarts them if needed
Plugin Manager	Loads .so, .dll, or .wasm plugins dynamically with sandboxing
circlectl	A powerful CLI tool to manage CircleOSD
Greeter	Optional TUI for user login at boot
Apps	Example applications (games, engines, demos)
Plugins	Extensible feature modules, loaded on demand

🧰 Installation & Build

1️⃣ Prerequisites

Rust (>=1.75)

cargo build tool

Linux or macOS (Unix socket support)

Optional: sqlite3, wasmtime (for WASM plugins)

2️⃣ Clone the Repo:

git clone https://github.com/yourusername/circleosd.git
cd circleosd

3️⃣ Build Everything:

cargo build --workspace


or build specific components:

cargo build -p core-daemon
cargo build -p circlectl

4️⃣ Run the System

Start the daemon:

cargo run -p core-daemon


Then in another terminal, authenticate:

cargo run -p circlectl -- user register admin
cargo run -p circlectl -- user login admin


List services:

cargo run -p circlectl -- service list


Load a plugin:

cargo run -p circlectl -- plugin load plugins/example/target/debug/libexample.so

🧩 Configuration

Configuration files live in /etc:

File	Purpose
circleosd.conf	Main daemon settings (socket path, logging)
services.toml	List of system services
auth.conf	Database and auth config
plugins.toml	Preloaded plugin definitions
🪵 Logs & Runtime Data

All runtime data lives in /var:

Path	Description
var/log/	Persistent system logs
var/run/	Unix sockets & PID files
var/sessions/	Active user sessions
🕹️ Example App: Game Engine Demo

You can create games or sandbox apps inside the apps/ directory.
Example:

cargo run -p game-engine-demo


This uses CircleOSD APIs to log, register with the service registry, and run in a safe sandbox.

🔌 Plugin Example

Each plugin has a manifest:

# plugins/example/Cargo.toml
[package]
name = "example"
version = "0.1.0"
crate-type = ["cdylib"]

[lib]
path = "src/lib.rs"


And an entrypoint:

#[no_mangle]
pub extern "C" fn plugin_entry() {
    println!("✅ Example plugin initialized!");
}


You can load it dynamically via the CLI or automatically at startup (if listed in plugins.toml).

💡 Development Workflow

Run core-daemon

Open a second terminal for circlectl

Add users, start services, and load plugins live

Watch logs update in real-time:

tail -f var/log/circleosd.log

🧩 Example RPC Flow
circlectl user login
  ↓
core-daemon → auth-service (verifies credentials)
  ↓
auth-service → core-daemon (returns session token)
  ↓
circlectl prints "Welcome, <username>"

🧱 Project Goals

Teach how OS concepts (registry, auth, services) can be modeled in userspace

Provide a modular foundation for building experimental environments

Demonstrate secure, async, service-oriented design in Rust

🧑‍💻 Roadmap

 GUI front-end (WebSocket or GTK)

 Plugin sandboxing via Wasmtime isolation

 Persistent user database

 Game engine SDK

 Distributed CircleOSD nodes (networked mode)

⚖️ License

Licensed under the MIT License.
See LICENSE
 for details.

❤️ Credits

Built with 🦀 Rust, ✨ Tokio, 🧩 Serde, and ❤️ by open-source contributors.
CircleOSD is a sandbox for innovation — not a replacement OS, but a vision for modular computing.
[BOOT] Powering on CPU cores...
[OK] Powering on CPU cores...
[BOOT] Loading kernel services...
[OK] Loading kernel services...
[SYSTEM] RPC socket ready on var/run/circleosd.sock
[SYSTEM] Boot sequence completed.
========================================

📸 Screenshot :
====================================================
🌀  CircleOSD Core Daemon Boot Sequence v0.2.0
====================================================

🟢 System ready for user login
Use `circlectl user login` to authenticate.
====================================================
