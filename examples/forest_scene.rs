mod cam;

use std::num::NonZero;
use bevy::prelude::*;
use bevy_lod::*;
use cam::*;
use rand::Rng;

use bevy::{
    pbr::{DirectionalLightShadowMap, PointLightShadowMap},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Forest Scene".into(),
                    resolution: (1000., 600.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // composite_alpha_mode: CompositeAlphaMode::Inherit, // or opaque
                    visible: true,
                    resizable: false,
                    transparent: false,
                    ime_enabled: false,
                    desired_maximum_frame_latency: NonZero::new(1),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(DirectionalLightShadowMap { size: 512 })
        .insert_resource(PointLightShadowMap { size: 128 })
        .add_plugins(LODPlugin)
        .add_systems(Startup, (setup_light, camera_setup, spawn_ground, spawn_trees, spawn_grass).chain())
        // .add_systems(Startup, (setup_light, camera_setup, spawn_grass).chain())
        .add_systems(Update, camera_test_move)
        .run();
}

fn spawn_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let plane_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)));
    let lod_mesh = lod_mesh::LodMesh {
        // l1: Some(asset_server.load("models/forest_ground_oak_pine_vol2/ground_l1_l2.glb#Mesh0/Primitive0")),
        // l2: Some(asset_server.load("models/forest_ground_oak_pine_vol2/ground_l1_l2.glb#Mesh1/Primitive0")),
        // l3: Some(plane_mesh),
        l1: Some(plane_mesh.clone()),
        l2: Some(plane_mesh.clone()),
        l3: Some(plane_mesh),
    };

    let lod_distance = lod_distance::LodDistances::new(15.0, 30.0, 500.0);

    let lod_pbr = lod_pbr::LodPbr {
        l1: Some(materials.add(
            StandardMaterial {
                base_color_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/1k/base_color.jpeg")),
                occlusion_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/1k/ambient_occlusion.jpeg")),
                metallic_roughness_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/1k/roughness.jpeg")),
                normal_map_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/1k/normal.jpeg")),
                emissive_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/1k/height.jpeg")),
                ..Default::default()
            }
        )),
        l2: Some(materials.add(
            StandardMaterial {
                base_color_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/256/base_color.jpeg")),
                occlusion_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/256/ambient_occlusion.jpeg")),
                metallic_roughness_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/256/roughness.jpeg")),
                normal_map_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/256/normal.jpeg")),
                emissive_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/256/height.jpeg")),
                ..Default::default()
            }
        )),
        l3: Some(materials.add(
            StandardMaterial {
                base_color_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/32/base_color.jpeg")),
                occlusion_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/32/ambient_occlusion.jpeg")),
                metallic_roughness_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/32/roughness.jpeg")),
                normal_map_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/32/normal.jpeg")),
                emissive_texture: Some(asset_server.load("models/forest_ground_oak_pine_vol2/32/height.jpeg")),
                ..Default::default()
            }
        )),
    };

    for z in -50..50 {
        for x in -50..50 {
            commands.spawn((
                PbrBundle {
                    // mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.475))),
                    mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0))),
                    transform: Transform::from_xyz(4.0 * x as f32, 9.0, 4.0 * z as f32).with_scale(Vec3::splat(2.00)),
                    ..Default::default()
                },
                // lod_mesh.clone(),
                lod_pbr.clone(),
                lod_mesh.clone(),
                lod_distance.clone()
            ));
        }
    }
}

