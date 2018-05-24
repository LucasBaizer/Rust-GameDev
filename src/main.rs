#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate alga;
extern crate noise;
extern crate rand;
extern crate bytebuffer;

mod camera;
mod object;
mod utils;
mod game;
mod input;
mod net;
mod quaternion;

#[derive(Clone, Copy)]
pub struct Instance {
    pub matrix: [[f32; 4]; 4],
    pub id: u8
}

use rand::Rng;
fn main() {
    use glium::{glutin, Surface};
    use glium::glutin::{VirtualKeyCode, MouseButton};
    use std::time::Instant;
    use std::time::Duration;
    use std::thread;
    use noise::{Perlin, NoiseModule, Seedable};
    use net::NetClient;
    use game::ItemStack;
    use game::Player;

    let mut player = Player::new();

    let mut camera: camera::Camera = camera::Camera::new(90);
    let mut game_input: input::Input = input::Input::new();

    /*thread::spawn(|| {
        let net_client = NetClient::new();

    });*/

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_title("Rust Minecraft");
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let mut display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();
    let window_size = display.get_framebuffer_dimensions();
    let perlin = Perlin::new();
    //perlin.set_seed(rand::thread_rng().gen::<usize>());
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
    let empty_params = &glium::DrawParameters {
        blend: glium::Blend {
            color: glium::BlendingFunction::Addition {
                source: glium::LinearBlendingFactor::SourceAlpha,
                destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
            },
            alpha: glium::BlendingFunction::Addition {
                source: glium::LinearBlendingFactor::One,
                destination: glium::LinearBlendingFactor::Zero
            },
            ..Default::default()
        },
        .. Default::default()
    };
    let wireframe = &glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            .. Default::default()
        },
        polygon_mode: glium::draw_parameters::PolygonMode::Line,
        .. Default::default()
    };

    let skybox_params = &glium::DrawParameters {
        .. Default::default()
    };

    let mut blocks: game::Blocks = game::Blocks::new();
    blocks.initialize();

    let blocks_count: f32 = blocks.block_map.len() as f32 - 1.0;

    let mut game: game::Game = game::Game::new(0, 16);
    for x in 0..255 {
        for z in 0..255 {
            let height: u8 = 64 + ((perlin.get([x as f32 / 10.0 + 0.5, z as f32 / 10.0 + 0.5])) * 4.0) as u8;
            for y in 0..height {
                let mut block = 1;
                if height - y <= 4 {
                    block = 3;
                }
                if height - 1 == y {
                    block = 4;
                }
                game.world.set_block(x, y, z, blocks.get_block(block));
            }
        }
    }

    let screen_size = display.get_framebuffer_dimensions();

    let mut closed = false;

    use game::Vertex;
	implement_vertex!(Vertex, position, uv, face);
    implement_vertex!(Instance, matrix, id);
    implement_vertex!(Vertex2D, position, uv);

	let vertex_shader_src = utils::file_to_string("shaders/vertex.glsl");
	let fragment_shader_src = utils::file_to_string("shaders/fragment.glsl");
	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let wireframe_vertex_shader_src = utils::file_to_string("shaders/wireframe.glsl");
	let wireframe_fragment_shader_src = utils::file_to_string("shaders/wireframe_fragment.glsl");
	let wireframe_program = glium::Program::from_source(&display, &wireframe_vertex_shader_src, &wireframe_fragment_shader_src, None).unwrap();

    let projection_matrix: [[f32; 4]; 4] = camera.create_projection_matrix(screen_size).into();
    let vertex_buffer = &game::Block::get_vertex_buffer(&mut display);
    let mut instance_buffer = game.world.get_instance_buffer(&mut display);
    let index_buffer = &game::Block::get_index_buffer(&mut display);
    //let sampler_raw = glium::texture::Texture2d::with_mipmaps(&mut display, utils::load_image_from_file("textures/blocks/atlas_old.png"), glium::texture::MipmapsOption::NoMipmap).unwrap();
    let sampler_raw = glium::texture::Texture2d::new(&mut display, utils::load_image_from_file("textures/blocks/atlas_old.png")).unwrap();
    let sampler = sampler_raw.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

    let crosshair_raw = glium::texture::Texture2d::new(&mut display, utils::load_image_from_file("textures/crosshair.png")).unwrap();
    let crosshair = crosshair_raw.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
    let hotbar_raw = glium::texture::Texture2d::new(&mut display, utils::load_image_from_file("textures/hotbar.png")).unwrap();
    let hotbar = hotbar_raw.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
    let hotbar_selected_raw = glium::texture::Texture2d::new(&mut display, utils::load_image_from_file("textures/hotbar_selected.png")).unwrap();
    let hotbar_selected = hotbar_selected_raw.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
    
    let text_vertex_shader_src = utils::file_to_string("shaders/text_vertex.glsl");
	let text_fragment_shader_src = utils::file_to_string("shaders/text_fragment.glsl");
    let text_program = glium::Program::from_source(&display, &text_vertex_shader_src, &text_fragment_shader_src, None).unwrap();
    let text_image_raw = glium::texture::Texture2d::new(&mut display, utils::load_image_from_file("textures/numbers.png")).unwrap();
    let text_image = text_image_raw.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
    let text_vertex = &glium::VertexBuffer::new(&mut display, &vec![ Vertex2D::new([0.0, 0.0], [0.0, 0.0]), Vertex2D::new([0.0, 1.0], [0.0, 1.0]), Vertex2D::new([1.0, 0.0], [1.0, 0.0]), Vertex2D::new([1.0, 1.0], [1.0, 1.0]) ]).unwrap();
    let text_indices = &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let hotbar_buffer = &glium::VertexBuffer::new(&mut display, &vec![ Vertex2D::new([0.0, 0.0], [0.0, 0.0]), Vertex2D::new([0.0, 1.0], [0.0, 1.0]), Vertex2D::new([1.0, 0.0], [1.0, 0.0]), Vertex2D::new([1.0, 1.0], [1.0, 1.0]) ]).unwrap();
    let hotbar_indices = &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let hotbar_selected_buffer = &glium::VertexBuffer::new(&mut display, &vec![ Vertex2D::new([0.0, 0.0], [0.0, 0.0]), Vertex2D::new([0.0, 1.0], [0.0, 1.0]), Vertex2D::new([1.0, 0.0], [1.0, 0.0]), Vertex2D::new([1.0, 1.0], [1.0, 1.0]) ]).unwrap();
    let hotbar_selected_indices = &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let flat_vertex_shader_src = utils::file_to_string("shaders/2d_vertex.glsl");
	let flat_fragment_shader_src = utils::file_to_string("shaders/2d_fragment.glsl");
    let flat_program = glium::Program::from_source(&display, &flat_vertex_shader_src, &flat_fragment_shader_src, None).unwrap();
    let crosshair_buffer = &glium::VertexBuffer::new(&mut display, &vec![ Vertex2D::new([0.0, 0.0], [0.0, 0.0]), Vertex2D::new([0.0, 1.0], [0.0, 1.0]), Vertex2D::new([1.0, 0.0], [1.0, 0.0]), Vertex2D::new([1.0, 1.0], [1.0, 1.0]) ]).unwrap();
    let crosshair_indices = &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let skybox_tex_raw = glium::texture::Texture2d::new(&mut display, utils::load_image_from_file("textures/skybox.png")).unwrap();
    let skybox_vertex_shader_src = utils::file_to_string("shaders/skybox_vertex.glsl");
	let skybox_fragment_shader_src = utils::file_to_string("shaders/skybox_fragment.glsl");
    let skybox_program = glium::Program::from_source(&display, &skybox_vertex_shader_src, &skybox_fragment_shader_src, None).unwrap();
    let skybox = texture_to_cubemap(skybox_tex_raw, &mut display);
    let skybox_sampled = skybox.sampled().wrap_function(glium::uniforms::SamplerWrapFunction::Clamp);

    // let wireframe_indices = &glium::IndexBuffer::new(&mut display, glium::index::PrimitiveType::LinesList, &[0, 1, 0, 2, 0, 3]);

    let orthographic_matrix: [[f32; 4]; 4] = (*nalgebra::Orthographic3::new(0.0, window_size.0 as f32, 0.0, window_size.1 as f32, 0.1, 100.0).as_matrix()).into();
    let crosshair_matrix: [[f32; 4]; 4] = [
        [36.0, 0.0, 0.0, 0.0],
        [0.0, 36.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [window_size.0 as f32 / 2.0 - 18.0, window_size.1 as f32 / 2.0 - 18.0, 0.0, 1.0]
    ];
    let hotbar_matrix: [[f32; 4]; 4] = [
        [182.0 * 2.5, 0.0, 0.0, 0.0],
        [0.0, 22.0 * 2.5, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [window_size.0 as f32 / 2.0 - (91.0 * 2.5), 33.0, 0.0, 1.0]
    ];

    camera.translate(utils::get_up_vector() * 72.0);
    camera.translate(utils::get_right_vector() * 4.0);
    camera.translate(utils::get_forward_vector() * 4.0);

    let mut gravity_velocity: f32 = 0.0;
    let mut dx: f32 = 0.0;
    let mut dy: f32 = 0.0;
    let mut grounded = false;
    while !closed {
        let start = Instant::now();

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        let view_matrix: [[f32; 4]; 4] = camera.get_view_matrix().try_inverse().unwrap().into();

        let mut skybox_view_matrix = view_matrix.clone();
        skybox_view_matrix[3][0] = 0.0;
        skybox_view_matrix[3][1] = 0.0;
        skybox_view_matrix[3][2] = 0.0;

        {
            target.draw(vertex_buffer, index_buffer, &skybox_program, &uniform! { view_matrix: skybox_view_matrix, projection_matrix: projection_matrix, cubemap: skybox_sampled }, skybox_params).unwrap();
            target.draw((vertex_buffer, instance_buffer.per_instance().unwrap()), index_buffer, &program, &uniform! { sampler: sampler, view_matrix: view_matrix, projection_matrix: projection_matrix, total_blocks: blocks_count }, params).unwrap();
        }

        let target_tuple = camera.get_targeted_block(&game);
        match target_tuple.0 {
            Some(pos) => {
                if game_input.get_button_down(MouseButton::Left) {
                    player.push_item(ItemStack::new(game.world.get_block(&blocks, pos.x, pos.y, pos.z).drop_id, 1, 64));
                    game.world.set_block(pos.x, pos.y, pos.z, blocks.get_block(0));
                    instance_buffer = game.world.get_instance_buffer(&mut display);
                }
                if game_input.get_button_down(MouseButton::Right) {
                    match target_tuple.1 {
                        Some(new) => {
                            let index = player.selected_index;
                            let mut slctd = &mut player.get_hotbar()[index as usize];
                            if !slctd.is_empty() {
                                let cam = camera.position;
                                let round = nalgebra::Vector3::<u32>::new(f32::round(cam[0]) as u32, f32::round(cam[1]) as u32, f32::round(cam[2]) as u32);

                                if round[0] != new.x || round[1] as u8 != new.y || round[2] != new.z {
                                    game.world.set_block(new.x, new.y, new.z, blocks.get_block(slctd.id));
                                    instance_buffer = game.world.get_instance_buffer(&mut display);
                                }

                                slctd.count -= 1;
                            }
                        },
                        None => ()
                    }
                }
                target.draw(vertex_buffer, index_buffer, &wireframe_program, &uniform! { view_matrix: view_matrix, projection_matrix: projection_matrix, cube_position: pos.to_array() }, wireframe).unwrap();
            },
            None => ()
        }
        target.draw(crosshair_buffer, crosshair_indices, &flat_program, &uniform! { transform_matrix: crosshair_matrix, projection_matrix: orthographic_matrix, sampler: crosshair }, &empty_params).unwrap();
        target.draw(hotbar_buffer, hotbar_indices, &flat_program, &uniform! { transform_matrix: hotbar_matrix, projection_matrix: orthographic_matrix, sampler: hotbar }, &empty_params).unwrap();

        let mut hotbar_selected_matrix = utils::get_identity_matrix();

        let w = 24.0 * 2.5;

        hotbar_selected_matrix[(0, 0)] = w;
        hotbar_selected_matrix[(1, 1)] = w;
        hotbar_selected_matrix[(2, 2)] = w;

        hotbar_selected_matrix[(0, 3)] = (window_size.0 as f32) / 2.0 - (91.0 * 2.5) + (player.selected_index as f32 * (20.0 * 2.5)) - 1.0;
        hotbar_selected_matrix[(1, 3)] = 32.0;
        hotbar_selected_matrix[(2, 3)] = -1.0;

        let into: [[f32; 4]; 4] = hotbar_selected_matrix.into();
        target.draw(hotbar_selected_buffer, hotbar_selected_indices, &flat_program, &uniform! { transform_matrix: into, projection_matrix: orthographic_matrix, sampler: hotbar_selected }, &empty_params).unwrap();

        for i in 0..9 {
            let hb = player.get_hotbar();
            if !hb[i].is_empty() {
                let mut transformation = utils::get_identity_matrix();
                let rot: nalgebra::Matrix4<f32> = nalgebra::Matrix4::from_euler_angles(f32::to_radians(-30.0), 0.0, 0.0) * nalgebra::Matrix4::from_euler_angles(0.0, f32::to_radians(-45.0), 0.0);

                let w = 22.0 * 2.5 / 2.0;

                transformation[(0, 0)] = w * 0.8;
                transformation[(1, 1)] = w * 0.8;
                transformation[(2, 2)] = w * 0.8;

                transformation[(0, 3)] = window_size.0 as f32 / 2.0 - (91.0 * 2.5) + (i as f32 * (20.0 * 2.5)) + ((20.0 * 2.5) / 2.0) + 3.0;
                transformation[(1, 3)] = 33.0 + ((22.0 * 2.5) / 2.0) - 1.0;
                transformation[(2, 3)] = -33.0;

                let rotated = transformation * rot;

                let instance = glium::VertexBuffer::new(&mut display, 
                    &vec![
                        Instance {
            			    matrix: rotated.into(),
            		    	id: hb[i].id
                        }
                    ]).unwrap();
                let into: [[f32; 4]; 4] = utils::get_identity_matrix().into();
                target.draw((vertex_buffer, instance.per_instance().unwrap()), index_buffer, &program, &uniform! { sampler: sampler, projection_matrix: orthographic_matrix, view_matrix: into, total_blocks: blocks_count }, params ).unwrap();
                
                let txt = hb[i].count.to_string();
                let mut transformation = utils::get_identity_matrix();
                transformation[(0, 0)] = w * 0.5;
                transformation[(1, 1)] = w * 0.5;
                transformation[(2, 2)] = w * 0.5;

                transformation[(1, 3)] = 33.0 + (w * 0.3);
                transformation[(2, 3)] = -1.0;

                let mut idx = txt.len() as f32 - 1.0;
                for ch in txt.bytes() {
                    transformation[(0, 3)] = (window_size.0 as f32 / 2.0 - (91.0 * 2.5) + (i as f32 * (20.0 * 2.5)) + ((20.0 * 2.5) / 2.0) + 10.0) - idx * 10.0;
                    let transform_into: [[f32; 4]; 4] = transformation.into();

                    target.draw(text_vertex, text_indices, &text_program, &uniform! { transform_matrix: transform_into, projection_matrix: orthographic_matrix, sampler: text_image, character: (ch - b'0') as i32 }, &empty_params).unwrap();
                    idx -= 1.0;
                }
            }
        }

        target.finish().unwrap();

        let ms = start.elapsed().as_secs() * 1000;
        if ms < 10 {
            thread::sleep(Duration::from_millis(10 - ms));
        }

        let old_pos = camera.position;
        let corner_positions = get_player_bounds(camera.position);

        camera.translate(-utils::get_up_vector() * gravity_velocity);
        gravity_velocity += 0.0055;

        let number_keys = [VirtualKeyCode::Key1, VirtualKeyCode::Key2, VirtualKeyCode::Key3, VirtualKeyCode::Key4, VirtualKeyCode::Key5, VirtualKeyCode::Key6, VirtualKeyCode::Key7, VirtualKeyCode::Key8, VirtualKeyCode::Key9];
        for i in 0..9 {
            if game_input.get_key(number_keys[i]) {
                player.selected_index = i as u8;
                break;
            }
        }

        let speed = 0.05;
        if game_input.get_key(VirtualKeyCode::W) {
            let f = camera.forward_2d(speed);
            if game_input.get_key(VirtualKeyCode::LShift) {
                camera.translate(f * 1.5);
            } else {
                camera.translate(f);
            }
        } else if game_input.get_key(VirtualKeyCode::S) {
            let b = camera.forward_2d(-speed);
            camera.translate(b);
        }

        if game_input.get_key(VirtualKeyCode::A) {
            let r = -camera.left_2d(speed);
            camera.translate(r);
        } else if game_input.get_key(VirtualKeyCode::D) {
            let l = camera.left_2d(speed);
            camera.translate(l);
        }

        if game_input.get_key(VirtualKeyCode::Space) {
            if grounded {
                gravity_velocity = -0.12;
            }
        }

        if game_input.get_key(VirtualKeyCode::Escape) {
            return;
        }

        if game.world.is_in_rendered_world_bounds(game.render_distance, old_pos.x as i64, old_pos.y as i16, old_pos.z as i64) {
            let mut v = camera.position - old_pos;
            let mut bottom_hit = false;
            for i in 0..8 {
                let o = &mut corner_positions[i].clone();
                let mut n = &mut (corner_positions[i] + v);

                let result = constrain_camera(&game.world, o, &mut n, gravity_velocity);
                gravity_velocity = result.0;
                if i < 4 {
                    bottom_hit = bottom_hit || result.1;
                }
            
                v = *n - *o;
            }
            grounded = bottom_hit;
            camera.position = old_pos + v;
        }

        game_input.set_button_down(MouseButton::Left, false);
        game_input.set_button_down(MouseButton::Right, false);

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
                    glutin::WindowEvent::MouseInput { button, state, .. } => match state {
                        glutin::ElementState::Pressed => {
                            game_input.set_button(button, true);
                            game_input.set_button_down(button, true);
                        },
                        glutin::ElementState::Released => {
                            game_input.set_button(button, false);
                        }
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        display.gl_window().window().set_cursor_position(screen_size.0 as i32 / 2, screen_size.1 as i32 / 2).unwrap();
    }
}

fn get_player_bounds(pos: nalgebra::Vector3<f32>) -> [nalgebra::Vector3<f32>; 8] {
    [
        nalgebra::Vector3::new(pos.x - 0.4, pos.y - 1.5, pos.z - 0.4),
        nalgebra::Vector3::new(pos.x - 0.4, pos.y - 1.5, pos.z + 0.4),
        nalgebra::Vector3::new(pos.x + 0.4, pos.y - 1.5, pos.z - 0.4),
        nalgebra::Vector3::new(pos.x + 0.4, pos.y - 1.5, pos.z + 0.4),
        nalgebra::Vector3::new(pos.x - 0.4, pos.y, pos.z - 0.4),
        nalgebra::Vector3::new(pos.x - 0.4, pos.y, pos.z + 0.4),
        nalgebra::Vector3::new(pos.x + 0.4, pos.y, pos.z - 0.4),
        nalgebra::Vector3::new(pos.x + 0.4, pos.y, pos.z + 0.4)
    ]
}

fn constrain_camera(world: &game::World, old_pos: &mut nalgebra::Vector3<f32>, new_pos: &mut nalgebra::Vector3<f32>, gravity_velocity: f32) -> (f32, bool) {
    let test_x = (new_pos.x, old_pos.y, old_pos.z);
    if world.is_solid_block(test_x.0, test_x.1, test_x.2) {
        new_pos.x = old_pos.x;
    }
    let test_z = (new_pos.x, old_pos.y, new_pos.z);
    if world.is_solid_block(test_z.0, test_z.1, test_z.2) {
        new_pos.z = old_pos.z;
    }
    let test_y = (new_pos.x, new_pos.y, new_pos.z);
    if world.is_solid_block(test_y.0, test_y.1, test_y.2) {
        new_pos.y = old_pos.y;
        return (0.0, true);
    }

    (gravity_velocity, false)
}

use glium::texture::cubemap::Cubemap;
use glium::texture::cubemap::CubemapMipmap;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::CubeLayer;
use glium::Rect;
use glium::BlitTarget;
use glium::uniforms::MagnifySamplerFilter;
use glium::Surface;
use glium::texture::Texture2d;
fn texture_to_cubemap(tex: Texture2d, display: &mut glium::Display) -> Cubemap {
    let map = Cubemap::empty(display, 160).unwrap();
    {
        let main_level = map.main_level();
        let to_rect = BlitTarget {
            left: 0,
            bottom: 0,
            width: 160,
            height: 160
        };
        let reverse_rect = BlitTarget {
            left: 160,
            bottom: 160,
            width: -160,
            height: -160
        };

        fn write_to_cubemap(display: &mut glium::Display, main_level: CubemapMipmap, side: CubeLayer, to_rect: BlitTarget, tex: &Texture2d, left: u32, right: u32) {
            let buffer = SimpleFrameBuffer::new(display, main_level.image(side)).unwrap();

            tex.as_surface().blit_color(&Rect {
                left: left,
                bottom: right,
                width: 160,
                height: 160
           }, &buffer, &to_rect, MagnifySamplerFilter::Nearest);
        }

        let size = 160;
        write_to_cubemap(display, main_level, CubeLayer::NegativeX, reverse_rect, &tex, 0, size);
        write_to_cubemap(display, main_level, CubeLayer::NegativeY, to_rect, &tex, size, 0);
        write_to_cubemap(display, main_level, CubeLayer::NegativeZ, reverse_rect, &tex, size, size);
        write_to_cubemap(display, main_level, CubeLayer::PositiveX, reverse_rect, &tex, size * 2, size);
        write_to_cubemap(display, main_level, CubeLayer::PositiveY, to_rect, &tex, size, size * 2);
        write_to_cubemap(display, main_level, CubeLayer::PositiveZ, reverse_rect, &tex, size * 3, size);
    }

    map
}

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub uv: [f32; 2]
}

impl Vertex2D {
    pub fn new(position: [f32; 2], uv: [f32; 2]) -> Vertex2D {
        Vertex2D {
            position: position,
            uv: uv
        }
    }
}