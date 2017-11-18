#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;

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

    let mut camera: camera::Camera = camera::Camera::new(90);
    let mut game_input: input::Input = input::Input::new();

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_title("Rust Minecraft");
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let mut display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();

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

    let mut game: game::Game = game::Game::new(0, 2);

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                game.world.set_block(x, y, z, blocks.get_block(1));
            }
        }
    }
    for z in 0..16 {
        game.world.set_block(2, 2, z, blocks.get_block(0));
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

    while !closed {
        let start = Instant::now();

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        let view_matrix: [[f32; 4]; 4] = camera.get_view_matrix().try_inverse().unwrap().into();
        //let mut translation_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();

        /*for chunk_x in 0..game.world.chunks.len() {
            for chunk_z in 0..game.world.chunks[chunk_x].len() {
                let chunk = game.world.get_chunk(chunk_x, chunk_z);
                let visible_blocks = chunk.visible_blocks.iter();

                for block_pos in visible_blocks {
                    let block_world_x = (chunk_x * 16) + block_pos.x as usize;
                    let block_world_z = (chunk_z * 16) + block_pos.z as usize;
                    let block: &game::Block = game.world.get_block(&blocks, block_world_x as u32, block_pos.y, block_world_z as u32);

                    if block.id > 0 {
                        translation_matrix[(0, 3)] = block_world_x as f32;
                        translation_matrix[(1, 3)] = block_pos.y as f32;
                        translation_matrix[(2, 3)] = block_world_z as f32;

                        let transform_matrix: [[f32; 4]; 4] = translation_matrix.into();
                        target.draw(vertex_buffer, instance_buffer.per_instance().unwrap(), index_buffer, &program, &uniform! { sampler: *samplers.get(block.id as usize).unwrap(), transform: transform_matrix, view_matrix: view_matrix, projection_matrix: projection_matrix }, params).unwrap();
                    }
                }
            }
        }*/
        target.draw((vertex_buffer, instance_buffer.per_instance().unwrap()), index_buffer, &program, &uniform! { sampler: sampler, view_matrix: view_matrix, projection_matrix: projection_matrix }, params).unwrap();
        target.finish().unwrap();

        let ms = start.elapsed().as_secs() * 1000;
        if ms < 10 {
            thread::sleep(Duration::from_millis(10 - ms));
        }

        if game_input.get_key(VirtualKeyCode::W) {
            camera.translate(utils::get_forward_vector() / 32.0);
        } else if game_input.get_key(VirtualKeyCode::S) {
            camera.translate(-utils::get_forward_vector() / 32.0);
        }

        if game_input.get_key(VirtualKeyCode::A) {
            camera.translate(-utils::get_right_vector() / 32.0);
        } else if game_input.get_key(VirtualKeyCode::D) {
            camera.translate(utils::get_right_vector() / 32.0);
        }

        if game_input.get_key(VirtualKeyCode::Space) {
            camera.translate(utils::get_up_vector() / 32.0);
        } else if game_input.get_key(VirtualKeyCode::LControl) {
            camera.translate(-utils::get_up_vector() / 32.0);
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
                	_ => ()
                },
                _ => ()
            }
        });
    }
}