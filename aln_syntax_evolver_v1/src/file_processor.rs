use crate::aln_model::{AlnFileDescriptor, AlnFileMetadata};
use chrono::{NaiveDateTime, Utc};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

pub struct FileProcessor {
    kafka_topic: String,
    producer: FutureProducer,
}

impl FileProcessor {
    pub fn new(kafka_topic: String) -> Self {
        let producer: FutureProducer = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("failed to create kafka producer");

        Self {
            kafka_topic,
            producer,
        }
    }

    pub async fn discover_files(
        &self,
        logical_files: Vec<String>,
    ) -> anyhow::Result<Vec<AlnFileDescriptor>> {
        let now = Utc::now().naive_utc();
        let mut out = Vec::with_capacity(logical_files.len());

        for lf in logical_files {
            let id = Uuid::new_v4();
            let path = format!("data/{}", lf);
            out.push(AlnFileDescriptor {
                id,
                logical_name: lf,
                source_url: None,
                path,
                timestamp: now,
            });
        }

        Ok(out)
    }

    pub async fn extract_metadata(
        &self,
        file: &AlnFileDescriptor,
    ) -> anyhow::Result<AlnFileMetadata> {
        let content_type = if file.logical_name.contains("opa")
            || file.logical_name.contains("rego")
            || file.logical_name.contains("policy")
        {
            "rego_policy"
        } else if file.logical_name.contains("text")
            || file.logical_name.contains("adventure")
            || file.logical_name.contains("game")
        {
            "text_game_tutorial"
        } else {
            "generic"
        }
        .to_string();

        let key_elements = vec![
            "rego_syntax".to_string(),
            "policy_examples".to_string(),
            "text_game_mechanics".to_string(),
            "navigation_steps".to_string(),
            "input_handling".to_string(),
        ];

        Ok(AlnFileMetadata {
            source: file.source_url.clone(),
            content_type,
            key_elements,
        })
    }

    pub async fn sync_generic(
        &self,
        file: &AlnFileDescriptor,
        meta: &AlnFileMetadata,
    ) -> anyhow::Result<()> {
        let payload = json!({
            "file_id": file.id,
            "logical_name": file.logical_name,
            "metadata": meta,
            "timestamp": file.timestamp,
            "encryption": "AES-256-GCM",
        });

        info!(
            "Syncing generic file to nodes via topic {}: {}",
            self.kafka_topic, file.logical_name
        );

        self.producer
            .send(
                FutureRecord::to(&self.kafka_topic)
                    .key(&file.id.to_string())
                    .payload(&payload.to_string()),
                0,
            )
            .await
            .map_err(|(e, _)| anyhow::anyhow!("kafka send failed: {}", e))?;

        Ok(())
    }
}
