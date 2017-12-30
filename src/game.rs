pub struct Game {
	pub world: World,
	pub render_distance: u8
}

impl Game {
	pub fn new(air_block: u8, render_distance: u8) -> Game {
		Game {
			world: create_world(air_block, render_distance as usize),
			render_distance: render_distance
		}
	}
}

pub struct World {
	pub chunks: Vec<Vec<Chunk>>
}

use Instance;
impl World {
	pub fn get_chunk(&self, x: usize, y: usize) -> &Chunk {
		&self.chunks[x][y]
	}

	pub fn get_block_id(&self, x: u32, y: u8, z: u32) -> u8 {
		self.chunks[(x >> 4) as usize][(z >> 4) as usize].blocks[(x & 15) as usize][(z & 15) as usize][y as usize]
	}

	pub fn get_block<'a>(&self, blocks: &'a Blocks, x: u32, y: u8, z: u32) -> &'a Block {
		blocks.get_block(self.get_block_id(x, y, z))
	}

	pub fn is_solid_block(&self, x: f32, y: f32, z: f32) -> bool {
		self.get_block_id((x + 0.5) as u32, (y + 0.5) as u8, (z + 0.5) as u32) > 0
	}

	pub fn set_block(&mut self, x: u32, y: u8, z: u32, block: &Block) {
		self.set_block_ignore_neighbors(x, y, z, block.id);

		for block_pos in &mut self.get_facial_neighbors(x as i64, y as i16, z as i64) {
			if block_pos.block_id != 0 {
				self.set_block_ignore_neighbors(block_pos.x, block_pos.y, block_pos.z, block_pos.block_id);
			}
		}
	}

	pub fn get_instance_buffer(&mut self, display: &mut glium::Display) -> glium::VertexBuffer<Instance> {
		let mut vec = Vec::new();

		for chunk_x in 0..self.chunks.len() {
            for chunk_z in 0..self.chunks[chunk_x].len() {
            	for block in &self.chunks[chunk_x][chunk_z].visible_blocks {
            		let mut mat = utils::get_identity_matrix();
            		mat[(0, 3)] = block.x as f32;
					mat[(1, 3)] = block.y as f32;
					mat[(2, 3)] = block.z as f32;

            		vec.push(Instance {
            			matrix: mat.into(),
            			id: block.block_id
            		});
            	}
            }
        }

        glium::VertexBuffer::new(display, &vec).unwrap()
	}

	fn set_block_ignore_neighbors(&mut self, raw_x: u32, raw_y: u8, raw_z: u32, block: u8) {
		let x = (raw_x & 15) as u8;
		let y = raw_y as u8;
		let z = (raw_z & 15) as u8;

		let ux = x as usize;
		let uy = y as usize;
		let uz = z as usize;

		self.chunks[(raw_x >> 4) as usize][(raw_z >> 4) as usize].blocks[ux][uz][uy] = block;

		if block != 0 {
			for block_pos in &mut self.get_facial_neighbors(raw_x as i64, y as i16, raw_z as i64) {
				if block_pos.block_id == 0 {
					self.chunks[(raw_x >> 4) as usize][(raw_z >> 4) as usize].visible_blocks.insert(BlockPos::new(raw_x as u32, y, raw_z as u32, block));
					return;
				}
			}
		}

		self.chunks[(raw_x >> 4) as usize][(raw_z >> 4) as usize].visible_blocks.remove(&BlockPos::new(raw_x as u32, y, raw_z as u32, block));
	}
	
	pub fn is_in_world_bounds(&self, x: i64, y: i16, z: i64) -> bool {
		x >= 0 && z >= 0 && y >= 0 && y <= 255
	}

	pub fn is_in_rendered_world_bounds(&self, render_distance: u8, x: i64, y: i16, z: i64) -> bool {
		x >= 0 && z >= 0 && y >= 0 && y <= 255 && x + 1 < render_distance as i64 * 16 && z + 1 < render_distance as i64 * 16
	}

	fn add_if_in_bounds(&self, vec: &mut Vec<BlockPos>, x: i64, y: i16, z: i64) {
		if self.is_in_world_bounds(x, y, z) {
			vec.push(BlockPos::new(x as u32, y as u8, z as u32, self.get_block_id(x as u32, y as u8, z as u32)));
		} else {
			vec.push(BlockPos::new(0, 0, 0, 0));
		}
	}

	pub fn get_facial_neighbors(&mut self, x: i64, y: i16, z: i64) -> Vec<BlockPos> {
		let mut neighbors = Vec::new();

		self.add_if_in_bounds(&mut neighbors, x, y + 1, z);
		self.add_if_in_bounds(&mut neighbors, x, y - 1, z);
		self.add_if_in_bounds(&mut neighbors, x + 1, y, z);
		self.add_if_in_bounds(&mut neighbors, x - 1, y, z);
		self.add_if_in_bounds(&mut neighbors, x, y, z + 1);
		self.add_if_in_bounds(&mut neighbors, x, y, z - 1);

		neighbors
	}
}

