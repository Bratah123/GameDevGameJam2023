#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  // no external terminal window for release
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugin(LoggerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();


}

/// Contains default plugins with tweaks for the logger
///
/// If running in debug mode, warn level is at DEBUG.
/// Release builds have the warn level set to INFO.
struct LoggerPlugin;
impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        // Debug mode
        #[cfg(debug_assertions)]
        app.add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "debug,wgpu_core=warn,wgpu_hal=warn,jigen_tensei=debug".into(),
        }));

        // Release mode
        #[cfg(not(debug_assertions))]
        app.add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
        }));
    }
    // Example log line: debug!("x: {}, state: {:?}", x, state);
    // Example log line: error!("Unknown condition!");
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        debug!("Ball altitude: {}", transform.translation.y);
    }
}