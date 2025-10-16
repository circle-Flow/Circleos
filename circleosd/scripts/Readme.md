# 🧰 CircleOSD System Scripts

Helper scripts for building, running, and installing the entire CircleOSD workspace.

---

## ⚙️ `build.sh`
Compiles all workspace crates in release mode.

```bash
./scripts/build.sh
Artifacts are placed in build/release/.

🚀 run.sh

Starts the core-daemon, initializes directories under var/, and logs output to var/log/circleosd.log.

./scripts/run.sh


To stop it, use:

circlectl system shutdown

🧩 install.sh

Copies all built binaries into /usr/local/bin (requires sudo).

sudo ./scripts/install.sh


After installation:

circlectl --help

🧾 Notes

These scripts assume Linux/macOS/WSL environment.

You can edit the paths or service behavior as needed.

The run.sh script simulates the OS boot process for development.


---

## 🌍 Optional: `README.md` (for `/var` and `/build`)

### **`var/README.md`**
```markdown
# 🗃️ CircleOSD Runtime Data

This directory holds temporary and runtime-generated data.

| Folder | Description |
|---------|-------------|
| `var/log/` | System logs for daemons and services |
| `var/run/` | UNIX sockets and PID files |
| `var/sessions/` | Temporary session data for user logins |

---

These folders are automatically created by `scripts/run.sh` or `core-daemon` at startup.

build/README.md
# 🏗️ CircleOSD Build Artifacts

All compiled binaries from the CircleOSD workspace are collected here.

| Folder | Purpose |
|---------|----------|
| `build/release/` | Optimized release builds for deployment |
| `build/debug/` | Development/debug builds |

Use the provided helper scripts:
```bash
./scripts/build.sh    # Build
./scripts/run.sh      # Run
./scripts/install.sh  # Install globally


---

## ✅ Summary

| Folder | Purpose |
|---------|----------|
| `var/` | Runtime data (logs, sockets, sessions) |
| `build/` | Compiled binaries for deployment |
| `scripts/` | Helper tools for build/run/install |
| `circleosd.log` | Main system log |
| `circleosd.sock` | IPC socket |
| `build.sh` | Builds all services |
| `run.sh` | Boots CircleOSD |
| `install.sh` | Installs to system |

---


[BOOT] Powering on...
[BOOT] Loading kernel services...
[OK] Auth Service ready.
[OK] Plugin Manager ready.
[OK] System login available.

That would make your simulated OS feel like a real boot sequence.
