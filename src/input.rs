use glium::glutin::VirtualKeyCode;
use std::collections::HashMap;

pub struct Input {
    pub key_map: HashMap<VirtualKeyCode, bool>
}

impl Input {
	pub fn new() -> Input {
		Input {
			key_map: HashMap::new()
		}
	}

    pub fn set_key(&mut self, key: VirtualKeyCode, status: bool) {
        self.key_map.insert(key, status);
    }

    pub fn get_key(&mut self, key: VirtualKeyCode) -> bool {
        match self.key_map.get(&key) {
            Some(down) => *down,
            _ => false
        }
    }
}