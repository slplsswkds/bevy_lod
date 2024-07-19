use bevy::prelude::*;
use bevy::core_pipeline::Skybox;
use bevy::pbr::VolumetricFogSettings;

#[derive(Component)]
pub struct CamMove {
    x_direct: bool,
    start: Vec3,
    finish: Vec3,
    progress: f32,
    speed: f32,
}

pub fn camera_setup(
    mut commands: Commands,
) {

    // Camera in 3D space.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-40.0, 10.0, -30.5).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            ..default()
        },
        CamMove {
            x_direct: true,
            start: Vec3::new(-20.0, 10.0, -40.5),
            finish: Vec3::new(17.0, 10.0, 10.0),
            progress: 0.0,
            speed: 0.04,
        },
        bevy::core_pipeline::bloom::BloomSettings::NATURAL
        // Skybox {
        //     image: asset_server.load(".jpeg"),
        //     brightness: 1000.0,
        // },
        // VolumetricFogSettings {
        //     // This value is explicitly set to 0 since we have no environment map light
        //     ambient_intensity: 0.0,
        //     ..default()
        // },
    ));
}

pub fn camera_test_move(
    mut query_cam: Query<(&mut Transform, &mut CamMove), With<Camera3d>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    let (mut transform, mut cam_move) = query_cam.get_single_mut().unwrap();

    cam_move.progress = (cam_move.progress + time.delta_seconds() * cam_move.speed).min(1.0); // обмеження прогресу до 1.0
    if cam_move.x_direct && transform.translation != cam_move.finish {
        transform.translation = cam_move.start.lerp(cam_move.finish, cam_move.progress);
    } else if cam_move.x_direct && transform.translation == cam_move.finish {
        cam_move.x_direct = false;
        cam_move.progress = 0.0;
    } else if !cam_move.x_direct && transform.translation != cam_move.start {
        transform.translation = cam_move.finish.lerp(cam_move.start, cam_move.progress);
    } else if !cam_move.x_direct && transform.translation == cam_move.start {
        cam_move.x_direct = true;
        cam_move.progress = 0.0;
    } else {}


    // transform.look_at(Vec3::ZERO, Vec3::Y);
    transform.look_at(Vec3::new(5.0, 10.0, 0.0), Vec3::Y);
}
