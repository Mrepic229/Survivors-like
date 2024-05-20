use crate::Actor;

use macroquad::prelude::*;
use core::f32::consts::PI;
//use crate::player;

const PLAYER_SPEED: f32 = 10.0;

pub struct Player {
    pub body: Actor,
    pub radius: f32
}

impl Player{
    pub fn draw(&self) {
        let radius = self.radius;
        let v1: Vec2 = Vec2 {
            x: self.body.position.x + (radius * self.body.rotation.cos()),
            y: self.body.position.y + (radius * self.body.rotation.sin())
        };
        let v2: Vec2 = Vec2 {
            x: self.body.position.x - (radius * (self.body.rotation + PI/4.0).cos()),
            y: self.body.position.y - (radius * (self.body.rotation + PI/4.0).sin())
        };
        let v3: Vec2 = Vec2 {
            x: self.body.position.x - (radius * (self.body.rotation - PI/4.0).cos()),
            y: self.body.position.y - (radius * (self.body.rotation - PI/4.0).sin())
        };
        draw_triangle(v1, v2, v3, BLACK)
    }

    pub fn direction(&mut self) {
        let mut final_velocity:Vec2 = Vec2 { x: 0.0, y: 0.0};

        if is_key_down(KeyCode::Up) {
            final_velocity.y += -1.0;
        }
        if is_key_down(KeyCode::Down) {
            final_velocity.y += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            final_velocity.x += -1.0;
        }
        if is_key_down(KeyCode::Right) {
            final_velocity.x += 1.0;
        }
        if final_velocity.x == 0.0 && final_velocity.y == 0.0 {
            self.body.velocity = Vec2 {x: 0.0, y: 0.0};
            return;
        }
        self.body.velocity = final_velocity.normalize() * PLAYER_SPEED;
        //println!("{}", self.body.velocity);
    }
}