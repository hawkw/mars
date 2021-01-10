use bevy::prelude::*;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Reflect, Default)]
#[reflect(Component)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
    pub z: u16,
}
