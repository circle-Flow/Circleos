use anyhow::Result;
use tokio::process::{Child, Command};
use tracing::{info, warn, error};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::{Instant, Duration};
use crate::service::RestartPolicy;

/// A supervised process wrapper that keeps runtime state.
pub struct SupervisedProcess {
    pub child: Option<Child>,
    pub cmd: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub restart_count: u32,
    pub last_start: Option<Instant>,
    /// backoff seconds for restarts
    pub backoff: Duration,
}

impl SupervisedProcess {
    pub fn new(cmd: Vec<String>, restart_policy: RestartPolicy) -> Self {
        Self {
            child: None,
            cmd,
            restart_policy,
            restart_count: 0,
            last_start: None,
            backoff: Duration::from_secs(1),
        }
    }

    /// spawn the child process asynchronously
    pub async fn spawn(&mut self) -> Result<()> {
        if self.cmd.is_empty() {
            anyhow::bail!("empty command");
        }
        let mut command = Command::new(&self.cmd[0]);
        if self.cmd.len() > 1 {
            command.args(&self.cmd[1..]);
        }
        // Inherit stdio (for now). You can redirect to logs later.
        command.stdout(std::process::Stdio::inherit());
        command.stderr(std::process::Stdio::inherit());

        info!("spawning process: {:?}", self.cmd);
        let child = command.spawn()?;
        self.child = Some(child);
        self.last_start = Some(Instant::now());
        self.restart_count = self.restart_count.saturating_add(1);
        Ok(())
    }

    /// Poll whether the child is still running. Returns Ok(Some(exit_status)) if exited,
    /// Ok(None) if still running.
    pub async fn poll_exit(&mut self) -> Result<Option<std::process::ExitStatus>> {
        if let Some(child) = &mut self.child {
            match child.try_wait()? {
                Some(status) => {
                    self.child = None;
                    Ok(Some(status))
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    /// Kill the child if running.
    pub async fn kill(&mut self) -> Result<()> {
        if let Some(child) = &mut self.child {
            match child.kill().await {
                Ok(_) => {
                    info!("killed child {:?}", self.cmd);
                }
                Err(e) => {
                    warn!("failed to kill child: {:?}", e);
                }
            }
            self.child = None;
        }
        Ok(())
    }

    /// Decide if process should be restarted based on policy and exit status.
    pub fn should_restart(&self, exit: Option<std::process::ExitStatus>) -> bool {
        match self.restart_policy {
            RestartPolicy::Never => false,
            RestartPolicy::Always => true,
            RestartPolicy::OnFailure => {
                if let Some(status) = exit {
                    !status.success()
                } else {
                    false
                }
            }
        }
    }
}

pub type SharedProcess = Arc<Mutex<SupervisedProcess>>;
