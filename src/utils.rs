use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use nalgebra;
use glium;
use image;

pub fn file_to_string(file: &str) -> String {
	let mut read = String::new();
	let mut file = File::open(file).expect("file not found");
    file.read_to_string(&mut read).expect("something went wrong reading the file");

    read
}

pub fn file_to_bytes(file: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(file).expect("file not found");
    file.read_to_end(&mut bytes).expect("something went wrong reading the file");

    bytes
}

pub fn load_image_from_file<'a>(file: &str) -> glium::texture::RawImage2d<'a, u8> {
	let image = image::load(Cursor::new(&file_to_bytes(file)), image::PNG).unwrap().to_rgba();
	let dimensions = image.dimensions();

	glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw()[..], dimensions)
}

pub fn get_identity_matrix() -> nalgebra::Matrix4<f32> {
	nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)
}

pub fn get_forward_vector() -> nalgebra::Vector3<f32> {
	nalgebra::Vector3::new(0.0, 0.0, 1.0)
}

pub fn get_up_vector() -> nalgebra::Vector3<f32> {
	nalgebra::Vector3::new(0.0, 1.0, 0.0)
}

pub fn get_right_vector() -> nalgebra::Vector3<f32> {
	nalgebra::Vector3::new(1.0, 0.0, 0.0)
}

pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
	f32::max(min, f32::min(max, val))
}

/*pub fn get_one_vector() -> nalgebra::Vector3<f32> {
	nalgebra::Vector3::new(1.0, 1.0, 1.0)
}*/