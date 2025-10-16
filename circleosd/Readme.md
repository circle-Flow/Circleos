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
