pub mod lod_mesh;
pub mod lod_distance;
pub mod lod_settings;

use bevy::prelude::*;
use crate::{
    lod_distance::LODDistances,
    lod_settings::LODSettings,
    lod_mesh::*,
};

pub struct LODPlugin;

impl Plugin for LODPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LODSettings {
                distances: LODDistances::new(10.0, 20.0, 100.0)
            })
            .add_systems(Update, lod_mesh_single);
    }
}
