use bevy::prelude::{Resource};
use crate::lod_distance::LodDistances;

///Stores the global values of LodSettings
#[derive(Resource)]
pub struct LodSettings {
    /// Used when LOD have not own LodDistances component
    pub distances: LodDistances,
}