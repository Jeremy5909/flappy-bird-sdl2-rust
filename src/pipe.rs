use rand::{
    distr::{Distribution, StandardUniform},
    random, random_range, Rng,
};
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

#[derive(Debug)]
enum Side {
    Top,
    Bottom,
}
impl Distribution<Side> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
        match rng.random_range(0..=1) {
            0 => Side::Top,
            _ => Side::Bottom,
        }
    }
}

pub struct Pipe {
    x: f32,
    height: i32,
    side: Side,
    speed: f32,
}
impl Pipe {
    pub fn new(speed: f32, min_height: i32, max_height: i32, canvas: &Canvas<Window>) -> Self {
        Self {
            x: canvas.window().size().1 as f32,
            height: random_range(min_height..max_height),
            side: random(),
            speed,
        }
    }
    pub fn update(&mut self) {
        self.x -= self.speed;
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        let window_height = canvas.window().size().1 as i32;
        let ground_height = window_height / 4;
        match self.side {
            Side::Top => {
                let y = self.height - 320;
                canvas
                    .copy_ex(
                        texture,
                        None,
                        Some(Rect::new(self.x as i32, y, 52, 320)),
                        0.0,
                        None,
                        false,
                        true,
                    )
                    .unwrap();
            }
            Side::Bottom => {
                let y = window_height - ground_height - self.height;
                canvas
                    .copy(texture, None, Some(Rect::new(self.x as i32, y, 52, 320)))
                    .unwrap();
            }
        }
    }
}

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
    pub fn update(&mut self) {
        self.pipes.iter_mut().for_each(|pipe| pipe.update());
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        self.pipes
            .iter()
            .for_each(|pipe| pipe.render(texture, canvas));
    }
}
