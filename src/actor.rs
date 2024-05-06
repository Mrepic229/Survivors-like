use macroquad::prelude::*;
use core::f32::consts::PI;
//use crate::player;


#[derive(Clone, Copy)]
pub struct Actor {
    pub position: Vec2,
    pub velocity: Vec2,

    pub rotation: f32,
    pub rot_velocity: f32,

}

impl Actor {
    pub fn apply_velocity(&mut self) {
        self.position += self.velocity.clone();
        self.rotation += &self.rot_velocity;
    }

    pub fn default_actor() -> Actor{
        Actor{
            position: Vec2 { x: 2.0, y: 2.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            rotation: -PI/2.0,
            rot_velocity:0.0,
        }
    }

}

