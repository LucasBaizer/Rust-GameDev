use nalgebra;

pub struct Camera {
	pub field_of_view: f32,
	pub position: nalgebra::Vector3<f32>,
	pub rot_x: f32,
	pub rot_y: f32
}

const NEAR_PLANE: f32 = 0.001;
const FAR_PLANE : f32 = 1000.0;

use utils;
use alga::linear::Transformation;
use game;

impl Camera {
	pub fn new(fov: u32) -> Camera {
		Camera {
			field_of_view: fov as f32,
			position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
			rot_x: 0.0,
			rot_y: 0.0
		}
	}

	pub fn create_projection_matrix(&self, screen_size: (u32, u32)) -> nalgebra::Matrix4<f32> {
	    let aspect_ratio: f32 = screen_size.0 as f32 / screen_size.1 as f32;
	    let y_scale = (1.0 / f32::tan(f32::to_radians(self.field_of_view / 2.0))) * aspect_ratio;
	    let x_scale = y_scale / aspect_ratio;
	    let frustum_length = FAR_PLANE - NEAR_PLANE;

	    let mut matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
	    matrix[(0, 0)] = x_scale;
	    matrix[(1, 1)] = y_scale;
	    matrix[(2, 2)] = (FAR_PLANE + NEAR_PLANE) / frustum_length;
	    matrix[(3, 2)] = 1.0;
	    matrix[(2, 3)] = -((2.0 * NEAR_PLANE * FAR_PLANE) / frustum_length);
	    matrix[(3, 3)] = 0.0;

	    matrix
	}

	pub fn get_view_matrix(&self) -> nalgebra::Matrix4<f32> {
		let mut translation_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();
        let rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::<f32>::from_euler_angles(-self.rot_y, -self.rot_x, 0.0);

        translation_matrix[(0, 3)] = self.position[0];
        translation_matrix[(1, 3)] = self.position[1];
        translation_matrix[(2, 3)] = self.position[2];

        translation_matrix * rotation_matrix
	}

	pub fn translate(&mut self, translation: nalgebra::Vector3<f32>) {
		self.position += translation;
	}

	pub fn forward(&self) -> nalgebra::Vector3<f32> {
		let mut point = nalgebra::Vector3::new(0.0, 0.0, 1.0);
		point = self.get_view_matrix().transform_vector(&point);

		point
	}

	pub fn forward_2d(&self, speed: f32) -> nalgebra::Vector3<f32> {
		use std::f32::consts::PI;
		nalgebra::Vector3::new(f32::cos(self.rot_x + PI / 2.0), 0.0, f32::sin(self.rot_x + PI / 2.0)) * speed
 	}

	pub fn right(&self) -> nalgebra::Vector3<f32> {
		let mut point = nalgebra::Vector3::new(1.0, 0.0, 0.0);
		point = self.get_view_matrix().transform_vector(&point);

		point
	}

	pub fn left_2d(&self, speed: f32) -> nalgebra::Vector3<f32> {
		nalgebra::Vector3::new(f32::cos(self.rot_x), 0.0, f32::sin(self.rot_x)) * speed
 	}

	pub fn get_targeted_block(&self, game: &game::Game) -> Option<game::BlockPos> {
		let mut jump = self.position;

		for _ in 0..10 {
			if game.world.is_in_rendered_world_bounds(game.render_distance, jump[0] as i64, jump[1] as i16, jump[2] as i64) {
				let id = game.world.get_block_id(jump[0] as u32, jump[1] as u8, jump[2] as u32);
				if id != 0 {
					return Some(game::BlockPos::new(jump[0] as u32, jump[1] as u8, jump[2] as u32, id));
				}
			}

			jump += (self.forward() / 5.0);
		}

		None
	}
}