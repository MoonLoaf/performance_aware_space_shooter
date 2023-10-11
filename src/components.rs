use specs::{prelude::*};
use specs_derive::Component;
use vector2d::Vector2D;

#[derive(Component)]
pub struct Renderable {
    pub texture_name: String,
    pub img_width: u32,
    pub img_height: u32,
    pub output_width: u32,
    pub output_height: u32,
    pub img_rotation: f64
}

#[derive(Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rot: f64
}

#[derive(Component)]
pub struct Player {
    pub impulse: Vector2D<f64>,
    pub current_speed: Vector2D<f64>,
    pub rotation_speed: f64,
    pub max_speed: f64,
    pub friction: f64
}