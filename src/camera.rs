use nalgebra;

pub struct Camera {
	field_of_view: f32,
}

const NEAR_PLANE: f32 = 0.001;
const FAR_PLANE : f32 = 1000.0;

impl Camera {
	pub fn new(fov: u32) -> Camera {
		Camera {
			field_of_view: fov as f32
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
}