mod actor;
mod player;
mod enemy;
mod util;

use std::default;

use actor::Actor;
use player::Player;
use enemy::{Enemy, EnemyVec};
use macroquad::prelude::*;
use util::get_random_offscreen_pos;


#[macroquad::main("BasicShapes")]
async fn main() {
    game_loop().await;
    
}

async fn game_loop() {
    let mut camera: Camera2D = Camera2D {
        ..Default::default()
    };

    let mut player: Player = Player{
        body: Actor::default_actor(),
        radius: 20.0,
    };

    let mut enemy_vec:EnemyVec = EnemyVec::new();

    set_camera(&camera);

    loop{
        clear_background(WHITE);

        player.direction();
        player.body.apply_velocity();

        camera = Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: -screen_height() });
        camera.target = player.body.position;
        set_camera(&camera);

        let mut new_enemy = Enemy::new();
        new_enemy.body.position = util::get_random_offscreen_pos(&camera);
        enemy_vec.vec.push(new_enemy);
        enemy_vec.move_pos(player.body.position);


        enemy_vec.draw();
        player.draw();
        draw_line(-500.0, -500.0, 500.0, 500.0, 3.0, RED);
        draw_line(500.0, -500.0, -500.0, 500.0, 3.0, RED);

        next_frame().await;
    }
}
