#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;
// no external terminal window for release
use crate::player::PlayerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::Cuboid};

#[path = "player/player.rs"]
mod player;
use player::Player;

const ASPECT_RATIO: f32 = 16. / 9.;

fn main() {
    App::new()
        .add_plugin(TweakedDefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        })
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(GroundDetection { on_ground: false })
        .add_startup_system(load_world_and_camera)
        .add_startup_system(spawn_sample_platform)
        .add_system(camera_fit_inside_current_level)
        .add_system(spawn_ground_sensor)
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
                    filter: "debug,wgpu_core=warn,wgpu_hal=warn,naga=warn,jigen_tensei=debug"
                        .into(),
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

fn load_world_and_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());

    // Levels & World
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("sandbox_world.ldtk"),
        ..Default::default()
    });
}

fn spawn_sample_platform(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {  // Create 1 pixel by 1 pixel block of colour
                color: Color::rgb(0.25, 0.25, 0.75),
                ..Default::default()
            },
            transform: Transform {  // scale and move to desired size and location
                translation: Vec3::new(300.0, 30.0, 1.0),
                scale: Vec3::new(500.0, 20.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));  // 1/2 * 1 pixel pre-scaling = 0.5
}

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;
                    orthographic_projection.viewport_origin = Vec2::ZERO;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        let height = (level.px_hei as f32 / 9.).round() * 9.;
                        let width = height * ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.x =
                            (player_translation.x - level_transform.translation.x - width / 2.)
                                .clamp(0., level.px_wid as f32 - width);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        let width = (level.px_wid as f32 / 16.).round() * 16.;
                        let height = width / ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.y =
                            (player_translation.y - level_transform.translation.y - height / 2.)
                                .clamp(0., level.px_hei as f32 - height);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}

#[derive(Clone, Default, Component, Resource)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider, &Transform), Added<GroundDetection>>,
) {
    for (entity, shape, transform) in detect_ground_for.iter() {
        if let Some(Cuboid { half_extents }) = shape.raw.0.as_cuboid() {
            commands.entity(entity).with_children(|builder| {
                builder.spawn((
                    Sensor,
                    Collider::cuboid(half_extents.x / 2., 2.),
                    ActiveEvents::COLLISION_EVENTS,
                    Transform::from_translation(
                        Vec3::new(0., -half_extents.y, 0.) / transform.scale,
                    ),
                    GlobalTransform::default(),
                    GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    },
                ));
            });
        }
    }
}
