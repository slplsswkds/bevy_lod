use bevy::app::{App, Plugin};
use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::{Component, Query, Handle, Image, Transform, With, Camera3d, Res, ResMut, Assets, StandardMaterial, Mesh};
use crate::lod_distance::LodDistances;
use crate::lod_mesh::LodMesh;
use crate::lod_settings::LodSettings;


#[derive(Component, Default, Clone)]
pub struct LodPbr {
    pub l1: Option<Handle<StandardMaterial>>,
    pub l2: Option<Handle<StandardMaterial>>,
    pub l3: Option<Handle<StandardMaterial>>,
}

pub fn lod_pbr(
    mut query_lod: Query<(&mut Handle<StandardMaterial>, &Transform, &LodPbr, Option<&LodDistances>)>,
    lod_settings: Res<LodSettings>,
    query_cam: Query<&Transform, With<Camera3d>>,
) {
    let cam_transform = query_cam.get_single().unwrap();
    let (l1_distances_global, l2_distances_global, l3_distances_global) = lod_settings.distances.get_tupple();

    for (mut material, transform, lod_pbr, distances_option) in &mut query_lod {
        let (l1_distance, l2_distance, l3_distance);

        if let Some(distances) = distances_option {
            // use unique values for each LOD distances
            (l1_distance, l2_distance, l3_distance) = distances.get_tupple();
        } else {
            // use global values for LOD distances from Resource
            (l1_distance, l2_distance, l3_distance) = (l1_distances_global, l2_distances_global, l3_distances_global);
        }

        let distance = cam_transform.translation.distance(transform.translation);

        if distance <= l1_distance {
            *material = lod_pbr.l1.clone().unwrap();
        } else if distance <= l2_distance {
            *material = lod_pbr.l2.clone().unwrap();
        } else if distance <= l3_distance {
            *material = lod_pbr.l3.clone().unwrap();
        }
    }
}
