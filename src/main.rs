use std::time::Duration;

use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect};

mod texture_loader;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("Flappy Bird", 432, 768)
        .position_centered()
        .build()
        .unwrap();
    let (width, height) = window.size();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let bird_texture = texture_creator.load_texture("assets/bird.png").unwrap();
    let sky_texture = texture_creator.load_texture("assets/sky.png").unwrap();
    let base_texture = texture_creator.load_texture("assets/base.png").unwrap();

    canvas.set_draw_color(Color::RGB(145, 246, 250));

    'running: loop {
        canvas.clear();

        canvas.copy(&sky_texture, None, None).unwrap();
        canvas
            .copy(
                &base_texture,
                None,
                Rect::new(0, 3 * (height as i32) / 4, width, height / 4),
            )
            .unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_secs_f32(1f32 / 60f32));
    }
}