fn setup_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(-100.0, 50.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            color: Color::srgb(0.25, 0.13, 0.11),
            illuminance: 1000000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn spawn_trees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let lod_distance = lod_distance::LodDistances::new(60.0, 200.0, 500.0);

    let lod_mesh_stem = lod_mesh::LodMesh {
        l1: Some(asset_server.load("models/tree/tree.glb#Mesh2/Primitive0")),
        l2: Some(asset_server.load("models/tree/tree.glb#Mesh3/Primitive0")),
        l3: Some(asset_server.load("models/tree/tree.glb#Mesh4/Primitive0")),
    };

    let lod_mesh_leaf = lod_mesh::LodMesh {
        l1: Some(asset_server.load("models/tree/tree.glb#Mesh0/Primitive0")),
        l2: Some(asset_server.load("models/tree/tree.glb#Mesh5/Primitive0")),
        l3: Some(Handle::default()), //Some(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0))))//Some(asset_server.load("models/tree/tree.glb#Mesh6/Primitive0")),
    };

    let stem_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("models/tree/trunk_Base_color.png")),
        metallic_roughness_texture: Some(asset_server.load("models/tree/trunk_Roughness.png")),
        occlusion_texture: Some(asset_server.load("models/tree/trunk_Mixed_AO.png")),
        normal_map_texture: Some(asset_server.load("models/tree/trunk_Normal_OpenGL.png")),
        ..default()
    });

    let leaf_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("models/tree/leaves_color.png")),
        metallic_roughness_texture: Some(asset_server.load("models/tree/leaves_roughness.png")),
        normal_map_texture: Some(asset_server.load("models/tree/leaves_nm.png")),
        alpha_mode: AlphaMode::AlphaToCoverage,
        ..default()
    });

    let mut rng = rand::thread_rng();
    for z in -30..30 {
        for x in -30..30 {
            let rand_range = -7.0..=7.0;
            let x_rand = rng.gen_range(rand_range.clone());
            let z_rand = rng.gen_range(rand_range);

            let random_yaw = rng.gen_range(0.0..std::f32::consts::TAU);

            let tree_transform = Transform::from_xyz(12.0 * x as f32 + x_rand, 8.5, 12.0 * z as f32 + z_rand).with_rotation(Quat::from_rotation_y(random_yaw));

            if rng.gen() {
                commands.spawn((
                    PbrBundle {
                        material: stem_material.clone(),
                        transform: tree_transform.clone(),
                        ..Default::default()
                    },
                    lod_mesh_stem.clone(),
                    lod_distance.clone()
                ));

                if rng.gen() || rng.gen() {
                    commands.spawn((
                        PbrBundle {
                            material: leaf_material.clone(),
                            transform: tree_transform.clone(),
                            ..Default::default()
                        },
                        lod_mesh_leaf.clone(),
                        lod_distance.clone()
                    ));
                }
            }
        }
    }
}


fn spawn_grass(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let lod_distance = lod_distance::LodDistances::new(30.0, 50.0, 200.0);

    let lod_mesh_grass = lod_mesh::LodMesh {
        l1: Some(asset_server.load("models/grass/grass.glb#Mesh0/Primitive0")),
        l2: Some(asset_server.load("models/grass/grass.glb#Mesh1/Primitive0")),
        l3: Some(Handle::default()),
    };

    let grass_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("models/grass/512/Grass.png")),
        metallic_roughness_texture: Some(asset_server.load("models/grass/512/Grass_Roughness.png")),
        normal_map_texture: Some(asset_server.load("models/grass/512/Grass_Normal.png")),
        alpha_mode: AlphaMode::AlphaToCoverage,
        ..default()
    });

    let mut rng = rand::thread_rng();

    for z in -50..50 {
        for x in -50..50 {
            let rand_range = -3.0..=3.0;
            let x_rand = rng.gen_range(rand_range.clone());
            let z_rand = rng.gen_range(rand_range);

            let random_yaw = rng.gen_range(0.0..std::f32::consts::TAU);

            let grass_transform = Transform::from_xyz(x as f32 + x_rand, 9.0, z as f32 + z_rand)
                .with_scale(Vec3::splat(0.4))
                .with_rotation(Quat::from_rotation_y(random_yaw));

            if rng.gen() && rng.gen() {
                commands.spawn((
                    PbrBundle {
                        material: grass_material.clone(),
                        transform: grass_transform,
                        ..Default::default()
                    },
                    lod_mesh_grass.clone(),
                    lod_distance.clone()
                ));
            }
        }
    }
}