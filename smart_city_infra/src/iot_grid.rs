use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Smart-city IoT grid node for cybernetic and infrastructure signals.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IotNode {
    pub id: Uuid,
    pub kind: String,
    pub location_label: String,
    pub last_seen: DateTime<Utc>,
}

impl IotNode {
    pub fn new<S: Into<String>>(kind: S, location_label: S) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: kind.into(),
            location_label: location_label.into(),
            last_seen: Utc::now(),
        }
    }

    pub fn touch(&mut self) {
        self.last_seen = Utc::now();
    }
}
