use crate::movement::acceleration::{angular_instant::InstantAngularAcceleration, instant::InstantAcceleration};
use super::*;

pub fn apply_angular_acceleration(time: Res<Time>, mut query: Query<(&mut InstantAngularVelocity, &mut InstantAngularAcceleration)>){
    for (mut vel, mut acc) in query.iter_mut(){
        *acc *= time.delta_secs();
        *vel += *acc;
        acc.clear();
    }
}
pub fn apply_linear_acceleration(time: Res<Time>, mut query: Query<(&mut InstantVelocity, &mut InstantAcceleration)>){
    for (mut vel, mut acc) in query.iter_mut(){
        *acc *= time.delta_secs();
        *vel += *acc;
        acc.clear();
    }
}