mod actor;
mod player;

use std::default;

use actor::Actor;
use player::Player;
use macroquad::prelude::*;


#[macroquad::main("BasicShapes")]
async fn main() {
    game_loop().await;
    
}

async fn game_loop() {
    let mut camera: Camera2D = Camera2D {
        //offset: Vec2 { x: 1.0, y: 1.0 },
        //target: Vec2 { x: 2.0, y: 2.0 },
        zoom: Vec2 { x: 0.001, y: 0.001 },
        ..Default::default()
    };

    let mut player: Player = Player{
        body: Actor::default_actor(),
        radius: 20.0,
    };

    set_camera(&camera);
    loop{
        clear_background(WHITE);

        player.direction();

        player.body.apply_velocity();
        //camera.target = player.body.position;
        //set_camera(&camera);

        player.draw();
        draw_line(-500.0, -500.0, 500.0, 500.0, 3.0, RED);

        

        next_frame().await;
    }
}
