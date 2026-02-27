use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Block {
    #[warn(dead_code)]
    pub internal: u8,
}