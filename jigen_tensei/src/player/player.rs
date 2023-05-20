use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

pub const PLAYER_SPEED: f32 = 350.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system)
            .add_system(player_movement_system)
            .add_system(apply_player_gravity_system);
    }
}

fn startup_system(mut commands: Commands) {
    // Spawn Main Character
    /* Create our temporary cube player. */
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(50 as Real, 50 as Real))
        .insert(TransformBundle::from(Transform::from_xyz(200.0, 0.0, 0.0)))
        .insert(GravityScale(1.0))
        .insert(KinematicCharacterController::default())
        .insert(Player);

    // Spawn a ball into the testing world
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)));
}

fn player_movement_system(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>) {
    if let Ok(mut controller) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;
        if keys.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        else if keys.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        controller.translation = Some(direction * PLAYER_SPEED * time.delta_seconds());
    }
}

fn apply_player_gravity_system(
    mut player_query: Query<(&mut KinematicCharacterController, &KinematicCharacterControllerOutput), With<Player>>,
    time: Res<Time>
) {
    for (mut controller, output) in player_query.iter_mut() {
        if !output.grounded {
            controller.translation =
                match controller.translation {
                    Some(mut v) => {
                        v.y = -1.0;
                        Some(v)
                    }
                    None => Some(Vec2::new(0.0, -1.0) * PLAYER_SPEED * time.delta_seconds()),
                };
        }
    }
}