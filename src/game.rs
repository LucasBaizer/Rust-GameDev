pub struct Game {
	pub world: World
}

impl Game {
	pub fn new(air_block: u8) -> Game {
		Game {
			world: create_world(air_block)
		}
	}
}

pub struct World {
	chunks: [[Chunk; 16]; 16]
}

impl World {
	pub fn get_block(&mut self, blocks: &'static Blocks, x: u32, y: u32, z: u32) -> &'static Block {
		blocks.block_map.get(&self.chunks[(x >> 4) as usize][(y >> 4) as usize].blocks[(x & 15) as usize][(z & 15) as usize][y as usize]).unwrap()
	}

	pub fn set_block(&mut self, x: u32, y: u32, z: u32, block: &Block) {
		self.chunks[(x >> 4) as usize][(z >> 4) as usize].blocks[(x & 15) as usize][(z & 15) as usize][y as usize] = block.id;
	}
}

pub struct Chunk {
	blocks: [[[u8; 255]; 16]; 16]
}

use glium;

pub struct Block {
	pub id: u8,
	//pub image: glium::texture::RawImage2d<'static, u8>
	pub texture: glium::texture::Texture2d
}

use utils;

impl Block {
	pub fn new(display: &mut glium::Display, img: &str, id: u8) -> Block {
		Block {
			texture: glium::texture::Texture2d::new(display, utils::load_image_from_file(img)).unwrap(),
			id: id
		}
	}

	pub fn get_vertex_buffer(&self, display: &mut glium::Display) -> glium::VertexBuffer<Vertex> {
		let vertices = vec![Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0] }];
		glium::VertexBuffer::new(display, &vertices).unwrap()
	}

	pub fn get_index_buffer(&self, display: &mut glium::Display) -> glium::IndexBuffer<u16> {
		let indices = vec![0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7, 8, 9, 10, 10, 9, 11, 12, 13, 14, 14, 13, 15, 16, 17, 18, 18, 17, 19, 20, 21, 22, 22, 21, 23];
		glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap()
	}
}

use std::collections::HashMap;
pub struct Blocks {
	pub block_map: &'static mut HashMap<u8, Block>
}

impl Blocks {
	pub fn new() -> Blocks {
		Blocks {
			block_map: &mut HashMap::new()
		}
	}

	pub fn initialize(&mut self, display: &mut glium::Display) {
		self.block_map.insert(0, Block::new(display, "models/stone.png", 0));
	}
}

pub fn create_world(air_block: u8) -> World {
	use std::ptr;
	let mut chunk_array: [[Chunk; 16]; 16];

	for x in 0..16 {
		for z in 0..16 {
			chunk_array[x][z] = create_chunk(air_block);
		}
	}

	World {
		chunks: chunk_array
	}
}

fn create_chunk(air_block: u8) -> Chunk {
	let mut block_array: [[[u8; 255]; 16]; 16];

	for x in 0..16 {
		for z in 0..16 {
			for y in 0..256 {
				block_array[x][z][y] = air_block;
			}
		}
	}

	Chunk {
		blocks: block_array
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            position: [0.0; 3],
            uv: [0.0; 2]
        }
    }
}