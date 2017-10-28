/*use nalgebra;
use obj;

pub struct GameObject {
	object: obj::Obj,
	position: nalgebra::Vector3<f32>,
	rotation: nalgebra::Vector3<f32>,
	scale: nalgebra::Vector3<f32>
}

use game::Vertex;
use utils;
use glium;

impl GameObject {
	pub fn load_from_obj(file: &str) -> GameObject {
		/*let mut vertices: Vec<Vertex> = Vec::new();
		let obj: String = utils::file_to_string(file);
		let split = obj.split("\n");
		for line in split {
			if line.starts_with("v ") {
				let vertex_str: String = line.chars().skip(2).collect();
				let split = vertex_str.split(" ");

				let mut vertex = Vertex::new();
				let mut i = 0;
				for value in split {
					vertex.position[i] = value.parse::<f32>().unwrap();
					i += 1;
				}

				vertices.push(vertex);
			} else {
				panic!();
			}
		}*/

		use std::io::BufReader;
		use std::fs::File;
		let input = BufReader::new(File::open(file).expect("file not found"));
		let obj: obj::Obj = obj::load_obj(input).unwrap();

		GameObject { object: obj, position: nalgebra::Vector3::new(0.0, 0.0, 0.0), rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0), scale: nalgebra::Vector3::new(1.0, 1.0, 1.0) }
	}

	pub fn get_vertex_buffer(&self, display: &mut glium::Display) -> glium::VertexBuffer<Vertex> {
		let mut verts: Vec<Vertex> = Vec::new();

		for fake_vert in &self.object.vertices {
			verts.push(Vertex {
				position: fake_vert.position,
				uv: [0.0; 2]
			});
		}

		glium::VertexBuffer::new(display, &verts).unwrap()
	}

	pub fn get_index_buffer(&self, display: &mut glium::Display) -> glium::IndexBuffer<u16> {
		glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.object.indices).unwrap()
	}

	pub fn get_transform_matrix(&self) -> nalgebra::Matrix4<f32> {
		let mut translation_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();
        let mut rotation_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();
        let mut scale_matrix: nalgebra::Matrix4<f32> = utils::get_identity_matrix();

        translation_matrix[(0, 3)] = self.position[0];
        translation_matrix[(1, 3)] = self.position[1];
        translation_matrix[(2, 3)] = self.position[2];
        rotation_matrix[(0, 0)] = f32::cos(f32::to_radians(self.rotation[2]));
        rotation_matrix[(2, 0)] = f32::sin(f32::to_radians(self.rotation[2]));
        rotation_matrix[(0, 2)] = -f32::sin(f32::to_radians(self.rotation[2]));
        rotation_matrix[(2, 2)] = f32::cos(f32::to_radians(self.rotation[2]));
        scale_matrix[(0, 0)] = self.scale[0];
        scale_matrix[(1, 1)] = self.scale[1];
        scale_matrix[(2, 2)] = self.scale[2];

        translation_matrix * rotation_matrix * scale_matrix
	}

	pub fn translate(&mut self, translation: nalgebra::Vector3<f32>) {
		self.position += translation;
	}

	pub fn rotate(&mut self, rotation: nalgebra::Vector3<f32>) {
		self.rotation += rotation;
	}

	pub fn scale(&mut self, increase: nalgebra::Vector3<f32>) {
		self.scale += increase;
	}
}*/