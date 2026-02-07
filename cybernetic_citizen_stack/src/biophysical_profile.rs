use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Biophysical augmentation and telemetry envelope.
/// Designed to live in a decentralized data-bank for
/// cybernetic evolution and safety monitoring.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BiophysicalProfile {
    pub subject_id: String,
    pub implants: Vec<ImplantChannel>,
    pub last_update: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImplantChannel {
    pub name: String,
    pub modality: String,
    pub firmware_version: String,
    pub safety_status: SafetyStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SafetyStatus {
    Nominal,
    Degraded,
    Critical,
}

impl BiophysicalProfile {
    pub fn new<S: Into<String>>(subject_id: S) -> Self {
        Self {
            subject_id: subject_id.into(),
            implants: Vec::new(),
            last_update: Utc::now(),
        }
    }

    pub fn add_implant(&mut self, channel: ImplantChannel) {
        self.implants.push(channel);
        self.last_update = Utc::now();
    }
}
