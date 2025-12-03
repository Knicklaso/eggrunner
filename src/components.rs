use bracket_lib::prelude::{FontCharType, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
