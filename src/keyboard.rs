use winit::keyboard::{KeyCode, PhysicalKey};

pub struct Keyboard {
    keys: Vec<PhysicalKey>,
}

impl Keyboard {
    pub fn press(&mut self, key: PhysicalKey) {
        if !self.keys.contains(&key) {
            self.keys.push(key);
        }
    }

    pub fn release(&mut self, key: PhysicalKey) {
        let mut i = 0;
        let mut exists = false;
        for k in &self.keys {
            if key == *k {
                exists = true;
                break;
            }
            i += 1;
        }

        if exists {
            self.keys.remove(i);
        }
    }

    pub fn down(&self, key: KeyCode) -> bool {
        let physical = winit::keyboard::PhysicalKey::Code(key);
        self.keys.contains(&physical)
    }

    pub fn any_key_down(&self) -> bool {
        self.keys.len() != 0
    }

    pub fn new() -> Keyboard {
        Keyboard { keys: vec![] }
    }
}
