mod cam;

use bevy::{
    prelude::*,
};
use bevy_lod::*;
use cam::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LODPlugin)
        .add_systems(Startup, (setup_light, camera_setup, spawn_lodable_cube_textures, spawn_lodable_trees).chain())
        .add_systems(Update, camera_test_move)
        .run();
}

fn spawn_lodable_cube_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let plane_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)));
    let lod_mesh = lod_mesh::LodMesh {
        l1: Some(asset_server.load("models/forest_ground_oak_pine_vol2/ground_l1_l2.glb#Mesh0/Primitive0")),
        l2: Some(asset_server.load("models/forest_ground_oak_pine_vol2/ground_l1_l2.glb#Mesh1/Primitive0")),
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
                metallic: 1.0,
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
                metallic: 1.0,
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
                metallic: 1.0,
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
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(17.0, 20.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 5000_000_0.0,
            // color: Color::srgb(1.0, 0.0, 0.0),
            color: Color::srgb(0.250, 0.214, 0.165),
            shadows_enabled: false,
            range: 30.0,
            shadow_depth_bias: 0.001, // Налаштування глибини тіней
            shadow_normal_bias: 0.001, // Налаштування нормалей тіней
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-15.0, 20.0, -24.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 5000000000.0,
            // color: Color::srgb(0.0, 0.0, 1.0),
            color: Color::srgb(0.250, 0.214, 0.165),
            shadows_enabled: false,
            range: 30.0,
            shadow_depth_bias: 0.001, // Налаштування глибини тіней
            shadow_normal_bias: 0.001, // Налаштування нормалей тіней
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 500000000.0,
            // color: Color::srgb(0.0, 0.0, 1.0),
            color: Color::srgb(0.250, 0.214, 0.165),
            shadows_enabled: false,
            range: 30.0,
            shadow_depth_bias: 0.001, // Налаштування глибини тіней
            shadow_normal_bias: 0.001, // Налаштування нормалей тіней
            ..default()
        },
        ..default()
    });
}

fn spawn_lodable_trees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let lod_distance = lod_distance::LodDistances::new(50.0, 130.0, 500.0);

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

    // let lod_mesh_branches = lod_mesh::LodMesh {
    //     l1: Some(asset_server.load("models/tree/tree.glb#Mesh1/Primitive0")),
    //     l2: Some(asset_server.load("models/tree/tree.glb#Mesh5/Primitive0")),
    //     l3: default()
    // };

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
    for z in -50..50 {
        for x in -50..50 {
            let rand_range = -7.0..=7.0;
            let x_rand = rng.gen_range(rand_range.clone());
            let z_rand = rng.gen_range(rand_range);

            let random_yaw = rng.gen_range(0.0..std::f32::consts::TAU);


            let tree_transform = Transform::from_xyz(12.0 * x as f32 + x_rand, 8.5, 12.0 * z as f32 + z_rand).with_rotation(Quat::from_rotation_y(random_yaw));

            commands.spawn((
                PbrBundle {
                    material: stem_material.clone(),
                    transform: tree_transform.clone(),
                    ..Default::default()
                },
                lod_mesh_stem.clone(),
                lod_distance.clone()
            ));

            commands.spawn((
                PbrBundle {
                    material: leaf_material.clone(),
                    transform: tree_transform.clone(),
                    ..Default::default()
                },
                lod_mesh_leaf.clone(),
                lod_distance.clone()
            ));

            // commands.spawn((
            //     PbrBundle {
            //         material: leaf_material.clone(),
            //         transform: Transform::from_xyz(6.0, 8.5, 6.0),
            //         ..Default::default()
            //     },
            //     lod_mesh_branches.clone(),
            //     lod_distance.clone()
            // ));
        }
    }
}
