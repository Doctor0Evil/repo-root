use crate::aln_model::AlnFeatureFlag;

/// Cybernetic augmentation feature flags for ALN evolution.
/// These flags describe biophysical and cognitive extensions
/// that can be enforced and audited via Rego-style policy layers.
pub fn cybernetic_feature_flags() -> Vec<AlnFeatureFlag> {
    vec![
        AlnFeatureFlag::new(
            "neural_link_v1.0",
            "Neural I/O channels for high-bandwidth human–machine interaction in ALN sessions.",
        ),
        AlnFeatureFlag::new(
            "sensory_mesh_v1.1",
            "Distributed sensory mesh nodes for biophysical telemetry and feedback.",
        ),
        AlnFeatureFlag::new(
            "implant_firmware_guard_v1.2",
            "Policy-guarded firmware update channels for cybernetic implants.",
        ),
        AlnFeatureFlag::new(
            "cognitive_rate_limiter_v1.0",
            "Rate-limiting and safety envelopes for cognitive overclock scenarios.",
        ),
        AlnFeatureFlag::new(
            "augmentation_audit_trail_v1.0",
            "Immutable audit trails for augmentation lifecycle and consent events.",
        ),
    ]
}
