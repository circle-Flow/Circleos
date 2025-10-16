# 🌀 CircleOSD

> A modular, Rust-based system daemon providing service registry, authentication, and plugin management — the foundation for a lightweight operating system.

---

## ⚙️ Overview

CircleOSD is the **core system daemon** that initializes at startup, manages services, verifies authentication, and loads dynamic plugins.  
It’s written in Rust for performance and safety, designed to be extensible for building advanced OS-level functionality or lightweight service platforms.

### 🧩 Features
- Service registry for managing background processes
- Secure Argon2-based authentication
- Plugin system for dynamic module loading
- JSON RPC communication over Unix socket
- Config-driven initialization

---

## 📂 Project Structure

circleosd/

│
├── Cargo.toml # Workspace manifest

├── README.md # Documentation

├── LICENSE # MIT license

└── core-daemon/ # Main daemon implementation


---

## 🚀 Quick Start

### 1️⃣ Install Rust
If you haven’t already:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2️⃣ Clone and build
git clone https://github.com/yourname/circleosd.git
cd circleosd
cargo build --release

3️⃣ Run
cargo run -p core-daemon


You’ll see logs:

[INFO] CircleOSD starting...
[OK] RPC listening on /tmp/circleosd.sock

💬 RPC Interface Example

All control happens through JSON commands sent over a local Unix socket.

Example (with socat):

echo '{"action":"create_user","username":"test","password":"1234"}' | \
socat - UNIX-CONNECT:/tmp/circleosd.sock


Output:

{"ok":true,"message":"user created"}

🧠 Design Goals
Goal	Description
🔒 Security	Safe Rust + Argon2 authentication
⚙️ Modularity	Service registry, auth, and plugins as separate modules
⚡ Performance	Async runtime (Tokio), lightweight IPC
🧩 Extensibility	Easily add new plugins and services
🧰 Transparency	JSON-based, human-readable configuration
📅 Roadmap
Phase	Feature
✅ Core Daemon	Base framework, auth, RPC
🔜 CLI Tool	circlectl for command-line control
🔜 Plugin Sandbox	Load WASM plugins safely
🔜 Greeter UI	Login interface (TUI/GUI)
🔜 Game / App Engine	Integration for user applications
🧰 Build & Test

Build:

cargo build --workspace


Test:

cargo test


Run in verbose mode:

RUST_LOG=debug cargo run -p core-daemon

🧩 Contributing

Pull requests welcome!
Please:

Fork the repo

Create a feature branch

Submit a PR

We follow Rust’s style guidelines (cargo fmt, cargo clippy).

🛡️ License

MIT License — see LICENSE
 for details.

🌟 Credits

Developed by You and the open-source community.
Inspired by Linux systemd, Bevy’s plugin architecture, and Windows service control — reimagined for Rust.


---

## 🪪 `LICENSE` (MIT License)

```text
MIT License

Copyright (c) 2025 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the “Software”), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.


✅ After this:

Create your core-daemon/ folder (with its Cargo.toml and src files from the previous step).

Run:

cargo build --workspace


You’ll get:

target/release/core-daemon



Explanation of Key Components
🔧 Core Components
Folder	Role

core-daemon/	The “heart” of the OS — starts early, manages services, handles auth, loads plugins, provides IPC.

service-registry/	Keeps track of running system services, dependencies, and restarts crashed ones.

auth-service/	Manages users, login, password hashing (Argon2), sessions/tokens.

plugin-manager/	Loads dynamic or WASM plugins; sandboxing; plugin manifest validation.

circlectl/	Command-line tool for users/admins — control services, manage plugins/users.

🎨 Interface & User Experience
Folder	Role
greeter/	Optional GUI/TUI login/greeter (like GNOME Display Manager or LightDM).
plugins/	Extensible plugins for system utilities or extra features.
apps/	Where your user applications and games live (built with Rust engines like Bevy).
⚙️ Config & Runtime
Folder	Description
etc/	System configuration — loaded by core-daemon during boot.
var/	Runtime data, sockets, session files, logs.
scripts/	Build and install helper scripts.
