use std::fmt;
use cgmath::Vector2;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::ui::graphics::Graphics;

pub enum EntityType {
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
pub struct Entity {
    pub position : Point,
    pub speed : Vector2<i32>,
    pub hp: u8
}

pub trait EntityInfo {
    fn get_id(&self) -> &str;
    fn get_entity_type(&self) -> EntityType;
    fn get_position(&self) -> Point;
    fn get_speed(&self) -> Vector2<i32>;
    fn get_hp(&self) -> u8;
}

pub trait EntityRenderer {
    fn update(&mut self) -> ();
    fn render(&self, g : &mut WindowCanvas) -> ();
}
