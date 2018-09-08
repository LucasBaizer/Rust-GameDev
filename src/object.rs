use game::Block;
use glium::uniforms::MagnifySamplerFilter;
use glium::Frame;
use glium::Surface;
use graphics::*;

pub struct Skybox;

impl GameObject for Skybox {
    fn draw(&self, graphics: &mut Graphics, params: &GraphicsParams) {
        let mut skybox_view_matrix = params.view_matrix.clone();
        skybox_view_matrix[3][0] = 0.0;
        skybox_view_matrix[3][1] = 0.0;
        skybox_view_matrix[3][2] = 0.0;

        let vertex_buffer = &Block::get_vertex_buffer(&mut graphics.display);
        let index_buffer = &Block::get_index_buffer(&mut graphics.display);

        let frame = graphics.get_frame();

        let cubemap = graphics
            .get_image("skybox")
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest);
        let draw_params = graphics.get_draw_params("skybox");
        let shader = graphics.get_shader("skybox");

        frame
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
    }
}
