use tracing::info;
use crate::service::ServiceSpec;

/// Health check utilities. Currently minimal: we can implement HTTP probe or command probe.
/// For now we only provide a placeholder function that returns true if process exists.
pub async fn check_process_alive(_spec: &ServiceSpec) -> bool {
    // Placeholder: in future, use HTTP probe or custom command.
    // Returning true indicates the process is "healthy" from the registry perspective.
    // The supervisor will treat process liveness separately.
    true
}
