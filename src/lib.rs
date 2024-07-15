pub mod lod_mesh;
pub mod lod_distance;
pub mod lod_settings;
pub mod lod_pbr;

use bevy::prelude::*;
use crate::{
    lod_distance::LodDistances,
    lod_settings::LodSettings,
    lod_mesh::*,
    lod_pbr::lod_pbr,
};

pub struct LODPlugin;

impl Plugin for LODPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LodSettings {
                distances: LodDistances::new(10.0, 20.0, 100.0)
            })
            .add_systems(Update, (lod_mesh_single, lod_pbr).chain());
    }
}
