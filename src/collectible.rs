use nalgebra_glm::Vec2 ;
use crate::Arc;
use crate::Texture;

pub struct Collectible {
    pub position: Vec2,
    pub texture: Arc<Texture>,
    pub collected: bool,
}

impl Collectible {
    pub fn new(x: f32, y: f32, texture: Arc<Texture>) -> Collectible {
        Collectible {
            position: Vec2::new(x, y),
            texture,
            collected: false,
        }
    }
}
