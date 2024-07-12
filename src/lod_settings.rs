use bevy::prelude::{Resource};
use crate::lod_distance::LODDistances;

#[derive(Resource)]
pub struct LODSettings {
    pub distances: LODDistances, // Used when LOD have not own LODDistances component
}