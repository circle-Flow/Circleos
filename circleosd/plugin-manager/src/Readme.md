# Plugin Manager (CircleOSD)

The `plugin-manager` loads and manages plugins for CircleOSD. It supports:

- Native plugins (`.so`, `.dll`) loaded via `libloading` (in-process).
- WebAssembly plugins (`.wasm`) executed inside a Wasmtime sandbox.

> ⚠️ Security note: native plugins run in-process and can execute arbitrary code. Use native plugins only when trusted. Prefer WASM plugins for untrusted third-party code.

---

## File layout

plugin-manager/

├── Cargo.toml

└── src/

├── main.rs

├── loader.rs

├── sandbox.rs

├── manifest.rs

└── api.rs

## Build

```bash
cd plugin-manager
cargo build --release
Run:

bash

cargo run --release
By default the manager listens on /tmp/plugin-manager.sock for a simple JSON-RPC API (newline-delimited JSON).

API
Send newline-delimited JSON to /tmp/plugin-manager.sock.

List loaded plugins
json

{"action":"list"}
Load plugin
json

{"action":"load","path":"./plugins/example.wasm","manifest":null}
If manifest is omitted, the manager will try to find example.json next to the plugin file.

Unload plugin
json

{"action":"unload","id":"<plugin-id>"}
Invoke WASM plugin function (toy example)
json

{"action":"invoke","id":"<plugin-id>","func":"handle","payload":"hello"}
Response structure:

json

{"ok":true,"message":null,"data": ...}
Plugin manifest
A plugin manifest is a JSON file alongside the plugin (e.g. example.wasm + example.json):

json

{
  "name": "example",
  "version": "0.1",
  "description": "Demo plugin",
  "plugin_type": "wasm",
  "entry": null
}
plugin_type is "native" or "wasm".

entry is optional symbol name for native plugins.

Example: Native plugin (Rust)
Create plugins/example_native/src/lib.rs:

rust

#[no_mangle]
pub extern "C" fn plugin_init() {
    println!("example native plugin initialized");
}
Build as cdylib and load via plugin-manager.

Example: WASM plugin
WASM plugin should export functions according to the ABI you choose. The sample sandbox currently implements a toy call interface. Extend sandbox.rs to match your plugin ABI.

Next steps / Hardening
Replace native in-process loading with out-of-process worker for isolation.

Implement a proper Wasm canonical ABI or WASI-based messaging for richer interactions.

Maintain plugin metadata and state in persistent storage.

Provide versioning, capability declarations, and signing for plugin authenticity.



---

