use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub struct Base {
    x: i32,
    speed: u32,
}
impl Base {
    pub fn new(speed: u32) -> Self {
        Self { x: 0, speed }
    }
    pub fn update(&mut self, canvas: &Canvas<Window>) {
        self.x = (self.x + self.speed as i32) % (canvas.window().size().0 as i32);
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        let (width, height) = canvas.window().size();
        canvas
            .copy(
                texture,
                None,
                Some(Rect::new(
                    -self.x,
                    3 * (height as i32) / 4,
                    width,
                    height / 4,
                )),
            )
            .unwrap();
        canvas
            .copy(
                texture,
                None,
                Some(Rect::new(
                    -self.x + width as i32,
                    3 * (height as i32) / 4,
                    width,
                    height / 4,
                )),
            )
            .unwrap();
    }
}
