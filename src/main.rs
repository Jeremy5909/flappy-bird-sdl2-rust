use crate::bird::Bird;
use std::time::Duration;

use base::Base;
use pipe_handler::PipeHandler;
use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color};
use sky::Sky;

mod base;
mod bird;
mod pipe;
mod pipe_handler;
mod sky;
mod wrap_texture;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("Flappy Bird", 432, 768)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let sky_texture = texture_creator.load_texture("assets/sky.png").unwrap();
    let bird_texture = texture_creator.load_texture("assets/bird.png").unwrap();
    let base_texture = texture_creator.load_texture("assets/base.png").unwrap();
    let pipe_texture = texture_creator.load_texture("assets/pipe.png").unwrap();

    canvas.set_draw_color(Color::RGB(145, 246, 250));

    let mut bird = Bird::default();
    let mut base = Base::new(5.0);
    let mut sky = Sky::new(0.5);
    let mut pipes = PipeHandler::new(5.0, 30, 280);
    pipes.spawn(&canvas);

    let mut i = 0;
    'running: loop {
        i = (i + 1) % 59;

        canvas.clear();

        if i == 0 {
            pipes.spawn(&canvas);
        }

        bird.update();
        base.update(&canvas);
        sky.update(&canvas);
        pipes.update();

        let base_colliding = base.colliding(bird.position(), &canvas);
        let roof_colliding = bird.position().1 <= 0;
        let pipes_colliding = pipes.colliding(bird.rect(), &mut canvas);
        if base_colliding || roof_colliding || pipes_colliding {
            break;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => bird.jump(),
                _ => {}
            }
        }

        sky.render(&sky_texture, &mut canvas);
        pipes.render(&pipe_texture, &mut canvas);
        base.render(&base_texture, &mut canvas);
        bird.render(&bird_texture, &mut canvas);

        canvas.present();
        std::thread::sleep(Duration::from_secs_f32(1f32 / 60f32));
    }
}
