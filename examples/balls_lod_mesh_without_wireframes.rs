mod cam;

use bevy::{
    prelude::*,
};
use bevy_lod::*;
use cam::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LODPlugin)
        .add_systems(Startup, (setup_light, camera_setup, spawn_lodable_uv_spheres_meshes_test).chain())
        .add_systems(Update, camera_test_move)
        .run();
}

fn spawn_lodable_uv_spheres_meshes_test(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let lod_mesh = lod_mesh::LODMesh {
        l1: Some(asset_server.load("models/lod_test.glb#Mesh0/Primitive0")),
        l2: Some(asset_server.load("models/lod_test.glb#Mesh1/Primitive0")),
        l3: Some(asset_server.load("models/lod_test.glb#Mesh2/Primitive0")),
    };
    let lod_distance = lod_distance::LODDistances {
        l1: 8.0,
        l2: 30.0,
        l3: 100.0,
    };

    for z in -10..10 {
        for y in -10..10 {
            for x in -10..10 {
                commands.spawn((
                    PbrBundle {
                        mesh: Handle::default(),
                        material: materials.add(StandardMaterial {
                            // base_color: Color::WHITE,
                            // reflectance: 2.0,
                            perceptual_roughness: 0.3,
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
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(12.0, 12.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 500_000_0.0,
            color: Color::srgb(1.0, 0.0, 0.0),
            shadows_enabled: false,
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-12.0, 12.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 50000000.0,
            color: Color::srgb(0.0, 0.0, 1.0),
            shadows_enabled: false,
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 50000000.0,
            color: Color::srgb(0.0, 0.0, 1.0),
            shadows_enabled: false,
            ..default()
        },
        ..default()
    });
}
