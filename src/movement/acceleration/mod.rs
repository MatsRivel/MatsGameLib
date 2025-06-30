use super::*;
pub use crate::movement::velocity::{maintained::MaintainedVelocity,instant::InstantVelocity};
pub mod instant;
pub mod angular_instant;
pub mod application;