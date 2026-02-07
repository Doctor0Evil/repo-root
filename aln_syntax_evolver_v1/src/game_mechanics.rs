use crate::aln_model::{AlnFileDescriptor, AlnFileMetadata};
use serde_json::json;
use tracing::info;

pub struct GameMechanicsAdapter;

impl GameMechanicsAdapter {
    pub fn new() -> Self {
        Self
    }

    pub async fn adapt_mechanics(
        &self,
        file: &AlnFileDescriptor,
        meta: &AlnFileMetadata,
    ) -> anyhow::Result<()> {
        let record = json!({
            "file_id": file.id,
            "logical_name": file.logical_name,
            "mode": "chat_interop_render",
            "separation": "game_engine_from_ai_chat",
            "metadata": meta,
            "log_tags": ["aln_game_adaptation"],
        });

        info!(
            "Adapting text game mechanics for ALN chat rendering: {}",
            record.to_string()
        );

        Ok(())
    }
}
