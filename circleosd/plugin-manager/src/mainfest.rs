use serde::{Deserialize, Serialize};

/// Plugin manifest describes plugin metadata and type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginType {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "wasm")]
    Wasm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub plugin_type: PluginType,
    #[serde(default)]
    pub entry: Option<String>, // symbol name for native or main function for wasm
}
