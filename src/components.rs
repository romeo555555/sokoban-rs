use ggez::{graphics, mint};

/// Path to the image that represents the entity
pub enum Renderable {
    Image(graphics::Image),
}

/// Position of the entity on the game's grid
pub type Position = mint::Point3<u8>;

/// Marks an entity as a player
#[derive(Default)]
pub struct Player;

#[derive(PartialEq, Copy, Clone)]
pub enum BoxColor {
    Blue,
    Red,
}

/// Marks an entity as a box
pub struct Box {
    pub color: BoxColor,
}

/// Marks an entity as a wall
#[derive(Default)]
pub struct Wall;

/// Marks an entity as a location where a box can be put into
pub struct BoxSpot {
    pub color: BoxColor,
}

#[derive(Default)]
pub struct Moveable;

#[derive(Default)]
pub struct Immovable;
