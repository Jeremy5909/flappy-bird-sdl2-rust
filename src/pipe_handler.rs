use crate::pipe::Pipe;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

pub struct PipeHandler {
    speed: f32,
    min_height: i32,
    max_height: i32,
    pipes: Vec<Pipe>,
}
impl PipeHandler {
    pub fn new(speed: f32, min_height: i32, max_height: i32) -> Self {
        Self {
            speed,
            min_height,
            max_height,
            pipes: Vec::new(),
        }
    }

    pub fn spawn(&mut self, canvas: &Canvas<Window>) {
        self.pipes.push(Pipe::new(
            self.speed,
            self.min_height,
            self.max_height,
            canvas,
        ));
    }
    pub fn update(&mut self, player_x: f32, player_score: &mut u32) {
        self.pipes
            .iter_mut()
            .for_each(|pipe| pipe.update(player_x, player_score));
        self.pipes.retain(|pipe| pipe.x >= -52.0);
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        self.pipes
            .iter()
            .for_each(|pipe| pipe.render(texture, canvas));
    }

    pub fn colliding(&self, target: Rect, canvas: &mut Canvas<Window>) -> bool {
        for pipe in &self.pipes {
            if pipe
                .rect(canvas.window().size().1 as i32)
                .has_intersection(target)
            {
                return true;
            }
        }
        false
    }
}
