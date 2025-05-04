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
    pub x: f32,
    height: i32,
    side: Side,
    speed: f32,
    passed: bool,
}
impl Pipe {
    pub fn new(speed: f32, min_height: i32, max_height: i32, canvas: &Canvas<Window>) -> Self {
        Self {
            x: canvas.window().size().1 as f32,
            height: random_range(min_height..max_height),
            side: random(),
            speed,
            passed: false,
        }
    }
    pub fn rect(&self, window_height: i32) -> Rect {
        let ground_height = window_height / 4;
        let texture_height = 320;
        match self.side {
            Side::Top => {
                let y = -texture_height + self.height;
                Rect::new(self.x as i32, y, 52, texture_height as u32)
            }
            Side::Bottom => {
                let y = window_height - ground_height - self.height;
                Rect::new(self.x as i32, y, 52, 320)
            }
        }
    }
    pub fn update(&mut self, player_x: f32, player_score: &mut u32) {
        self.x -= self.speed;
        if player_x >= self.x && !self.passed {
            self.passed = true;
            *player_score += 1;
            println!("{}", player_score);
        }
    }
    pub fn render(&self, texture: &Texture, canvas: &mut Canvas<Window>) {
        let window_height = canvas.window().size().1 as i32;
        canvas
            .copy_ex(
                texture,
                None,
                Some(self.rect(window_height)),
                0.0,
                None,
                false,
                match self.side {
                    Side::Bottom => false,
                    Side::Top => true,
                },
            )
            .unwrap();
    }
}
