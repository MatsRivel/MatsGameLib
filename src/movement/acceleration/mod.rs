use super::*;
use crate::movement::velocity::{angular_instant::InstantAngularVelocity, instant::InstantVelocity, maintained::MaintainedVelocity};
pub mod instant;
pub mod angular_instant;
pub mod application;