use cgmath::{Point2, Vector2};
use log::info;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use crate::objects::entity::{Entity, EntityInfo, EntityRenderer, EntityType};
use crate::ui::graphics::Graphics;

pub struct Player {
    entity : Entity
}

impl Player {
    pub(crate) fn new() -> Box<Player> {
        return Box::new(Player {
            entity: Entity {
                position: Point::new(100,100),
                speed: Vector2 { x : 0, y : 0},
                hp: 100
            }
        });
    }
}

impl EntityInfo for Player {
    fn get_id(&self) -> &str {
        return "PLAYER-ID";
    }


    fn get_entity_type(&self) -> EntityType {
        return EntityType::PLAYER;
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

impl EntityRenderer for Player {
    fn update(&mut self) -> () {
        self.entity.position = self.entity.position.offset(3,4);
    }

    fn render(&self, canvas: &mut WindowCanvas)  {
        canvas.set_draw_color(Color::RGB(255,255,255));
        let p1 = self.get_position();
        let p2 = p1.offset(20, 0);
        let p3 = p1.offset(10, -20);
        let icon : [Point; 4] = [p1, p2, p3, p1];
        canvas.draw_lines(&icon[..]);
        // canvas.draw_line(self.get_position(), self.get_position().offset(20,0));
        // canvas.draw_line(self.get_position(), self.get_position().offset(20,0));
    }
}
