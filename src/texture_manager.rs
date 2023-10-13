use std::collections::HashMap;
use sdl2::render::{Texture};

pub struct TextureManager<'r> {
    textures: HashMap<String, &'r Texture<'r>>,
}

impl<'r> TextureManager<'r> {
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, name: String, texture: &'r Texture<'r>) {
        self.textures.insert(name, texture);
    }

    pub fn get_texture(&self, name: &str) -> Option<&'r Texture<'r>> {
        self.textures.get(name).cloned()
    }
}