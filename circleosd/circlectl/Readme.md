# circlectl — CircleOSD CLI

`circlectl` is the command-line admin tool for CircleOSD. It communicates with local Unix-socket services (auth, service-registry, plugin-manager).

## Install & Build

```bash
cd circlectl
cargo build --release

Usage
circlectl <COMMAND>
Commands:
  user <create|login> <username>
  service <list|start|stop> [name]
  plugin <list|load|unload> [path|id]
  system <status>


Examples:

Create user:

circlectl user create alice


Login:

circlectl user login alice


List services:

circlectl service list


Start a service:

circlectl service start auth-service


List plugins:

circlectl plugin list


Load a plugin:

circlectl plugin load ./plugins/example.wasm

Config

Default socket paths are used:

plugin manager: /tmp/plugin-manager.sock

service registry: /tmp/service-registry.sock

auth service: /tmp/auth-service.sock

Place a config at ~/.circleosd/config.toml if you want to override (not yet implemented).

Notes

circlectl uses simple JSON messages and expects the services to be running and listening on the default sockets.

The CLI is intentionally minimal — expand commands to suit your operational workflow.


---

## Final notes, tips & next steps

1. **Security**  
   - Native plugin loading uses `libloading` (unsafe). For production, prefer launching native plugins in a separate process to protect the manager process. WASM plugins are sandboxed using Wasmtime and are safer for untrusted code.

2. **WASM ABI**  
   - The provided `WasmInstance::call_func` is a placeholder (returns reversed payload). Replace it with an ABI matching your plugin contract (WASI, canonical ABI, custom mailbox using linear memory allocation, or host functions).

3. **Permissions**  
   - Unix sockets are created with `0600` (owner only). Make sure the admin CLI runs as the same user or adapt permissions.

4. **Integration**  
   - Wire `plugin-manager` into your `service-registry` so it gets supervised automatically.
   - Add plugin manifests next to plugin files (`example.wasm` + `example.json`) to specify plugin type and metadata.

5. **Testing**  
   - Build and run `plugin-manager` and `circlectl` locally, then use `circlectl plugin load` to test WASM or native plugins.

---

- generate a **sample WASM plugin** (Rust -> `wasm32-unknown-unknown` or `wasm32-wasi`) with the ABI you choose, or
- change native plugin handling to **out-of-process** model (spawn plugin process and communicate via stdio or socket), or
- implement a **real Wasm ABI** (host function for logging + string passing) and update `sandbox.r
