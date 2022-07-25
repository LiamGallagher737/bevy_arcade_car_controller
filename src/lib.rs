// This is a workaround for issue: https://github.com/bevyengine/bevy/issues/4601
#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;
use heron::prelude::*;

pub struct ArcadeCarControllerPlugin;
impl Plugin for ArcadeCarControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin::default());
        app.add_system(drive_car_system);
        app.add_system(turn_car_system);
        app.add_system(handbrake_car_system);
        app.add_system(copy_position_system);
    }
}

#[derive(Bundle)]
pub struct ArcadeCarBundle {
    motor: ArcadeCarMotor,
    input: ArcadeCarInput,
    transform: Transform,
    global_transform: GlobalTransform,
    acceleration: Acceleration,
    velocity: Velocity,
    damping: Damping,
    rigidbody: RigidBody,
    collider: CollisionShape,
    physics_material: PhysicMaterial,
}

impl ArcadeCarBundle {
    pub fn new(car_entity: Entity, mut position: Vec3, size: f32, speed: f32, turn_speed: f32) -> Self {
        let radius = size / 2.0;
        position.y += radius;
        let transforms = TransformBundle::from_transform(Transform::from_translation(position));
        Self {
            motor: ArcadeCarMotor {
                car_entity,
                radius,
                speed,
                turn_speed,
            },
            input: ArcadeCarInput::default(),
            transform: transforms.local,
            global_transform: transforms.global,
            acceleration: Acceleration::default(),
            velocity: Velocity::default(),
            damping: Damping::from_linear(1.0),
            rigidbody: RigidBody::Dynamic,
            collider: CollisionShape::Sphere { radius },
            physics_material: PhysicMaterial {
                restitution: 0.2,
                density: 2500.0,
                friction: 1.0,
            },
        }
    }
}

#[derive(Component)]
pub struct ArcadeCar;

#[derive(Component)]
pub struct ArcadeCarMotor {
    car_entity: Entity,
    radius: f32,
    speed: f32,
    turn_speed: f32,
}

#[derive(Component)]
pub struct ArcadeCarInput {
    pub acceleration: f32,
    pub turn: f32,
    pub handbrake: bool,
}

impl Default for ArcadeCarInput {
    fn default() -> Self {
        Self {
            acceleration: 0.0,
            turn: 0.0,
            handbrake: false,
        }
    }
}

impl ArcadeCarInput {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

fn drive_car_system(
    mut query_motor: Query<
        (&mut Acceleration, &ArcadeCarInput, &ArcadeCarMotor),
        Without<ArcadeCar>,
    >,
    query_car: Query<&Transform, With<ArcadeCar>>,
) {
    for (mut acceleration, input, motor) in query_motor.iter_mut() {
        if input.handbrake {
            continue;
        }
        if let Ok(car_t) = query_car.get(motor.car_entity) {
            acceleration.linear = car_t.forward() * -input.acceleration * motor.speed;
        } else {
            warn_cant_find_car(motor.car_entity);
        }
    }
}

fn turn_car_system(
    mut query_car: Query<&mut Transform, With<ArcadeCar>>,
    query_motor: Query<(&ArcadeCarMotor, &ArcadeCarInput, &Velocity), Without<ArcadeCar>>,
    time: Res<Time>,
) {
    for (motor, input, velocity) in query_motor.iter() {
        if let Ok(mut car_t) = query_car.get_mut(motor.car_entity) {
            let speed = velocity.linear.length();
            let mut turn_speed = motor.turn_speed;
            if speed.abs() < 10.0 {
                turn_speed *= speed / 10.0;
            }
            car_t.rotate(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                input.turn * speed.clamp(-1.0, 1.0) * turn_speed * time.delta_seconds(),
                0.0,
            ));
        } else {
            warn_cant_find_car(motor.car_entity);
        }
    }
}

fn handbrake_car_system(
    mut query: Query<(&mut Damping, &ArcadeCarInput)>,
) {
    for (mut damping, input) in query.iter_mut() {
        if input.handbrake {
            *damping = Damping::from_linear(5.0);
        } else {
            *damping = Damping::from_linear(1.0);
        }
    }
}

fn copy_position_system(
    mut query_car: Query<&mut Transform, With<ArcadeCar>>,
    query_motor: Query<(&ArcadeCarMotor, &Transform), Without<ArcadeCar>>,
) {
    for (motor, transform) in query_motor.iter() {
        if let Ok(mut car_t) = query_car.get_mut(motor.car_entity) {
            let mut new_position = transform.translation;
            new_position.y -= motor.radius;
            car_t.translation = new_position;
            // car_t.look_at(velocity.linear.normalize(), Vec3::Y);
        } else {
            warn_cant_find_car(motor.car_entity);
        }
    }
}

fn warn_cant_find_car(car_entity: Entity) {
    warn!("An arcade motor points to entity {:?} that does not container enither a `Transform` or a `ArcadeCar` or may have been despawned", car_entity);
}
