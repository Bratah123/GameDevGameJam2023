use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

pub const PLAYER_SPEED: f32 = 350.0;
pub const JUMP_VEL: f32 = 65000.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system)
            .add_system(player_movement_system)
            .add_system(read_result_system);
    }
}

fn startup_system(mut commands: Commands) {
    // Spawn Main Character
    /* Create our temporary cube player. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(KinematicCharacterController::default())
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(12.0))
        .insert(Collider::cuboid(50 as Real, 50 as Real))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 300.0, 0.0)))
        .insert(Player);

    // Spawn a ball into the testing world
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)));
}

fn player_movement_system(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut KinematicCharacterController), With<Player>>,
    time: Res<Time>) {
    for (mut velocity, mut controller) in player_query.iter_mut() {
        let mut direction = Vec2::ZERO;
        let mut pressed = false;
        if keys.pressed(KeyCode::D) {
            direction = Vec2::new(1.0, 0.0);
            pressed = true;
        }
        else if keys.pressed(KeyCode::A) {
            direction = Vec2::new(-1.0, 0.0);
            pressed = true;
        }
        if keys.pressed(KeyCode::Space) {
            velocity.linvel = Vec2::new(0., 1.0) * JUMP_VEL * time.delta_seconds();
        }
        if pressed {
            controller.translation = Some(direction * PLAYER_SPEED * time.delta_seconds());
        }
    }
}
fn read_result_system(controllers: Query<&KinematicCharacterControllerOutput>) {
    for controller in controllers.iter() {
        println!("Grounded: {:?}", controller.grounded);
    }
}
