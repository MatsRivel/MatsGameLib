use super::*;
pub mod player;

#[derive(Component,Default)]
#[require(Transform, Sprite)]
pub struct Object;