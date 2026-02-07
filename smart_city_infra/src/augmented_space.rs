use serde::{Deserialize, Serialize};

/// Augmented urban space descriptor for AR / cybernetic overlays.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AugmentedZone {
    pub zone_id: String,
    pub description: String,
    pub safety_rating: u8,
    pub access_level: u8,
}

impl AugmentedZone {
    pub fn public_plaza<S: Into<String>>(zone_id: S) -> Self {
        Self {
            zone_id: zone_id.into(),
            description: "Public augmented plaza with mixed reality overlays.".to_string(),
            safety_rating: 5,
            access_level: 1,
        }
    }
}
