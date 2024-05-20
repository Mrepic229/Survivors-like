use crate::Actor;
use crate::util;

use macroquad::prelude::*;
use core::f32::consts::PI;
use std::clone;

const ENEMY_SPEED: f32 = 2.0;

#[derive(Clone)]
pub struct Enemy {
    pub body: Actor,
    pub radius: f32,
    pub speed: f32
}

pub struct EnemyVec {
    pub vec: Vec<Enemy>
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            body: Actor {
                position: Vec2 { x: 0.0, y: 0.0 },
                velocity: Vec2 { x: 0.0, y: 0.0 }, 
                rotation: 0.0, 
                rot_velocity: 0.0 
            },
            radius: 5.0,
            speed: ENEMY_SPEED,
        }
    }

    pub fn move_pos(&mut self, target: Vec2) {
        let rotation = util::get_angle(self.body.position, target);
        self.body.position += Vec2{x: rotation.cos(), y: rotation.sin()} * self.speed;
    }

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
        draw_triangle(v1, v2, v3, RED)
    }

}

impl EnemyVec {
    pub fn new() -> EnemyVec {
        let temp: EnemyVec = EnemyVec { vec: Vec::new() };
        return temp;
    }

    pub fn move_pos(&mut self, target: Vec2) {
        for i in 0..self.vec.clone().into_iter().len() {
            self.vec[i].move_pos(target);
        }
    }

    pub fn draw(&self) {
        for i in 0..self.vec.clone().into_iter().len() {
            self.vec[i].draw();
        }
    }
}