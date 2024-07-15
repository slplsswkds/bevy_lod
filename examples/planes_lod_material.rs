mod cam;

use bevy::{
    prelude::*,
    pbr::CascadeShadowConfigBuilder,
};
use core::f32::consts::PI;
use bevy::render::render_resource::TextureViewDimension::Cube;
use bevy_lod::*;
use lod_pbr::LodPbr;
use cam::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LODPlugin)
        .add_systems(Startup, (setup_light, camera_setup, spawn_lodable_cube_textures).chain())
        .add_systems(Update, camera_test_move)
        .run();
}

fn spawn_lodable_cube_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let lod_mesh = lod_mesh::LodMesh {
        l1: Some(asset_server.load("models/lod_test.glb#Mesh0/Primitive0")),
        l2: Some(asset_server.load("models/lod_test.glb#Mesh1/Primitive0")),
        l3: Some(asset_server.load("models/lod_test.glb#Mesh2/Primitive0")),
    };

    let lod_distance = lod_distance::LodDistances::new(10.0, 30.0, 200.0);

    let lod_pbr = lod_pbr::LodPbr {
        l1: Some(materials.add(
            StandardMaterial {
                base_color_texture: Some(asset_server.load("models/wood/textures_2k/wood_color.png")),
                occlusion_texture: Some(asset_server.load("models/wood/textures_2k/wood_ao.png")),
                metallic_roughness_texture: Some(asset_server.load("models/wood/textures_2k/wood_roughness.png")),
                normal_map_texture: Some(asset_server.load("models/wood/textures_2k/wood_normal_directx.png")),
                // clearcoat_roughness_texture: Some(asset_server.load("models/wood/textures_2k/wood_ao.png")),
                ..Default::default()
            }
        )),
        l2: Some(materials.add(
            StandardMaterial {
                base_color_texture: Some(asset_server.load("models/wood/textures_128/wood_color.png")),
                occlusion_texture: Some(asset_server.load("models/wood/textures_128/wood_ao.png")),
                metallic_roughness_texture: Some(asset_server.load("models/wood/textures_128/wood_roughness.png")),
                normal_map_texture: Some(asset_server.load("models/wood/textures_128/wood_normal_directx.png")),
                ..Default::default()
            }
        )),
        l3: Some(materials.add(
            StandardMaterial {
                base_color_texture: Some(asset_server.load("models/wood/textures_32/wood_color.png")),
                occlusion_texture: Some(asset_server.load("models/wood/textures_32/wood_ao.png")),
                metallic_roughness_texture: Some(asset_server.load("models/wood/textures_32/wood_roughness.png")),
                normal_map_texture: Some(asset_server.load("models/wood/textures_32/wood_normal_directx.png")),
                ..Default::default()
            }
        )),
    };

    for z in -50..50 {
        for x in -50..50 {
            commands.spawn((
                PbrBundle {
                    // mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.475))),
                    mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5))),
                    transform: Transform::from_xyz(x as f32, 9.0, z as f32).with_scale(Vec3::splat(1.00)),
                    ..Default::default()
                },
                // lod_mesh.clone(),
                lod_pbr.clone(),
                lod_distance.clone()
            ));
        }
    }
}

fn spawn_lodable_uv_spheres_meshes_test(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let lod_mesh = lod_mesh::LodMesh {
        l1: Some(asset_server.load("models/lod_test.glb#Mesh0/Primitive0")),
        l2: Some(asset_server.load("models/lod_test.glb#Mesh1/Primitive0")),
        l3: Some(asset_server.load("models/lod_test.glb#Mesh2/Primitive0")),
    };

    let lod_distance = lod_distance::LodDistances::new(15.0, 50.0, 200.0);

    for z in -100..100 {
        for y in 9..10 {
            for x in -100..100 {
                commands.spawn((
                    PbrBundle {
                        material: materials.add(StandardMaterial {
                            occlusion_texture: Some(asset_server.load("models/wood/textures_128/wood_ao.png")),
                            base_color_texture: Some(asset_server.load("models/wood/textures_128/wood_color.png")),
                            // metallic_roughness_texture: Some(asset_server.load("models/wood/textures/Wood1_Metallic_2K.png")),
                            normal_map_texture: Some(asset_server.load("models/wood/textures_128/wood_normal_directx.png")),
                            // clearcoat_roughness_texture: Some(asset_server.load("models/wood/textures/Wood1_Roughness_2_2K.png")),
                            metallic_roughness_texture: Some(asset_server.load("models/wood/textures_128/wood_roughness.png")),
                            // base_color: Color::WHITE,
                            // reflectance: 2.0,
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32).with_scale(Vec3::new(0.35, 0.35, 0.35)),
                        ..Default::default()
                    },
                    lod_mesh.clone(),
                    lod_distance.clone()
                ));
            }
        }
    }
}

fn setup_light(mut commands: Commands) {
    // Light up the scene.
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(12.0, 12.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     point_light: PointLight {
    //         intensity: 500_000_0.0,
    //         // color: Color::srgb(1.0, 0.0, 0.0),
    //         color: Color::srgb(0.5, 0.5, 0.5),
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     ..default()
    // });
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(-12.0, 12.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     point_light: PointLight {
    //         intensity: 50000000.0,
    //         // color: Color::srgb(0.0, 0.0, 1.0),
    //         color: Color::srgb(0.5, 0.5, 0.5),
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     ..default()
    // });
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     point_light: PointLight {
    //         intensity: 50000000.0,
    //         // color: Color::srgb(0.0, 0.0, 1.0),
    //         color: Color::srgb(0.5, 0.5, 0.5),
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     ..default()
    // });
    // ambient light

    // commands.insert_resource(AmbientLight {
    //     color: Color::srgb(1.0, 1.0, 1.0),
    //     brightness: 5000.0,
    // });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 50.0,
        //     ..default()
        // }.into(),
        ..default()
    });
}
