use aln_syntax_evolver_v1::aln_model::{AlnEvolutionConfig, AlnSyntaxVersion};
use aln_syntax_evolver_v1::AlnSyntaxEvolver;
use chrono::NaiveDateTime;

pub async fn run_exec_acts_sys_maintenance_v1_5() -> anyhow::Result<()> {
    let version = AlnSyntaxVersion {
        current: "1.0.0.5".to_string(),
        sequence: vec![
            "1.0.0.6".to_string(),
            "1.0.0.7".to_string(),
            "1.0.0.8".to_string(),
            "1.0.0.9".to_string(),
            "1.0.1.0".to_string(),
            "1.0.1.1".to_string(),
            "1.0.1.2".to_string(),
            "1.0.1.3".to_string(),
            "1.0.1.4".to_string(),
            "1.0.1.5".to_string(),
        ],
    };

    let files = vec![
        "web_0_openpolicyagent.org_policy-language.aln".to_string(),
        "web_1_snyk.io_rego-tutorial.aln".to_string(),
        "web_7_github.com_styrainc_awesome-opa.aln".to_string(),
        "web_6_spacelift.io_opa-rego-tutorial.aln".to_string(),
        "web_6_stackoverflow.com_run-script-without-password.aln".to_string(),
        "web_9_thenewstack.io_5-things-opa.aln".to_string(),
        "web_10_aquasec.com_opa-authorization.aln".to_string(),
        "web_11_hashbangcode.com_text-adventure-godot.aln".to_string(),
        "web_8_wikipedia.org_text-based-game.aln".to_string(),
        "web_4_instructables.com_text-turn-based-battle.aln".to_string(),
        "web_13_webbpickersgill.com_text-adventures-tools.aln".to_string(),
        "web_5_filiph.medium.com_skyrim-text-render.aln".to_string(),
    ];

    let config = AlnEvolutionConfig {
        version,
        files,
        kafka_topic_files: "aln_file_evolution".to_string(),
        kafka_topic_progress: "aln_evolution_progress".to_string(),
        progress_kafka_topic: "aln_evolution_progress".to_string(),
        vm_image: "aln_vmamazon.aln".to_string(),
        vm_location: "HOMEDIR$:V://System".to_string(),
        vm_contract: true,
        vm_lan_mode: "full_service_enabled".to_string(),
        monitor_interval_secs: 5,
        monitor_adoption_threshold: 0.95,
        timestamp: NaiveDateTime::parse_from_str("2025-08-10T12:00:00", "%Y-%m-%dT%H:%M:%S")
            .expect("invalid timestamp"),
    };

    let evolver = AlnSyntaxEvolver::new(config);
    let result = evolver.execute_evolution().await?;

    println!(
        "ALN evolution complete: status={}, version={}, files={}, features={}, sync={}",
        result.status,
        result.version,
        result.files_processed,
        result.features_added,
        result.sync_status
    );

    Ok(())
}
