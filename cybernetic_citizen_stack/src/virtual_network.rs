use serde::{Deserialize, Serialize};

/// Virtual networking surface that connects augmented citizens
/// into next-generation ALN meshes and smart-city overlays.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VirtualNetworkEndpoint {
    pub name: String,
    pub protocol: String,
    pub scope: NetworkScope,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkScope {
    PersonalMesh,
    DistrictMesh,
    CityMesh,
    GlobalMesh,
}

impl VirtualNetworkEndpoint {
    pub fn smart_city_mesh<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            protocol: "aln-smart-lan-v1".to_string(),
            scope: NetworkScope::CityMesh,
        }
    }
}
