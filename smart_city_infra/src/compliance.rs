use serde::{Deserialize, Serialize};

/// Compliance profile for smart-city and cybernetic regulations.
/// Intended to be backed by Rego policies at deployment time.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceProfile {
    pub jurisdiction: String,
    pub data_retention_days: u32,
    pub requires_consent_for_biometric: bool,
    pub encrypted_at_rest: bool,
}

impl ComplianceProfile {
    pub fn default_global() -> Self {
        Self {
            jurisdiction: "global_meta_city".to_string(),
            data_retention_days: 365,
            requires_consent_for_biometric: true,
            encrypted_at_rest: true,
        }
    }
}
