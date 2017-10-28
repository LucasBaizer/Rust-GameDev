#[macro_use]
extern crate glium;
#[macro_use]
extern crate lazy_static;
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
    let context = glium::glutin::ContextBuilder::new();
    let mut display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();

    use game;
    let blocks: &'static game::Blocks = &game::Blocks::new();
    blocks.initialize(&mut display);

    let mut game: game::Game = game::Game::new(0);

    game.world.set_block(0, 0, 0, blocks.block_map.get(&0).unwrap());

    let screen_size = display.get_framebuffer_dimensions();

    let mut closed = false;

    /*(let bytes = utils::file_to_bytes("rust_logo.jpg");

	let image = image::load(Cursor::new(&bytes), image::JPEG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw()[..], image_dimensions);
	let texture = glium::texture::Texture2d::new(&display, image).unwrap();*/

    use game::Vertex;
	implement_vertex!(Vertex, position, uv);

	//let vertex_buffer = game_object.get_vertex_buffer(&mut display);
	//let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
    //let indices = game_object.get_index_buffer(&mut display);

	let vertex_shader_src = utils::file_to_string("shaders/vertex.glsl");
	let fragment_shader_src = utils::file_to_string("shaders/fragment.glsl");

	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let projection_matrix: nalgebra::Matrix4<f32> = camera.create_projection_matrix(screen_size);
    while !closed {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let mut translation_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();

        translation_matrix[(0, 3)] = 0.0;
        translation_matrix[(1, 3)] = 0.0;
        translation_matrix[(2, 3)] = 0.0;

        let block: &game::Block = game.world.get_block(&blocks, 0, 0, 0);
        let transform_matrix: [[f32; 4]; 4] = /*game_object.get_transform_matrix().into();*/ (translation_matrix * utils::get_identity_matrix() * utils::get_identity_matrix()).into();
        let texture_2d = &block.texture;
        let projection_matrix: [[f32; 4]; 4] = projection_matrix.into();
        target.draw(&block.get_vertex_buffer(&mut display), &block.get_index_buffer(&mut display), &program, &uniform! { sampler: texture_2d, transform: transform_matrix, projection_matrix: projection_matrix },
            &Default::default()).unwrap();
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