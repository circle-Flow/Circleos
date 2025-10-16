use anyhow::Result;
use std::path::Path;
use wasmtime::{Engine, Module, Store, Linker};
use wasmtime_wasi::WasiCtxBuilder;
use tokio::sync::Mutex;
use std::sync::Arc;
use tracing::info;

/// Lightweight Wasm instance wrapper using Wasmtime.
/// Exposes a simple `call_func` API that expects an exported function with signature `(i32, i32) -> i32`
/// reading memory for string args. For a production plugin ABI, design a proper message interface.
pub struct WasmInstance {
    engine: Engine,
    module: Module,
    // we store one store per instance to allow stateful plugins
    // wrap store/linker in a mutex so we can await across calls
    store_and_link: Arc<Mutex<(Store<()>, Linker<()>)>>,
}

impl WasmInstance {
    pub async fn new(path: &Path) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, path)?;
        let mut store = Store::new(&engine, ());
        // setup WASI (if needed)
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        // instantiate the module eagerly
        let instance = WasmInstance {
            engine,
            module,
            store_and_link: Arc::new(Mutex::new((store, linker))),
        };
        // instantiate once to ensure validity
        {
            let mut guarded = instance.store_and_link.lock().await;
            let (store, linker) = &mut *guarded;
            let inst = linker.instantiate(store, &instance.module)?;
            // optional: call canonical "_initialize"
            let _ = inst.get_func(&mut *store, "_start");
        }
        Ok(instance)
    }

    /// Call a function by name with a JSON payload string; the plugin ABI must provide `handle(payload_ptr, len) -> resp_ptr`.
    /// For simplicity, we call an exported function `handle` which returns an integer code and writes response to memory.
    /// NOTE: This is a toy interface — adapt to your chosen ABI.
    pub async fn call_func(&self, _func: &str, payload: &str) -> Result<String> {
        // This demo implementation returns the payload reversed to show a roundtrip.
        // Replace with actual Wasm call logic for your ABI (WASI, wasm-bindgen or canonical ABI).
        let resp = payload.chars().rev().collect::<String>();
        Ok(resp)
    }

    pub async fn shutdown(&self) -> Result<()> {
        info!("shutting down wasm instance");
        // drop store/linker (no-op)
        Ok(())
    }
}
