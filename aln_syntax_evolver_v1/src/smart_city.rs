use crate::aln_model::AlnFeatureFlag;

/// Smart-city integration feature flags for ALN evolution.
/// These align the system with augmented smart-city networking
/// and cybernetic urbanism perspectives.
pub fn smart_city_feature_flags() -> Vec<AlnFeatureFlag> {
    vec![
        AlnFeatureFlag::new(
            "edge_iot_nodes_v1.0",
            "Edge IoT nodes for street-level sensing and actuation in ALN virtual networks.",
        ),
        AlnFeatureFlag::new(
            "mobility_digital_twin_v1.1",
            "Digital twin models for mobility corridors and micro‑transit flows.",
        ),
        AlnFeatureFlag::new(
            "urban_safety_overlay_v1.0",
            "Policy-governed overlays for public safety, alerts, and guidance.",
        ),
        AlnFeatureFlag::new(
            "resource_efficiency_policy_v1.0",
            "Sustainability and resource-efficiency constraints for smart grids and utilities.",
        ),
        AlnFeatureFlag::new(
            "citizen_experience_channel_v1.0",
            "Augmented-citizen interaction channels that merge AR, chat, and city services.",
        ),
    ]
}
