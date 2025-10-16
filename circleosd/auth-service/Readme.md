# 🔐 CircleOSD Auth Service

The **Auth Service** is a standalone authentication microservice for **CircleOSD**.  
It provides user registration, login, and session validation APIs.  
It uses **SQLite** for persistent user storage and **Argon2** for password hashing, with sessions managed in-memory or persisted as needed.

---

## ✨ Features

- 🧍 User registration and login
- 🔑 Secure password hashing via **Argon2id**
- 🪣 Persistent user storage using **SQLite**
- 🧠 In-memory session store with TTL-based expiration
- 🧩 JSON-RPC–style API over Unix socket or TCP (configurable)
- 🧰 Modular and embeddable into other CircleOSD components

---

## 🧱 Architecture Overview

auth-service/

├── src/

│ ├── main.rs # Entry point — starts the RPC server

│ ├── db.rs # SQLite database access (rusqlite / sqlx)

│ ├── hash.rs # Argon2 password hashing helpers

│ ├── api.rs # JSON-RPC API endpoints (login, register, validate)

│ └── session.rs # Session token management (in-memory)

└── Cargo.toml

The service can run independently or under CircleOSD’s `service-registry` process supervisor.

---

## ⚙️ Configuration

Configuration is currently minimal and defined via environment variables:

| Variable | Default | Description |
|-----------|----------|-------------|
| `AUTH_DB_PATH` | `auth.db` | Path to SQLite database file |
| `AUTH_SOCKET` | `/tmp/auth-service.sock` | Unix socket for RPC server |
| `SESSION_TTL_SECS` | `3600` | Lifetime of user sessions (seconds) |

---

## 🚀 Running the Service

### Prerequisites

- [Rust](https://www.rust-lang.org/) (edition 2021)
- SQLite 3 (for local persistence)

### Build & Run

```bash
cd auth-service
cargo build --release
cargo run --release
The service will start and listen on /tmp/auth-service.sock.

🧪 API Usage
All requests are newline-delimited JSON-RPC objects.
You can test easily using socat or ncat.

Register User
bash
echo '{"action":"register","username":"alice","password":"secret"}' \
  | socat - UNIX-CONNECT:/tmp/auth-service.sock
Login
bash
echo '{"action":"login","username":"alice","password":"secret"}' \
  | socat - UNIX-CONNECT:/tmp/auth-service.sock
Validate Session
bash
echo '{"action":"validate","token":"<session_token>"}' \
  | socat - UNIX-CONNECT:/tmp/auth-service.sock
Example Response
json
{"ok":true,"message":"login successful","data":{"token":"f3b2c8e7..."}}
🧰 Integration with CircleOSD
greeter/ uses this service for login/authentication.

core-daemon may query it to validate session tokens.

service-registry can supervise it like any other microservice.

Example entry in etc/services.toml:


[[service]]
name = "auth-service"
cmd = ["./auth-service"]
restart = "Always"
🧑‍💻 Development Notes
Passwords are hashed with Argon2id and random salt.

Sessions are stored in-memory; replaceable with Redis or SQLite later.

Error responses are structured as:

json

{"ok": false, "message": "Invalid credentials"}
🧾 License
This service is part of the CircleOSD Project.
Licensed under the MIT License — see the root LICENSE file for details.

📚 Future Improvements
 Persistent session store (SQLite / Redis)

 JWT-based tokens

 Account recovery and password reset APIs

 Public key auth support for CircleCLI


**How to build & run**

Create directory structure and files:

circleosd/
└── auth-service/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        ├── db.rs
        ├── hash.rs
        ├── api.rs
        └── session.rs


Build:

cd auth-service
cargo build --release


Run:

cargo run --release


The service will:

create ./var/auth.db (ensure ./var/ exists or it will be created),

listen on Unix socket /tmp/auth-service.sock for JSON-RPC lines.

Example usage (via socat)

Create a user:

echo '{"action":"create_user","username":"alice","password":"secret"}' | socat - UNIX-CONNECT:/tmp/auth-service.sock
# -> {"ok":true,"message":"user created"}


Login:

echo '{"action":"login","username":"alice","password":"secret"}' | socat - UNIX-CONNECT:/tmp/auth-service.sock
# -> {"ok":true,"message":"authenticated","data":{"token":"...","username":"alice"}}


Check token:

echo '{"action":"whoami","token":"<token>"}' | socat - UNIX-CONNECT:/tmp/auth-service.sock
# -> {"ok":true,"data":{"username":"alice"}}

Notes / Next improvements

Sessions are in-memory and cleared on restart — consider persisting refresh tokens.

The socket permission is set to owner-only (600) on Unix; run clients as the same user or adapt permissions.

For production, add TLS-authenticated IPC or use system-level socket activation (systemd).

Add rate-limiting, account lockout, and stronger validation for production security.

You can easily adapt api.rs to expose HTTP endpoints (warp/axum) instead of Unix socket.
