use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system)
            .add_system(print_cube_altitude_system);
    }
}

fn startup_system(mut commands: Commands) {
    // Spawn Main Character
    /* Create our temporary cube player. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(50 as Real, 50 as Real))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(Player);
}

fn print_cube_altitude_system(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("cube altitude: {}", transform.translation.y);
    }
}