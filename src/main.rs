#[macro_use]
extern crate glium;
extern crate alga;
extern crate bytebuffer;
extern crate image;
extern crate nalgebra;
extern crate noise;
extern crate rand;

mod camera;
mod game;
mod graphics;
mod input;
mod nbt;
mod object;
mod quaternion;
mod utils;

use camera::*;
use game::*;
use graphics::*;
use input::*;
use object::*;
//use nbt::*;
//use quaternion::*;

use nalgebra::Vector3;
use std::time::Instant;

fn main() {
    let mut player = Player::new();
    let mut camera = Camera::new(90);
    let mut input = Input::new();
    let mut graphics = Graphics::new();
    let mut blocks = Blocks::new();

    camera.position = Vector3::new(32.0, 64.0, 32.0);
    player.creative = true;

    let projection_matrix: [[f32; 4]; 4] = camera
        .create_projection_matrix(graphics.display.get_framebuffer_dimensions())
        .into();

    let mut window_closed = false;
    let mut prev_time = Instant::now();
    let mut cur_time = Instant::now();

    let skybox = Skybox;

    while !window_closed {
        prev_time = cur_time;
        cur_time = Instant::now();
        let dt: f32 = (cur_time - prev_time).subsec_nanos() as f32 / 1_000_000_000.0;
        let view_matrix: [[f32; 4]; 4] = camera.get_view_matrix().try_inverse().unwrap().into();
        let graphics_params = GraphicsParams::new(view_matrix, projection_matrix);

        graphics.start();

        skybox.draw(&mut graphics, &graphics_params);

        graphics.finish();
    }
}
