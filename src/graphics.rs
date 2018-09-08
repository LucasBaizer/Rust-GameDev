use glium::backend::glutin::*;
use glium::draw_parameters::*;
use glium::glutin::*;
use glium::index::*;
use glium::texture::*;
use glium::*;
use std::cell::RefCell;

use camera::Camera;
use game::Vertex;
use std::collections::HashMap;
use utils;

pub struct Graphics<'a> {
    pub display: Display,
    pub window_size: (f32, f32),
    pub shaders: HashMap<String, Program>,
    pub images: HashMap<String, Texture2d>,
    pub draw_params: HashMap<String, DrawParameters<'a>>,
    pub transforms_2d: HashMap<String, [[f32; 4]; 4]>,
    target: RefCell<Option<Frame>>,
}

impl<'a> Graphics<'a> {
    pub fn new() -> Graphics<'a> {
        implement_vertex!(Vertex, position, uv, face);
        implement_vertex!(Instance, matrix, id);
        implement_vertex!(Vertex2D, position, uv);

        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new().with_title("Rust Minecraft");
        let context = ContextBuilder::new().with_depth_buffer(24);
        let display = Display::new(window, context, &events_loop).unwrap();
        let window_size = display.get_framebuffer_dimensions();

        display
            .gl_window()
            .window()
            .set_cursor_state(CursorState::Hide)
            .unwrap();

        let mut graphics = Graphics {
            display: display,
            window_size: (window_size.0 as f32, window_size.1 as f32),
            shaders: HashMap::new(),
            images: HashMap::new(),
            draw_params: HashMap::new(),
            transforms_2d: HashMap::new(),
            target: RefCell::new(None),
        };

        graphics.load_standard_shaders();
        graphics.load_standard_images();
        graphics.load_standard_transforms_2d();
        graphics.load_standard_draw_params();

