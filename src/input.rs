use glium::glutin::{VirtualKeyCode, MouseButton};
use std::collections::HashMap;

pub struct Input {
    pub key_map: HashMap<VirtualKeyCode, bool>,
    pub key_down_map: HashMap<VirtualKeyCode, bool>,
    pub mouse_map: HashMap<MouseButton, bool>,
    pub mouse_down_map: HashMap<MouseButton, bool>
}

impl Input {
	pub fn new() -> Input {
		Input {
			key_map: HashMap::new(),
            key_down_map: HashMap::new(),
            mouse_map: HashMap::new(),
            mouse_down_map: HashMap::new()
		}
	}

    pub fn set_key(&mut self, key: VirtualKeyCode, status: bool) {
        self.key_map.insert(key, status);
    }

    pub fn set_key_down(&mut self, key: VirtualKeyCode, status: bool) {
        self.key_down_map.insert(key, status);
    }

    pub fn get_key(&mut self, key: VirtualKeyCode) -> bool {
        match self.key_map.get(&key) {
            Some(down) => *down,
            _ => false
        }
    }

    pub fn get_key_down(&mut self, key: VirtualKeyCode) -> bool {
        match self.key_down_map.get(&key) {
            Some(down) => *down,
            _ => false
        }
    }

    pub fn set_button(&mut self, button: MouseButton, status: bool) {
        self.mouse_map.insert(button, status);
    }

    pub fn set_button_down(&mut self, button: MouseButton, status: bool) {
        self.mouse_down_map.insert(button, status);
    }

    pub fn get_button(&mut self, button: MouseButton) -> bool {
        match self.mouse_map.get(&button) {
            Some(down) => *down,
            _ => false
        }
    }

    pub fn get_button_down(&mut self, button: MouseButton) -> bool {
        match self.mouse_down_map.get(&button) {
            Some(down) => *down,
            _ => false
        }
    }
}