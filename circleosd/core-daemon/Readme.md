core-daemon crate you listed. It’s a small but usable skeleton that:

launches a Unix socket RPC server (/tmp/circleosd.sock by default),

exposes simple JSON RPC actions: create_user, auth, register_service, list_services, load_plugin,

contains the modules you requested: service_registry, auth, plugin, rpc, config,

uses tokio for async, arg0n2 for password hashing, serde/serde_json for RPC payloads,

is intentionally minimal and easy to extend.

Drop this into circleosd/core-daemon/ (as shown), run cargo build and then cargo run to start the daemon. For production you’ll want to add persistence, permissions, secure sockets, better error handling, and more robust process launching/sandboxing.
This is a framework skeleton. To make it production ready you’ll want:

persistent user storage (SQLite / encrypted files),

proper file permissions for the Unix socket and dropping privileges,

TLS or credentialed local socket access,

real process spawn/monitoring in service_registry using tokio::process::Command,

plugin sandboxing (WASM or effective kernel isolation),

unit/integration tests and CI,

config parsing from /etc/circleosd/*.
