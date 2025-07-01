use super::*;
use crate::movement::velocity::angular_instant::InstantAngularVelocity;
use crate::movement::velocity::instant::InstantVelocity;
use crate::movement::velocity::maintained::MaintainedVelocity;


pub fn apply_instant_velocity(mut query: Query<(&mut Transform, &mut InstantVelocity)>){
    for (mut transform, mut velocity) in query.iter_mut(){
        *transform += &mut *velocity;
    }
}

pub fn apply_maintained_velocity(mut query: Query<(&mut Transform, &mut MaintainedVelocity)>){
    for (mut transform, mut velocity) in query.iter_mut(){
        *transform += &mut *velocity;
    }
}

pub fn apply_instant_angular_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut InstantAngularVelocity)>,
){
    for (mut transform, mut angular_velocity) in query.iter_mut() {
        let delta_angle = angular_velocity.consume() * time.delta_secs();
        let delta_rotation = Quat::from_rotation_z(delta_angle);
        transform.rotation *= delta_rotation;
    }
}