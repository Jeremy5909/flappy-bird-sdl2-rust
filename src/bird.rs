const GRAVITY: f32 = 1.0 / 3.0;
const JUMP_HEIGHT: f32 = 10.0;

use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub struct Bird {
    y: i32,
    velocity: f32,
}
impl Bird {
    pub fn new() -> Self {
        Self {
            y: 100,
            velocity: 0.0,
        }
    }
    pub fn update(&mut self) {
        self.velocity += GRAVITY;
        self.y += self.velocity as i32;
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        canvas
            .copy(texture, None, Some(Rect::new(10, self.y, 50, 40)))
            .unwrap();
    }
    pub fn jump(&mut self) {
        self.velocity = -JUMP_HEIGHT;
    }
}