#[derive(Debug)]
pub struct BlockPos {
	pub x: u32,
	pub y: u8,
	pub z: u32,
	pub block_id: u8
}

impl BlockPos {
	fn new(x: u32, y: u8, z: u32, block_id: u8) -> BlockPos {
		BlockPos {
			x: x,
			y: y,
			z: z,
			block_id: block_id
		}
	}
}

impl PartialEq for BlockPos {
	fn eq(&self, other: &BlockPos) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

use std::hash::{Hash, Hasher};
impl Hash for BlockPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl Eq for BlockPos {}

use std::collections::HashSet;
pub struct Chunk {
	pub blocks: Vec<Vec<Vec<u8>>>,
	pub visible_blocks: HashSet<BlockPos>
}

use glium;

pub struct Block {
	pub id: u8,
}

use utils;

impl Block {
	pub fn new(id: BlockType) -> Block {
		Block {
			id: id as u8
		}
	}

	pub fn get_vertex_buffer(display: &mut glium::Display) -> glium::VertexBuffer<Vertex> {
		let mut vertices = vec![Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 1.0], face: 0 }, //0 back
								Vertex { position: [0.5, -0.5, 0.5], uv: [1.0, 1.0], face: 0 }, //1
								Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0], face: 0 }, //2
								Vertex { position: [0.5, 0.5, 0.5], uv: [1.0, 0.0], face: 0 }, //3

								Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0], face: 1 }, //4 top
								Vertex { position: [0.5, 0.5, 0.5], uv: [1.0, 0.0], face: 1 }, //5
								Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 1.0], face: 1 }, //6
								Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 1.0] , face: 1 }, //7

								Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0], face: 2 }, //8 front
								Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 0.0], face: 2 }, //9
								Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 1.0], face: 2 }, //10
								Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 1.0], face: 2 }, //11

								Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 1.0], face: 3 }, //12 bottom
								Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 1.0], face: 3 }, //13
								Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0], face: 3 }, //14
								Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 1.0], face: 3 }, //15

								Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 1.0], face: 4}, //16 right
								Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 1.0], face: 4 }, //17
								Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0], face: 4 }, //18
								Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 0.0], face: 4 }, //19

								Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 1.0], face: 5 }, //20 left
								Vertex { position: [-0.5, -0.5, 0.5], uv: [1.0, 1.0], face: 5 }, //21
								Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0], face: 5 }, //22		
								Vertex { position: [-0.5, 0.5, 0.5], uv: [1.0, 0.0], face: 5 } //23
								];
		for v in &mut vertices {
			v.uv[1] = 1.0 - v.uv[1];
		}
		glium::VertexBuffer::new(display, &vertices).unwrap()
	}

	pub fn get_index_buffer(display: &mut glium::Display) -> glium::IndexBuffer<u16> {
		let indices = vec![0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7, 8, 9, 10, 10, 9, 11, 12, 13, 14, 14, 13, 15, 16, 17, 18, 18, 17, 19, 20, 21, 22, 22, 21, 23];
		//let indices = vec![12, 13, 14, 14, 13, 15];
		glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap()
	}
}

pub enum BlockType {
	Air = 0,
	Cobblestone = 1,
	Dirt = 2,
	Grass = 3
}

pub struct Blocks {
	pub block_map: Vec<Block>
}

impl Blocks {
	pub fn new() -> Blocks {
		Blocks {
			block_map: Vec::new()
		}
	}

	pub fn initialize(&mut self) {
		self.block_map.push(Block::new(BlockType::Air));
		self.block_map.push(Block::new(BlockType::Cobblestone));
		self.block_map.push(Block::new(BlockType::Dirt));
		self.block_map.push(Block::new(BlockType::Grass));
	}

	pub fn get_block(&self, id: u8) -> &Block {
		self.block_map.get(id as usize).unwrap()
	}

	pub fn get_block_count(&self) -> u8 {
		self.block_map.len() as u8
	}
}

pub fn create_world(air_block: u8, render_distance: usize) -> World {
	let mut chunk_array = Vec::with_capacity(render_distance);

	for x in 0..render_distance {
		chunk_array.push(Vec::with_capacity(render_distance));
		for _ in 0..render_distance {
			chunk_array[x].push(create_chunk(air_block));
		}
	}

	World {
		chunks: chunk_array
	}
}

fn create_chunk(air_block: u8) -> Chunk {
	let mut block_array = Vec::with_capacity(16);

	for x in 0..16 {
		block_array.push(Vec::with_capacity(16));
		for z in 0..16 {
			block_array[x].push(Vec::with_capacity(255));
			for _ in 0..256 {
				block_array[x][z].push(air_block);
			}
		}
	}

	Chunk {
		blocks: block_array,
		visible_blocks: HashSet::new()
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub face: u8
}

/*impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            position: [0.0; 3],
            uv: [0.0; 2]
        }
    }
}*/