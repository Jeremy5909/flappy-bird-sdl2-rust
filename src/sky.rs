use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub struct Sky {
    x: f32,
    speed: f32,
}
impl Sky {
    pub fn new(speed: f32) -> Self {
        Self { x: 0.0, speed }
    }

    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        canvas
            .copy(
                texture,
                None,
                Rect::new(
                    -self.x as i32,
                    0,
                    canvas.window().size().0,
                    canvas.window().size().1,
                ),
            )
            .unwrap();
        canvas
            .copy(
                texture,
                None,
                Rect::new(
                    -self.x as i32 + canvas.window().size().0 as i32,
                    0,
                    canvas.window().size().0,
                    canvas.window().size().1,
                ),
            )
            .unwrap();
    }
    pub fn update(&mut self, canvas: &Canvas<Window>) {
        self.x = (self.x + self.speed) % (canvas.window().size().0) as f32;
    }
}
