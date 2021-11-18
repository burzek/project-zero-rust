use std::f32::MIN;
use std::fmt;
use cgmath::{Point2, point2, Vector2};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::ui::graphics::Graphics;
use crate::ui::inputstate::InputState;

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
    pub(crate) id : &'static str,
    pub(crate) entity_type : EntityType,
    pub(crate) position : Point2<f32>,
    pub(crate) health_points : u8, //in %
    pub(crate) rotation : f32,     //in rads

}

impl Entity {
    pub fn new(id : &'static str, entity_type : EntityType) -> Self {
        return Self {
            id,
            entity_type,
            position : Point2::new(f32::MIN, f32::MIN),  //out of screen
            rotation : 0.0,
            health_points: 0
        }
    }

    pub fn new_f(id : &'static str, entity_type : EntityType, position : Point2<f32>, rotation:f32, health_points :u8) -> Self {
        return Self {
            id,
            entity_type,
            position,
            health_points,
            rotation
        }
    }

}


pub trait EntityInfo {
    fn get_id(&self) -> &str;
    fn get_position(&self) -> Point2<f32>;
    fn set_position(&mut self, x:f32, y:f32);
    fn get_rotation(&self) -> f32;
    fn get_hp(&self) -> u8;
}

impl EntityInfo for Entity {
    fn get_id(&self) -> &str {
        return self.id;
    }

    fn get_position(&self) -> Point2<f32> {
        return self.position;
    }

    fn set_position(&mut self, x:f32, y:f32)  {
        self.position.x = x;
        self.position.y = y;
    }

    fn get_rotation(&self) -> f32 {
        return self.rotation;
    }

    fn get_hp(&self) -> u8 {
        return self.health_points;
    }
}

pub trait EntityUpdater {
    fn update(&mut self, input_state:&InputState, dt:f32) -> ();
}

pub trait EntityRenderer {
    fn render(&self,  g : &mut WindowCanvas) -> ();
}
