use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
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

    pub fn add_ui_texture<'b>(&mut self, name: String, texture_creator: &'b TextureCreator<WindowContext>, font: &sdl2::ttf::Font, text: &String) -> Result<(), String>
        where 'b: 'a
    {
        let surface = font.render(text).solid(Color::RGB(255, 255, 255)).map_err(|e|e.to_string())?;
        let surface_texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        self.textures.insert(name, surface_texture);
        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&'a Texture> {
        self.textures.get(name)
    }
}