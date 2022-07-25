<div align="center">

# Arcade Car Controller

A simple arcade car controller for [Bevy](https://bevyengine.org/)

</div>

## How to use

`Cargo.toml`
```toml
bevy ="0.7"
bevy_arcade_car_controller = { git = "https://github.com/LiamGallagher737/bevy_arcade_car_controller" }
heron = { version = "3.1", features = ["3d"] }
```

`main.rs`
```rs
use bevy::prelude::*;
use bevy_arcade_car_controller::*;
use heron::prelude::*;

fn main() {
    App::new()
    
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ArcadeCarControllerPlugin)
        
        // Systems
        .add_startup_system(startup_system)
        .add_system(drive_system)
        
        .run();
}

fn startup_system(
    mut commands: Commands,
) {
    // Your car model
    let car_entity = commands.spawn()
        // Add Mesh, PbrBundle etc.
        .insert(ArcadeCar)
        .id();
        
    // The car motor
    commands.spawn_bundle(ArcadeCarBundle::new(car_entity, position, size, speed, turn_speed));
    
    // Spawn camera, lights etc.
}

fn drive_system(
    mut query: Query<&mut ArcadeCarInput>,
    keys: Res<Input<KeyCode>>
) {
    // This assumes you only have one car being controlled by the player,
    // If you have multiple do a for loop here instead.
    if let Ok(mut input) = query.get_single_mut() {
        input.reset();
        if keys.pressed(KeyCode::W) {
            input.acceleration += 1.0;
        }
        if keys.pressed(KeyCode::S) {
            input.acceleration -= 0.5;
        }
        if keys.pressed(KeyCode::A) {
            input.turn += 1.0;
        }
        if keys.pressed(KeyCode::D) {
            input.turn -= 1.0;
        }
        // Not implemented yet
        // if keys.pressed(KeyCode::Space) {
        //     input.handbrake = true;
        // }
    }
}
```
