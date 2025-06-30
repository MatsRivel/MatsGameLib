use super::*;
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