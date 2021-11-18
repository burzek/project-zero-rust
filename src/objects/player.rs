use std::f32::consts::PI;
use std::f64::consts;
use cgmath::{Point2, Vector2};
use cgmath::num_traits::ToPrimitive;
use log::info;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use crate::Game;
use crate::objects::entity::{Entity, EntityInfo, EntityRenderer, EntityType, EntityUpdater};
use crate::objects::entity::EntityType::PLAYER;
use crate::ui::graphics::Graphics;
use crate::ui::inputstate::InputState;



struct MovementVector {
    speed : f32,
    direction: f32  //in rads
}


pub struct Player {
    entity : Entity,
    movement : MovementVector,
    thrust_in_percent : f32,   //in %
    frame:u32,
}

impl Player {
    const ROTATION_PER_DT : f32 = PI / 1000.0;  // 180/sec
    const THRUST_PER_DT : f32 = 100.0 / 3.0 / 1000.0;   // max thrust 100% is reached in 3secs

    pub(crate) fn new() -> Player {
        return Player {
            entity: Entity::new_f("Player",
                                  EntityType::PLAYER,
                                  Point2::new((crate::WINDOW_WIDTH / 2) as f32, (crate::WINDOW_HEIGHT / 2) as f32),
                                  0.0,
                                  100),
            movement: MovementVector { speed: 0.0, direction: 0.0 },
            thrust_in_percent: 0.0,
            frame:0
        };
    }
}

impl EntityUpdater for Player {
    fn update(&mut self, input_state:&InputState, dt:f32) {
        self.frame = self.frame + 1;
        //update direction
        self.entity.rotation += if input_state.is_key_right() { Player::ROTATION_PER_DT * dt }
            else if input_state.is_key_left() { -Player::ROTATION_PER_DT * dt }
            else {0.0};
        if input_state.is_key_pressed(Keycode::Space) {
            self.thrust_in_percent += Player::THRUST_PER_DT * dt;
            self.thrust_in_percent = self.thrust_in_percent.min(100.0);
        } else {
            self.thrust_in_percent = 0.0;
        }
        //self.speed += dtSpeed;
        //update position

        let x = self.entity.get_position().x + (self.movement.direction.cos() * (20.0 * self.thrust_in_percent / 100.0 / 1000.0));
        let y = self.entity.get_position().y + (self.movement.direction.sin() * (20.0 * self.thrust_in_percent / 100.0/ 1000.0));
        self.entity.set_position(x, y);
        if (self.frame % 25000 == 0) {
            info!("x:{}, dx:{}", self.entity.get_position().x, self.movement.direction.cos() * (self.thrust_in_percent / 100.0));
        }
    }
}

impl EntityRenderer for Player {

    fn render(&self, canvas: &mut WindowCanvas)  {
        let p1 = Point2::new(
            self.entity.get_position().x + 10.0 * (self.entity.get_rotation()).cos(),
            self.entity.get_position().y + 10.0 * (self.entity.get_rotation()).sin());
        let p2 = Point2::new(
            self.entity.get_position().x + 10.0 * (self.entity.get_rotation() - PI).cos(),
            self.entity.get_position().y + 10.0 * (self.entity.get_rotation() - PI).sin());
        let p3 = Point2::new(
            self.entity.get_position().x - 30.0 * (self.entity.get_rotation() + PI/2.0).cos(),
            self.entity.get_position().y - 30.0 * (self.entity.get_rotation() + PI/2.0).sin()
        );

        let icon : [Point; 4] = [
            Point::new(p1.x as i32, p1.y as i32),
            Point::new(p2.x as i32, p2.y as i32),
            Point::new(p3.x as i32, p3.y as i32),
            Point::new(p1.x as i32, p1.y as i32)];
        canvas.set_draw_color(Color::RGB(255,255,255));
        canvas.draw_lines(&icon[..]);
    }
}
