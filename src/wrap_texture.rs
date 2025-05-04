use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub fn update_wrap_x(x: &mut f32, speed: f32, canvas: &Canvas<Window>) {
    *x = (*x + speed) % (canvas.window().size().0) as f32;
}

pub fn render_wrapped(
    x: &f32,
    y: i32,
    width: u32,
    height: u32,
    texture: &Texture,
    canvas: &mut Canvas<Window>,
) {
    canvas
        .copy(texture, None, Some(Rect::new(-x as i32, y, width, height)))
        .unwrap();
    canvas
        .copy(
            texture,
            None,
            Some(Rect::new(-x as i32 + width as i32, y, width, height)),
        )
        .unwrap();
}
