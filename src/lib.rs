pub mod lod_mesh;
pub mod lod_distance;

use lod_mesh::*;
use bevy::prelude::*;

pub struct LODPlugin;

impl Plugin for LODPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lod_mesh);
    }
}
