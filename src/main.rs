#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate alga;
extern crate noise;

mod camera;
mod object;
mod utils;
mod game;
mod input;

#[derive(Clone, Copy)]
pub struct Instance {
    pub matrix: [[f32; 4]; 4],
    pub id: u8
}

fn main() {
    use glium::{glutin, Surface};
    use glium::glutin::VirtualKeyCode;
    use std::time::Instant;
    use std::time::Duration;
    use std::thread;
    use noise::{Perlin, NoiseModule, Seedable};

    let mut camera: camera::Camera = camera::Camera::new(90);
    let mut game_input: input::Input = input::Input::new();

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_title("Rust Minecraft");
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let mut display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();
    let perlin = Perlin::new();
    perlin.set_seed(1);

    display.gl_window().window().set_cursor_state(glium::glutin::CursorState::Hide).unwrap();

    let params = &glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        .. Default::default()
    };

    let mut blocks: game::Blocks = game::Blocks::new();
    blocks.initialize();

    let blocks_count: f32 = blocks.block_map.len() as f32 - 1.0;

    let mut game: game::Game = game::Game::new(0, 2);
    for x in 0..31 {
        for z in 0..31 {
            let height: u8 = 64 + ((perlin.get([x as f32 / 10.0 + 0.5, z as f32 / 10.0 + 0.5])) * 4.0) as u8;
            for y in 0..height {
                let mut block = 1;
                if height - y <= 4 {
                    block = 2;
                }
                game.world.set_block(x, y, z, blocks.get_block(block));
            }
        }
    }

    let screen_size = display.get_framebuffer_dimensions();

    let mut closed = false;

    use game::Vertex;
	implement_vertex!(Vertex, position, uv);
    implement_vertex!(Instance, matrix, id);

	let vertex_shader_src = utils::file_to_string("shaders/vertex.glsl");
	let fragment_shader_src = utils::file_to_string("shaders/fragment.glsl");

	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
    let projection_matrix: [[f32; 4]; 4] = camera.create_projection_matrix(screen_size).into();
    let vertex_buffer = &game::Block::get_vertex_buffer(&mut display);
    let instance_buffer = &game.world.get_instance_buffer(&mut display);
    let index_buffer = &game::Block::get_index_buffer(&mut display);
    let sampler_raw = glium::texture::Texture2d::new(&mut display, utils::load_image_from_file("textures/blocks/atlas.png")).unwrap();
    let sampler = sampler_raw.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

    camera.translate(utils::get_up_vector() * 64.0);

    let mut dx : f32 = 0.0;
    let mut dy : f32 = 0.0;
    while !closed {
        let start = Instant::now();

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        let view_matrix: [[f32; 4]; 4] = camera.get_view_matrix().try_inverse().unwrap().into();

        target.draw((vertex_buffer, instance_buffer.per_instance().unwrap()), index_buffer, &program, &uniform! { sampler: sampler, view_matrix: view_matrix, projection_matrix: projection_matrix, total_blocks: blocks_count }, params).unwrap();
        target.finish().unwrap();

        let ms = start.elapsed().as_secs() * 1000;
        if ms < 10 {
            thread::sleep(Duration::from_millis(10 - ms));
        }

        if game_input.get_key(VirtualKeyCode::W) {
            let forward = camera.forward();
            camera.translate(forward / 32.0);
        } else if game_input.get_key(VirtualKeyCode::S) {
            let forward = camera.forward();
            camera.translate(-forward / 32.0);
        }

        if game_input.get_key(VirtualKeyCode::A) {
            let right = camera.right();
            camera.translate(-right / 32.0);
        } else if game_input.get_key(VirtualKeyCode::D) {
            let right = camera.right();
            camera.translate(right / 32.0);
        }

        if game_input.get_key(VirtualKeyCode::Space) {
            camera.translate(utils::get_up_vector() / 32.0);
        } else if game_input.get_key(VirtualKeyCode::LControl) {
            camera.translate(-utils::get_up_vector() / 32.0);
        }

        if game_input.get_key(VirtualKeyCode::Escape) {
            return;
        }

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                	glutin::WindowEvent::Closed => closed = true,
                	glutin::WindowEvent::KeyboardInput { input, .. } => match input.state {
                        glutin::ElementState::Pressed => match input.virtual_keycode {
                            Some(key) => game_input.set_key(key, true),                 
                		    _ => ()
                        },
                        glutin::ElementState::Released => match input.virtual_keycode {                 
                            Some(key) => game_input.set_key(key, false),
                		    _ => ()
                        }
                	},
                    glutin::WindowEvent::MouseMoved{position, ..} => {
                       dx = (screen_size.0 / 2) as f32 - position.0 as f32;
                       dy = (screen_size.1 / 2) as f32 - position.1 as f32;
                       camera.rot_x += dx / 10.0 / (180.0 / std::f32::consts::PI);
                       camera.rot_y = utils::clamp(camera.rot_y, -(3.14 / 2.0), 3.14 / 2.0);

                       camera.rot_y += dy / 10.0 / (180.0 / std::f32::consts::PI);
                   },
                	_ => ()
                },
                _ => ()
            }
        });

        display.gl_window().window().set_cursor_position(screen_size.0 as i32 / 2, screen_size.1 as i32 / 2).unwrap();
    }
}