use std::fmt;
use cgmath::{Point2, Vector2};

pub(super) enum EntityType {
    PLAYER,
    ENEMY1
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EntityType::PLAYER => write!(f, "EntityType[PLAYER]"),
            EntityType::ENEMY1 => write!(f, "EntityType[ENEMY1]"),
        }
    }
}

pub(super) struct Entity {
    pub position : Point2<i32>,
    pub speed : Vector2<i32>,
    pub hp: u8
}

pub(super) trait EntityInfo {
    fn get_id(&self) -> &str;
    fn get_entity_type(&self) -> EntityType;
    fn get_position(&self) -> Point2<i32>;
    fn get_speed(&self) -> Vector2<i32>;
    fn get_hp(&self) -> u8;
}

pub trait EntityRenderer {
    fn update(&self) -> ();
    fn render(&self) -> ();
}
