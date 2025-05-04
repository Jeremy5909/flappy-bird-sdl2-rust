use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

use crate::wrap_texture::{render_wrapped, update_wrap_x};

pub struct Sky {
    x: f32,
    speed: f32,
}
impl Sky {
    pub fn new(speed: f32) -> Self {
        Self { x: 0.0, speed }
    }

    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        render_wrapped(
            &self.x,
            0,
            canvas.window().size().0,
            canvas.window().size().1,
            texture,
            canvas,
        );
    }
    pub fn update(&mut self, canvas: &Canvas<Window>) {
        update_wrap_x(&mut self.x, self.speed, canvas);
    }
}
