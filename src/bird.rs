use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub struct Bird {
    x: i32,
    y: i32,
    velocity: f32,
    gravity: f32,
    jump_height: f32,
}
impl Bird {
    pub fn new(x: i32, y: i32, gravity: f32, jump_height: f32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
            gravity,
            jump_height,
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.gravity;
        self.y += self.velocity as i32;
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        canvas
            .copy(texture, None, Some(Rect::new(self.x, self.y, 50, 40)))
            .unwrap();
    }
    pub fn jump(&mut self) {
        self.velocity = -self.jump_height;
    }
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Default for Bird {
    fn default() -> Self {
        Self::new(20, 50, 1.0 / 3.0, 10.0)
    }
}
