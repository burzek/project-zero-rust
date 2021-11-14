use cgmath::{Point2, Vector2};
use log::info;
use crate::objects::entity::{Entity, EntityInfo, EntityRenderer, EntityType};

pub struct Player {
    entity : Entity
}

impl Player {
    pub(crate) fn new() -> Box<Player> {
        return Box::new(Player {
            entity: Entity {
                position: Point2 {x : 0, y : 0},
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

    fn get_position(&self) -> Point2<i32> {
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
    fn update(&self) -> () {
        info!("UPDATING {} [{}]", self.get_id(), self.get_entity_type())
    }

    fn render(&self) -> () {
        todo!()
    }
}
