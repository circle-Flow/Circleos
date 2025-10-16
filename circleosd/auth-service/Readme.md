How to build & run

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
