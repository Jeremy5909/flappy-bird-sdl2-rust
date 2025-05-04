use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

use crate::wrap_texture::{render_wrapped, update_wrap_x};

pub struct Base {
    x: f32,
    speed: f32,
}
impl Base {
    pub fn new(speed: f32) -> Self {
        Self { x: 0.0, speed }
    }
    pub fn update(&mut self, canvas: &Canvas<Window>) {
        update_wrap_x(&mut self.x, self.speed, canvas);
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        let (width, height) = canvas.window().size();
        render_wrapped(
            &self.x,
            3 * (height as i32) / 4,
            width,
            height / 4,
            texture,
            canvas,
        );
    }
}
