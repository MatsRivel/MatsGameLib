use crate::movement::{acceleration::{angular_instant::InstantAngularAcceleration, instant::InstantAcceleration}, velocity::{angular_maintained::MaintainedAngularVelocity, instant::InstantVelocity, maintained::MaintainedVelocity}};
use super::*;
#[derive(Component, Default)]
#[require(Object, InstantVelocity)]
pub struct PlayerTag;

// Used when velocity is reset every turn. There is no acceleration, only instantaneous velocity.
pub fn apply_velocity_to_single_from_keyboard<T:Component>(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&Transform, &mut InstantVelocity), With<T>>,
){
    let (transform, mut velocity) = player_query.into_inner();
    let left = keyboard_input.pressed(KeyCode::KeyQ);
    let right = keyboard_input.pressed(KeyCode::KeyE);
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let sideways_momentum = match (left,right){
        (true,false) => -1.0,
        (false,true) => 1.0,
        (true,true) | (false,false) => 0.0
    };
    let forwards_momentum = match (up,down){
        (true,false) => 1.0,
        (false,true) => -1.0,
        (true,true) | (false,false) => 0.0
    };
    let momentum = vec2(sideways_momentum,forwards_momentum);
    if momentum == Vec2::ZERO{
        return;
    }
    let rotation_adjusted_movement = transform.rotation.mul_vec3(momentum.extend(0.0)).truncate();
    let delta_time_movement = rotation_adjusted_movement*time.delta_secs();
    *velocity += delta_time_movement;

}

/// Used in combination with maintained velocity. You apply a change to velocity each tick, but 
pub fn apply_acceleration_to_single_from_keyboard<T:Component>(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&Transform, &mut InstantAcceleration), (With<T>, With<MaintainedVelocity>)>,
){
    let (transform, mut acceleration) = player_query.into_inner();
    let left = keyboard_input.pressed(KeyCode::KeyQ);
    let right = keyboard_input.pressed(KeyCode::KeyE);
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let sideways_momentum = match (left,right){
        (true,false) => -1.0,
        (false,true) => 1.0,
        (true,true) | (false,false) => 0.0
    };
    let forwards_momentum = match (up,down){
        (true,false) => 1.0,
        (false,true) => -1.0,
        (true,true) | (false,false) => 0.0
    };
    let momentum = vec2(sideways_momentum,forwards_momentum);
    if momentum == Vec2::ZERO{
        return;
    }
    let rotation_adjusted_movement = transform.rotation.mul_vec3(momentum.extend(0.0)).truncate();
    let delta_time_movement = rotation_adjusted_movement*time.delta_secs();
    *acceleration += delta_time_movement;

}

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,    (
            spawn_player, 
        )).add_systems(Update, apply_velocity_to_single_from_keyboard::<PlayerTag>);
    }
}
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    let asset_path = r"sprites\Ships\ship-a\ship-a1.png";
    let image = asset_server.load(asset_path);
    let mut sprite =     Sprite::from_image(image);
    sprite.custom_size = Some(Vec2::splat(128.0));
    let player_entity = commands.spawn((
        PlayerTag,
        sprite,
        MaintainedVelocity::default(),
        MaintainedAngularVelocity::default(),
        InstantAcceleration::default(),
        InstantAngularAcceleration::default(),
    )).id();
    println!("Player Entity: {player_entity:?}");
}