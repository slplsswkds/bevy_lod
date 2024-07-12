use bevy::prelude::{Component, Handle, Mesh, Query, Transform, With, Camera3d, Res};
use crate::lod_distance::LODDistances;
use crate::lod_settings::LODSettings;


// !!!! Remove Clone impl !!!!!
/// LODs meshes for single entities.
#[derive(Component, Clone)]
pub struct LODMesh {
    pub l1: Option<Handle<Mesh>>,
    pub l2: Option<Handle<Mesh>>,
    pub l3: Option<Handle<Mesh>>,
}

pub fn lod_mesh_single(
    mut query_lod: Query<(&mut Handle<Mesh>, &Transform, &LODMesh, Option<&LODDistances>)>,
    lod_settings: Res<LODSettings>,
    query_cam: Query<&Transform, With<Camera3d>>,
) {
    let cam_transform = query_cam.get_single().unwrap();

    let l1_distances_global = lod_settings.distances.l1;
    let l2_distances_global = lod_settings.distances.l2;
    let l3_distances_global = lod_settings.distances.l3;

    for (mut mesh, mesh_transform, lod, distances_option) in &mut query_lod {
        let l1_distance;
        let l2_distance;
        let l3_distance;

        if let Some(distances) = distances_option {
            // use unique values for LOD distances
            l1_distance = distances.l1.clone();
            l2_distance = distances.l2.clone();
            l3_distance = distances.l3.clone();
        } else {
            // use global values for LOD distances from Resource
            l1_distance = l1_distances_global;
            l2_distance = l2_distances_global;
            l3_distance = l3_distances_global;
        }

        let distance = cam_transform.translation.distance(mesh_transform.translation);

        if distance <= l1_distance {
            *mesh = lod.l1.clone().unwrap();
        } else if distance <= l2_distance {
            *mesh = lod.l2.clone().unwrap();
        } else if distance <= l3_distance {
            *mesh = lod.l3.clone().unwrap();
        }
    }
}
// https://github.com/bevyengine/bevy/blob/main/examples/3d/shadow_caster_receiver.rs
