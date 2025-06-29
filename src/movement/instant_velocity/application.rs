use crate::movement::instant_velocity::structure::InstantVelocity;
use super::*;

pub fn apply_instant_velocity(mut query: Query<(&mut Transform, &mut InstantVelocity)>){
    for (mut transform, mut velocity) in query.iter_mut(){
        transform.translation += velocity.consume().extend(0.0);
    }
}