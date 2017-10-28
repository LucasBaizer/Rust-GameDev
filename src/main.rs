#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate obj;

mod camera;
mod object;
mod utils;
mod game;

fn main() {
    use glium::{glutin, Surface};

    let camera: camera::Camera = camera::Camera::new(60);

    //let mut game_object: object::GameObject = object::GameObject::load_from_obj("models/cube.obj");

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_title("Rust Minecraft");
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);;
    let mut display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    use game;
    let mut blocks: game::Blocks = game::Blocks::new();
    blocks.initialize(&mut display);

    let mut game: game::Game = game::Game::new(0);

    game.world.set_block(0, 0, 0, blocks.get_block(0));

    let screen_size = display.get_framebuffer_dimensions();

    let mut closed = false;

    use game::Vertex;
	implement_vertex!(Vertex, position, uv);

	let vertex_shader_src = utils::file_to_string("shaders/vertex.glsl");
	let fragment_shader_src = utils::file_to_string("shaders/fragment.glsl");

	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
    let mut block_angles: f32 = 0.0;
    let projection_matrix: nalgebra::Matrix4<f32> = camera.create_projection_matrix(screen_size);
    while !closed {
        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        let mut translation_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();
        let mut rotation_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();

        translation_matrix[(0, 3)] = 0.0;
        translation_matrix[(1, 3)] = 0.0;
        translation_matrix[(2, 3)] = 5.0;
        rotation_matrix[(0, 0)] = f32::cos(f32::to_radians(block_angles));
        rotation_matrix[(2, 0)] = f32::sin(f32::to_radians(block_angles));
        rotation_matrix[(0, 2)] = -f32::sin(f32::to_radians(block_angles));
        rotation_matrix[(2, 2)] = f32::cos(f32::to_radians(block_angles));
        block_angles = block_angles + 0.05;

        let block: &game::Block = game.world.get_block(&blocks, 0, 0, 0);
        let transform_matrix: [[f32; 4]; 4] = /*game_object.get_transform_matrix().into();*/ (translation_matrix * rotation_matrix * utils::get_identity_matrix()).into();
        let texture_2d = &block.texture;
        let projection_matrix: [[f32; 4]; 4] = projection_matrix.into();
        target.draw(&block.get_vertex_buffer(&mut display), &block.get_index_buffer(&mut display), &program, &uniform! { sampler: texture_2d.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest), transform: transform_matrix, projection_matrix: projection_matrix },
            &params).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                	glutin::WindowEvent::Closed => closed = true,
                	/*glutin::WindowEvent::KeyboardInput { input, .. } => match input.state {
                        glutin::ElementState::Pressed => match input.virtual_keycode {                 
                		    Some(glutin::VirtualKeyCode::Escape) => closed = true,
                		    Some(glutin::VirtualKeyCode::Left) => game_object.translate(-utils::get_right_vector()),
                		    Some(glutin::VirtualKeyCode::Right) => game_object.translate(utils::get_right_vector()),
                		    Some(glutin::VirtualKeyCode::Up) => game_object.translate(utils::get_up_vector()),
                		    Some(glutin::VirtualKeyCode::Down) => game_object.translate(-utils::get_up_vector()),
                            Some(glutin::VirtualKeyCode::Space) => game_object.scale(utils::get_one_vector() / 10.0),
                            Some(glutin::VirtualKeyCode::LControl) => game_object.scale(-utils::get_one_vector() / 10.0),
                            Some(glutin::VirtualKeyCode::A) => game_object.rotate(utils::get_forward_vector()),
                            Some(glutin::VirtualKeyCode::D) => game_object.rotate(-utils::get_forward_vector()),
                            Some(glutin::VirtualKeyCode::LShift) => game_object.translate(-utils::get_forward_vector()),
                            Some(glutin::VirtualKeyCode::RShift) => game_object.translate(utils::get_forward_vector()),
                		    _ => ()
                        },
                        _ => ()
                	},*/
                	_ => ()
                },
                _ => ()
            }
        });
    }
}