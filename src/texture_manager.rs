use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct TextureManager<'a> {
    textures: HashMap<String, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, name: String, texture_creator: &'a TextureCreator<WindowContext>) -> Result<(), String> {
        let texture = texture_creator.load_texture(&name)?;
        self.textures.insert(name, texture);
        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&'a Texture> {
        self.textures.get(name)
    }
}