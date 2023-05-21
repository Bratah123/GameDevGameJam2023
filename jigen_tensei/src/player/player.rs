use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

pub const PLAYER_SPEED: f32 = 350.0;
pub const JUMP_VEL: f32 = 550.0;
const MAX_JUMP_HEIGHT: f32 = 100.0;
const PLAYER_GRAVITY: f32 = 200.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system)
            .add_system(modify_character_controller_impulses)
            .add_system(player_lateral_movement_system)
            .add_system(set_jump_accumulator)
            .add_system(use_jump_accumulator)
            .add_system(fall_system)
            .add_system(read_result_system);
    }
}

fn startup_system(mut commands: Commands) {
    // Spawn Main Character
    /* Create our temporary cube player. */
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true, // Enable collisions
            ..default()
        })
        .insert(Velocity {
            // Initialise as state of rest
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(20 as Real, 50 as Real))
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, 300.0, 1.0, // Set initial position
        )))
        .insert(Player);

    // Spawn a ball into the testing world
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(25.0))
        .insert(GravityScale(3.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            200.0, 300.0, 1.0,
        )));
}

/// Handle collisions between players and dynamic objects
/// by configuring dynamic impulses inside of a system.
fn modify_character_controller_impulses(
    mut character_controllers: Query<&mut KinematicCharacterController>,
) {
    for mut character_controller in character_controllers.iter_mut() {
        character_controller.apply_impulse_to_dynamic_bodies = true;
    }
}

fn player_lateral_movement_system(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    for mut controller in player_query.iter_mut() {
        let mut translation_accumulator = Vec2::ZERO;
        let mut pressed = false;

        // Left and Right
        if keys.pressed(KeyCode::D) {
            translation_accumulator.x = PLAYER_SPEED * time.delta_seconds();
            pressed = true;
        } else if keys.pressed(KeyCode::A) {
            translation_accumulator.x = -1.0 * PLAYER_SPEED * time.delta_seconds();
            pressed = true;
        }

        if pressed {
            // Preserve vertical motion
            match controller.translation {
                Some(existing_motion) => translation_accumulator.y = existing_motion.y,
                None => translation_accumulator.y = 0.0,
            }

            controller.translation = Some(translation_accumulator);
        }
    }
}

/// Jump animation occurs across multiple frames, so we use an accumulator
#[derive(Component)]
struct JumpAccumulator(f32);

/// Adds a jump accumulator to a character if character is on the ground and pressing SPACE
fn set_jump_accumulator(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), (With<KinematicCharacterController>, Without<JumpAccumulator>)>,
) {
    if query.is_empty() {
        return;
    }

    for (entity, controller) in query.iter() {
        if input.pressed(KeyCode::Space) {
            if controller.grounded {
                commands.entity(entity).insert(JumpAccumulator(0.0));
            }
        }
    }
}

/// Consume the jump accumulator to provide jump action
fn use_jump_accumulator(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut JumpAccumulator)>,
) {
    if query.is_empty() {
        return;
    }

    for (entity, mut controller, mut jump) in query.iter_mut() {

        let mut movement = time.delta_seconds() * JUMP_VEL;

        // I have no idea how this bit works - I copy-pasted
        if movement + jump.0 >= MAX_JUMP_HEIGHT {
            movement = MAX_JUMP_HEIGHT - jump.0;
            commands.entity(entity).remove::<JumpAccumulator>();
        }

        jump.0 += movement;

        // Preserve players lateral movement when jumping
        match controller.translation {
            Some(vec) => controller.translation = Some(Vec2::new(vec.x, movement)),
            None => controller.translation = Some(Vec2::new(0.0, movement)),
        }
    }
}

fn fall_system(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<JumpAccumulator>>) {
    if query.is_empty() {
        return;
    }

    for mut controller in query.iter_mut() {
        let movement = time.delta_seconds() * PLAYER_GRAVITY * -1.0;
        // Preserve players lateral movement when falling
        match controller.translation {
            Some(vec) => controller.translation = Some(Vec2::new(vec.x, movement)),
            None => controller.translation = Some(Vec2::new(0.0, movement)),
        }
    }
}

fn read_result_system(controllers: Query<&KinematicCharacterControllerOutput>) {
    for controller in controllers.iter() {
        debug!("Grounded: {:?}", controller.grounded);
    }
}
