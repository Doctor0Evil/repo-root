use serde::{Deserialize, Serialize};

/// Policy binding that connects citizen and biophysical state
/// to Rego/OPA-style authorization and safety checks in ALN.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyBinding {
    pub policy_package: String,
    pub rule: String,
    pub description: String,
}

impl PolicyBinding {
    pub fn safety_envelope() -> Self {
        Self {
            policy_package: "aln_augmentation_safety".to_string(),
            rule: "allow_augmentation_if_safe".to_string(),
            description: "Ensure all biophysical augmentations remain within safety envelopes.",
        }
    }

    pub fn smart_city_access() -> Self {
        Self {
            policy_package: "aln_smart_city_access".to_string(),
            rule: "allow_city_grid_access".to_string(),
            description: "Guard access to smart-city infrastructure channels for citizens.",
        }
    }
}
