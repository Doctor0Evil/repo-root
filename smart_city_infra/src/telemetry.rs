use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Telemetry envelope for smart-city and augmented-citizen signals
/// feeding into the decentralized ALN data-bank.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TelemetrySample {
    pub source: String,
    pub channel: String,
    pub value: String,
    pub recorded_at: DateTime<Utc>,
}

impl TelemetrySample {
    pub fn new<S: Into<String>>(source: S, channel: S, value: S) -> Self {
        Self {
            source: source.into(),
            channel: channel.into(),
            value: value.into(),
            recorded_at: Utc::now(),
        }
    }
}
