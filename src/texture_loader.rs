use sdl2::{render::Canvas, video::Window};

struct TextureLoader {
    canvas: &mut Canvas<Window>,
}
impl TextureLoader {
    fn new(canvas: &mut Canvas<Window>) -> Self {
        Self { canvas }
    }
}
