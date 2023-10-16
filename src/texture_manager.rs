use std::collections::HashMap;
use sdl2::render::{Texture};

pub struct TextureManager<'a> {
    textures: HashMap<String, &'a Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, name: String, texture: &'a Texture<'a>) {
        self.textures.insert(name, texture);
    }

    pub fn get_texture(&self, name: &str) -> Option<&'a Texture<'a>> {
        self.textures.get(name).cloned()
    }
}