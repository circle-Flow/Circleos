use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::sync::Arc;

/// Basic in-memory session store. Not persistent — restart clears sessions.
/// Token TTL is configurable via TTL_SECS.
const TTL_SECS: i64 = 60 * 60; // 1 hour

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub username: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone, Default)]
pub struct SessionStore {
    inner: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// Create a new session for username and return token.
    pub async fn create_session(&self, username: &str) -> String {
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::seconds(TTL_SECS);
        let session = Session {
            token: token.clone(),
            username: username.to_string(),
            expires_at,
        };
        self.inner.lock().await.insert(token.clone(), session);
        token
    }

    /// Validate token and return associated username if valid.
    pub async fn validate(&self, token: &str) -> Option<String> {
        let mut map = self.inner.lock().await;
        if let Some(sess) = map.get(token) {
            if Utc::now() < sess.expires_at {
                return Some(sess.username.clone());
            } else {
                // expired, remove
                map.remove(token);
            }
        }
        None
    }

    /// Periodic cleanup of expired sessions (callable).
    pub async fn cleanup(&self) {
        let mut map = self.inner.lock().await;
        let now = Utc::now();
        let keys: Vec<String> = map.iter()
            .filter(|(_, s)| s.expires_at <= now)
            .map(|(k, _)| k.clone())
            .collect();
        for k in keys {
            map.remove(&k);
        }
    }
}
