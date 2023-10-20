use euler::{vec2, Vec2};
use raylib::color::Color;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use std::f32::consts::PI;
use std::f32::consts::TAU;

struct Line {
    start: Vec2,
    end: Vec2,
    color: Color,
}

impl Line {
    fn from(start: Vec2, end: Vec2, color: Color) -> Line {
        Line { start, end, color }
    }
}

#[derive(Copy, Clone)]
pub enum TurtleAction {
    LowerPen,
    RaisePen,
    Rotate(f32),
    Move(f32),
    SetColot(Color),
    Push,
    Pop,
}

#[derive(Copy, Clone)]
pub struct TurtleSlice {
    color: Color,
    position: Vec2,
    direction: f32,
    drawing: bool,
}

impl TurtleSlice {
    fn new() -> Self {
        Self {
            color: Color::WHITE,
            position: vec2!(0.0, 0.0),
            direction: PI / 2.0,
            drawing: true,
        }
    }
    fn lower_pen(&mut self) {
        self.drawing = true;
    }
    fn raise_pen(&mut self) {
        self.drawing = false;
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn move_forward(&mut self, length: f32) -> Option<Line> {
        let old_position = self.position;
        self.position = vec2!(
            self.position.x + length * self.direction.cos(),
            self.position.y + length * self.direction.sin()
        );
        if self.drawing {
            Some(Line::from(old_position, self.position, self.color))
        } else {
            None
        }
    }
    fn rotate(&mut self, angle: f32) {
        self.direction += angle;
        if self.direction > TAU {
            self.direction -= TAU;
        } else if self.direction < 0.0 {
            self.direction += TAU;
        }
    }
}

pub struct Turtle {
    slices: Vec<TurtleSlice>,
    turtle: TurtleSlice,
    background: Color,
    screen: Vec2,
    scale: Vec2,
    origin: Vec2,
}

use raylib::ffi::Vector2;

impl Turtle {
    pub fn new(screen: Vec2, scale: Vec2, origin: Vec2, background: Color) -> Turtle {
        Turtle {
            slices: Vec::new(),
            turtle: TurtleSlice::new(),
            background,
            screen,
            scale,
            origin,
        }
    }
    pub fn execute(&mut self, actions: &Vec<TurtleAction>, drawer: &mut RaylibDrawHandle) {
        for action in actions {
            self.execute_once(action, drawer);
        }
    }
    pub fn execute_once(&mut self, action: &TurtleAction, drawer: &mut RaylibDrawHandle) {
        use TurtleAction as TA;
        match action {
            TA::LowerPen => self.turtle.lower_pen(),
            TA::RaisePen => self.turtle.raise_pen(),
            TA::Rotate(angle) => self.turtle.rotate(*angle),
            TA::Move(length) => match self.turtle.move_forward(*length) {
                Some(line) => {
                    drawer.draw_line_ex(
                        Vector2 {
                            x: self.origin.x + self.scale.x * line.start.x,
                            y: self.origin.y - self.scale.y * line.start.y,
                        },
                        Vector2 {
                            x: self.origin.x + self.scale.x * line.end.x,
                            y: self.origin.y - self.scale.y * line.end.y,
                        },
                        3.0,
                        line.color,
                    );
                }
                None => {}
            },
            TA::SetColot(color) => self.turtle.set_color(*color),
            TA::Push => self.slices.push(self.turtle),
            TA::Pop => self.turtle = self.slices.pop().unwrap(),
        }
    }
    pub fn run(mut self, actions: Vec<TurtleAction>) {
        let (mut rl, thread) = raylib::init()
            .size(self.screen.x as i32, self.screen.y as i32)
            .title("turtle")
            .build();

        rl.set_target_fps(1);
        while !rl.window_should_close() {
            let mut drawer = rl.begin_drawing(&thread);
            drawer.clear_background(self.background);
            self.execute(&actions, &mut drawer);
        }
    }
}

pub trait TurtleToken {
    fn apply(token: &Self) -> &[Self]
    where
        Self: Sized;
    fn act(t: Self) -> &'static [TurtleAction];
}

pub struct Runner<T: Eq + TurtleToken + Clone> {
    axiom: Vec<T>,
}

impl<T: Eq + TurtleToken + Clone> Runner<T> {
    pub fn from(axiom: Vec<T>) -> Self {
        Self { axiom }
    }
    fn apply_once(tokens: &mut Vec<T>) {
        *tokens = Self::apply_once_helper(std::mem::take(tokens));
    }
    fn apply_once_helper(tokens: Vec<T>) -> Vec<T> {
        tokens
            .iter()
            .map(|a| TurtleToken::apply(a))
            .fold(Vec::new(), |mut acc, elem| {
                acc.extend_from_slice(elem);
                acc
            })
    }
    pub fn iterate(&self, n: usize) -> Vec<T> {
        let mut tokens = self.axiom.clone();
        for _ in 0..n {
            Self::apply_once(&mut tokens);
        }
        tokens
    }
}
