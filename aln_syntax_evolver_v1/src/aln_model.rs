use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlnSyntaxVersion {
    pub current: String,
    pub sequence: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlnEvolutionConfig {
    pub version: AlnSyntaxVersion,
    pub files: Vec<String>,
    pub kafka_topic_files: String,
    pub kafka_topic_progress: String,
    pub progress_kafka_topic: String,
    pub vm_image: String,
    pub vm_location: String,
    pub vm_contract: bool,
    pub vm_lan_mode: String,
    pub monitor_interval_secs: u64,
    pub monitor_adoption_threshold: f64,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlnFileDescriptor {
    pub id: Uuid,
    pub logical_name: String,
    pub source_url: Option<String>,
    pub path: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlnFileMetadata {
    pub source: Option<String>,
    pub content_type: String,
    pub key_elements: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlnFeatureFlag {
    pub key: String,
    pub description: String,
}

impl AlnFeatureFlag {
    pub fn new<K: Into<String>, D: Into<String>>(key: K, description: D) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlnEvolutionResult {
    pub status: String,
    pub token_id: String,
    pub version: String,
    pub files_processed: u32,
    pub features_added: u32,
    pub sync_status: String,
    pub last_evolution: String,
    pub better_than_python: String,
}
