#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // no external terminal window for release
use crate::player::PlayerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[path = "player/player.rs"]
mod player;

fn main() {
    App::new()
        .add_plugin(TweakedDefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        })
        .insert_resource(LevelSelection::Index(0))
        .run();
}

/// Contains default plugins with tweaks for the logger
///
/// If running in debug mode, warn level is at DEBUG.
/// Release builds have the warn level set to INFO.
struct TweakedDefaultPlugins;
impl Plugin for TweakedDefaultPlugins {
    fn build(&self, app: &mut App) {
        // Debug mode
        #[cfg(debug_assertions)]
        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "debug,wgpu_core=warn,wgpu_hal=warn,jigen_tensei=debug".into(),
                })
                .set(ImagePlugin::default_nearest()),
        );

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

fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());

    // Levels & World
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("sandbox_world.ldtk"),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}
