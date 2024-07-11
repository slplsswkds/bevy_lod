use bevy::math::NormedVectorSpace;
use bevy::prelude::{Component, Handle, Mesh, Query, Transform, With, Camera3d};
use crate::lod_distance::LODDistances;


// !!!! Remove Clone component !!!!!
#[derive(Component, Clone)]
pub struct LODMesh {
    pub l1: Option<Handle<Mesh>>,
    pub l2: Option<Handle<Mesh>>,
    pub l3: Option<Handle<Mesh>>,
}

pub fn lod_mesh(
    mut query_lod: Query<(&mut Handle<Mesh>, &Transform, &LODMesh, Option<&LODDistances>)>,
    query_cam: Query<&Transform, With<Camera3d>>,
) {
    let cam_transform = query_cam.get_single().unwrap();

    for (mut mesh, mesh_transform, lod, distances_option) in &mut query_lod {
        // let (mut mesh, mesh_transform, lod, distances_option) = query_lod.get_single_mut().unwrap();

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
            // l1_distance = 12.0;
            // l2_distance = 50.0;
            // l3_distance = 100.0;
            l1_distance = 1.0;
            l2_distance = 1.0;
            l3_distance = 1.0;
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
