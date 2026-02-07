pub mod aln_model;
pub mod file_processor;
pub mod rego_integration;
pub mod game_mechanics;
pub mod sync_engine;
pub mod vm_deploy;
pub mod monitoring;

use chrono::{DateTime, Utc};
use tracing::info;

use crate::aln_model::{
    AlnEvolutionConfig, AlnEvolutionResult, AlnFeatureFlag, AlnFileDescriptor,
    AlnSyntaxVersion,
};
use crate::file_processor::FileProcessor;
use crate::rego_integration::RegoIntegrator;
use crate::game_mechanics::GameMechanicsAdapter;
use crate::sync_engine::SyncEngine;
use crate::vm_deploy::VmDeployer;
use crate::monitoring::MonitoringService;

pub struct AlnSyntaxEvolver {
    config: AlnEvolutionConfig,
    file_processor: FileProcessor,
    rego_integrator: RegoIntegrator,
    game_adapter: GameMechanicsAdapter,
    sync_engine: SyncEngine,
    vm_deployer: VmDeployer,
    monitoring: MonitoringService,
}

impl AlnSyntaxEvolver {
    pub fn new(config: AlnEvolutionConfig) -> Self {
        tracing_subscriber::fmt()
            .with_env_filter("info")
            .init();

        Self {
            file_processor: FileProcessor::new(config.kafka_topic_files.clone()),
            rego_integrator: RegoIntegrator::new(),
            game_adapter: GameMechanicsAdapter::new(),
            sync_engine: SyncEngine::new(
                config.kafka_topic_files.clone(),
                config.kafka_topic_progress.clone(),
            ),
            vm_deployer: VmDeployer::new(config.vm_image.clone()),
            monitoring: MonitoringService::new(config.version.clone()),
            config,
        }
    }

    pub async fn execute_evolution(&self) -> anyhow::Result<AlnEvolutionResult> {
        info!("Starting ALN syntax evolution to {}", self.config.version.current);

        let files: Vec<AlnFileDescriptor> = self
            .file_processor
            .discover_files(self.config.files.clone())
            .await?;

        let processed_count = files.len() as u32;

        for file in &files {
            let meta = self.file_processor.extract_metadata(file).await?;

            match meta.content_type.as_str() {
                "rego_policy" => {
                    self.rego_integrator
                        .integrate_policy(file, &meta)
                        .await?;
                }
                "text_game_tutorial" => {
                    self.game_adapter
                        .adapt_mechanics(file, &meta)
                        .await?;
                }
                _ => {
                    self.file_processor.sync_generic(file, &meta).await?;
                }
            }
        }

        let features = self.build_features();
        let new_version = self
            .config
            .version
            .sequence
            .last()
            .cloned()
            .unwrap_or_else(|| self.config.version.current.clone());

        self.sync_engine
            .sync_progress(
                &new_version,
                features.len() as u32,
                processed_count,
                self.config.progress_kafka_topic.clone(),
            )
            .await?;

        self.vm_deployer
            .deploy_vm(
                &self.config.vm_location,
                &self.config.vm_contract,
                &self.config.vm_lan_mode,
            )
            .await?;

        self.monitoring
            .start_monitoring(
                self.config.monitor_interval_secs,
                self.config.monitor_adoption_threshold,
            )
            .await?;

        let result = AlnEvolutionResult {
            status: "evolution_complete".to_string(),
            token_id: "ALN_EVOLUTION_2025".to_string(),
            version: new_version,
            files_processed: processed_count,
            features_added: features.len() as u32,
            sync_status: "all_nodes_databases_vm_lan".to_string(),
            last_evolution: DateTime::<Utc>::from(self.config.timestamp).to_rfc3339(),
            better_than_python: "chaotic_self_evolving_rego_with_comprehensions_and_sets"
                .to_string(),
        };

        info!(
            "ALN evolution completed: version={}, files={}, features={}",
            result.version, result.files_processed, result.features_added
        );

        Ok(result)
    }

    fn build_features(&self) -> Vec<AlnFeatureFlag> {
        vec![
            AlnFeatureFlag::new(
                "rego_comprehensions_v1.2",
                "Data filtering in policies with high-density comprehensions.",
            ),
            AlnFeatureFlag::new(
                "rego_sets_efficiency_v1.3",
                "Unique membership checks for roles and groups in authorization flows.",
            ),
            AlnFeatureFlag::new(
                "opa_document_model_v1.1",
                "Virtual and base documents for dynamic policy evaluation.",
            ),
            AlnFeatureFlag::new(
                "rego_playground_debug_v1.2",
                "Coverage and selection-style debugging for live policy validation.",
            ),
            AlnFeatureFlag::new(
                "rego_repl_exploratory_v1.3",
                "Exploratory development via REPL with contextual overrides.",
            ),
            AlnFeatureFlag::new(
                "rego_deny_rules_v1.4",
                "Declarative deny messages for label and constraint enforcement.",
            ),
            AlnFeatureFlag::new(
                "rego_conflict_checks_v1.2",
                "Ingress host conflict denial and surface unification.",
            ),
            AlnFeatureFlag::new(
                "rego_gatekeeper_integration_v1.1",
                "Gatekeeper-style constraint and audit integration for ALN VMs.",
            ),
            AlnFeatureFlag::new(
                "text_room_navigation_v1.3",
                "Room exits and navigation mapped into chat navigation primitives.",
            ),
            AlnFeatureFlag::new(
                "text_input_parsing_v1.2",
                "Command parsing for directions and actions in-frame.",
            ),
            AlnFeatureFlag::new(
                "turn_based_mechanics_v1.4",
                "Turn-based health, attack, defend mechanics separated as policies.",
            ),
            AlnFeatureFlag::new(
                "if_cyoa_modularity_v1.2",
                "CYOA-style modular story tooling for self-evolving ALN modules.",
            ),
        ]
    }
}

impl AlnSyntaxVersion {
    pub fn minor_10_levels(&self) -> Self {
        let last = self
            .sequence
            .last()
            .cloned()
            .unwrap_or_else(|| self.current.clone());

        Self {
            current: last.clone(),
            sequence: self.sequence.clone(),
        }
    }
}