        graphics
    }

    pub fn load_standard_shaders(&mut self) {
        self.load_shader(
            "block",
            "shaders/block_vertex.glsl",
            "shaders/block_fragment.glsl",
        );
        self.load_shader(
            "wireframe",
            "shaders/wireframe_vertex.glsl",
            "shaders/wireframe_fragment.glsl",
        );
        self.load_shader(
            "text",
            "shaders/text_vertex.glsl",
            "shaders/text_fragment.glsl",
        );
        self.load_shader("flat", "shaders/2d_vertex.glsl", "shaders/2d_fragment.glsl");
        self.load_shader(
            "skybox",
            "shaders/skybox_vertex.glsl",
            "shaders/skybox_fragment.glsl",
        );
    }

    pub fn load_standard_images(&mut self) {
        self.load_image("atlas", "textures/blocks/atlas_old.png");
        self.load_image("crosshair", "textures/crosshair.png");
        self.load_image("hotbar", "textures/hotbar.png");
        self.load_image("hotbar_selected", "textures/hotbar_selected.png");
        self.load_image("text", "textures/numbers.png");
        self.load_image("skybox", "textures/skybox.png");
    }

    pub fn load_standard_transforms_2d(&mut self) {
        let size_x = self.window_size.0;
        let size_y = self.window_size.1;
        self.load_transform_2d(
            "crosshair",
            [
                [36.0, 0.0, 0.0, 0.0],
                [0.0, 36.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [size_x / 2.0 - 18.0, size_y / 2.0 - 18.0, 0.0, 1.0],
            ],
        );
        self.load_transform_2d(
            "hotbar",
            [
                [182.0 * 2.5, 0.0, 0.0, 0.0],
                [0.0, 22.0 * 2.5, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [size_x / 2.0 - (91.0 * 2.5), 33.0, 0.0, 1.0],
            ],
        );
    }

    pub fn load_standard_draw_params(&mut self) {
        self.load_draw_params(
            "block",
            DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullCounterClockwise,
                ..Default::default()
            },
        );
        self.load_draw_params(
            "empty",
            DrawParameters {
                blend: Blend {
                    color: BlendingFunction::Addition {
                        source: LinearBlendingFactor::SourceAlpha,
                        destination: LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    alpha: BlendingFunction::Addition {
                        source: LinearBlendingFactor::One,
                        destination: LinearBlendingFactor::Zero,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        self.load_draw_params(
            "wireframe",
            DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                polygon_mode: PolygonMode::Line,
                ..Default::default()
            },
        );
        self.load_draw_params(
            "skybox",
            DrawParameters {
                ..Default::default()
            },
        );
    }

    pub fn load_shader(&mut self, name: &str, vertex: &str, fragment: &str) {
        let vertex_shader_src = utils::file_to_string(vertex);
        let fragment_shader_src = utils::file_to_string(fragment);
        self.shaders.insert(
            String::from(name),
            Program::from_source(
                &self.display,
                &vertex_shader_src,
                &fragment_shader_src,
                None,
            ).unwrap(),
        );
    }

    pub fn get_shader(&self, name: &str) -> &Program {
        self.shaders.get(name).unwrap()
    }

    pub fn load_image(&mut self, name: &str, file: &str) {
        let sampler_raw =
            Texture2d::new(&mut self.display, utils::load_image_from_file(file)).unwrap();
        self.images.insert(String::from(name), sampler_raw);
    }

    pub fn get_image(&self, name: &str) -> &Texture2d {
        self.images.get(name).unwrap()
    }

    pub fn load_transform_2d(&mut self, name: &str, transform: [[f32; 4]; 4]) {
        self.transforms_2d.insert(String::from(name), transform);
    }

    pub fn load_draw_params(&mut self, name: &str, params: DrawParameters<'a>) {
        self.draw_params.insert(String::from(name), params);
    }

    pub fn get_draw_params(&self, name: &str) -> &DrawParameters<'a> {
        self.draw_params.get(name).unwrap()
    }

    pub fn new_2d_vertex_buffer(&mut self) -> VertexBuffer<Vertex2D> {
        VertexBuffer::new(
            &mut self.display,
            &vec![
                Vertex2D::new([0.0, 0.0], [0.0, 0.0]),
                Vertex2D::new([0.0, 1.0], [0.0, 1.0]),
                Vertex2D::new([1.0, 0.0], [1.0, 0.0]),
                Vertex2D::new([1.0, 1.0], [1.0, 1.0]),
            ],
        ).unwrap()
    }

    pub fn new_2d_index_buffer(&mut self) -> NoIndices {
        NoIndices(PrimitiveType::TriangleStrip)
    }

    /*pub fn draw(&mut self, vertex_buffer: &VertexBuffer<Vertex>, index_buffer:  name: &str) {
        self.frame.as_mut().
            .draw(
                vertex_buffer,
                index_buffer,
                shader,
                &uniform! {
                    view_matrix: skybox_view_matrix,
                    projection_matrix: params.projection_matrix,
                    cubemap: cubemap
                },
                draw_params,
            )
            .unwrap();
    }*/

    pub fn start(&mut self) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        self.target.replace(Some(target));
    }

    pub fn finish(&mut self) {
        self.target.replace(None).unwrap().finish().unwrap();
    }

    pub fn get_frame(&self) -> &mut Frame {
        self.target.borrow_mut().as_mut().unwrap()
    }
}

pub struct GraphicsParams {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
}

impl GraphicsParams {
    pub fn new(view_matrix: [[f32; 4]; 4], projection_matrix: [[f32; 4]; 4]) -> GraphicsParams {
        GraphicsParams {
            view_matrix,
            projection_matrix,
        }
    }
}

pub trait GameObject {
    fn draw(&self, graphics: &mut Graphics, params: &GraphicsParams);
}

#[derive(Clone, Copy)]
pub struct Instance {
    pub matrix: [[f32; 4]; 4],
    pub id: u8,
}

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

impl Vertex2D {
    pub fn new(position: [f32; 2], uv: [f32; 2]) -> Vertex2D {
        Vertex2D {
            position: position,
            uv: uv,
        }
    }
}
