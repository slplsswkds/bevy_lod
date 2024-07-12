use bevy::prelude::{Resource};
use crate::lod_distance::LodDistances;

#[derive(Resource)]
pub struct LodSettings {
    pub distances: LodDistances, // Used when LOD have not own LODDistances component
}