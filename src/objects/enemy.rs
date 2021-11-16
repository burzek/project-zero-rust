use cgmath::{Point2, Vector2};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::objects::entity::{Entity, EntityInfo, EntityRenderer, EntityType};
use crate::ui::graphics::Graphics;

pub struct Enemy {
    entity : Entity,
}

impl Enemy {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            entity : Entity {
                position: Point::new(0, 0),
                speed: Vector2 { x: 0, y: 0 },
                hp: 0
            }
        })
    }

}

impl EntityInfo for Enemy {
    fn get_id(&self) -> &str {
       return "ENEMY-";
    }
    fn get_entity_type(&self) -> EntityType {
        return EntityType::ENEMY1;
    }
    fn get_position(&self) -> Point {
        return self.entity.position;
    }
    fn get_speed(&self) -> Vector2<i32> {
        return self.entity.speed;
    }

    fn get_hp(&self) -> u8 {
        return self.entity.hp;
    }
}

impl EntityRenderer for Enemy {
    fn update(&mut self) -> () {
        todo!()
    }

    fn render(&self, g : &mut WindowCanvas) -> () {
        todo!()
    }
}