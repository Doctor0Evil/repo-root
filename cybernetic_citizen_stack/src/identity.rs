use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core augmented-citizen identity record.
/// This is the anchor for cybernetic and smart-city bindings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CitizenIdentity {
    pub id: Uuid,
    pub handle: String,
    pub created_at: DateTime<Utc>,
    pub smart_city_domain: String,
    pub verified: bool,
}

impl CitizenIdentity {
    pub fn new<S: Into<String>>(handle: S, smart_city_domain: S) -> Self {
        Self {
            id: Uuid::new_v4(),
            handle: handle.into(),
            created_at: Utc::now(),
            smart_city_domain: smart_city_domain.into(),
            verified: false,
        }
    }

    pub fn mark_verified(&mut self) {
        self.verified = true;
    }
}
