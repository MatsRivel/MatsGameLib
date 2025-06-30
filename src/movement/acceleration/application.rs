use crate::movement::acceleration::angular_instant::InstantAngularAcceleration;
use crate::movement::velocity::angular_maintained::MaintainedAngularVelocity;
use crate::movement::acceleration::instant::InstantAcceleration;
use super::*;

pub fn apply_angular_acceleration(time: Res<Time>, mut query: Query<(&mut MaintainedAngularVelocity, &mut InstantAngularAcceleration)>){
    for (mut vel, mut acc) in query.iter_mut(){
        *acc *= time.delta_secs();
        *vel += *acc;
        acc.clear();
    }
}
pub fn apply_linear_acceleration(time: Res<Time>, mut query: Query<(&mut MaintainedVelocity, &mut InstantAcceleration)>){
    for (mut vel, mut acc) in query.iter_mut(){
        *acc *= time.delta_secs();
        *vel += *acc;
        acc.clear();
    }
}