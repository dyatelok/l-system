use euler::*;
use l_system::lsystems::{LSystem, LToken};
use l_system::turtle::{TurtleAction, TurtleDrawer, TurtleToken};
// use lazy_static::lazy_static;
use raylib::color::Color;
use std::f32::consts::PI;

#[derive(Copy, Clone, PartialEq)]
enum DragonToken {
    LineF(f32),
    LineB(f32),
    Rot(f32),
}

impl TurtleToken for DragonToken {
    fn action(t: Self) -> Vec<TurtleAction> {
        use DragonToken as DT;
        use TurtleAction as TA;
        match t {
            DT::LineF(l) | DT::LineB(l) => vec![TA::Move(l)],
            DT::Rot(rot) => vec![TA::Rotate(rot)],
        }
    }
}

static THETA1: f32 = 0.82006;
static THETA2: f32 = 0.57411;

static M1: f32 = 0.5516670817897428991373945094665101561710649903290951692334613572;
static M2: f32 = 0.7427429446246816413695660476057885141497552527069779641441434078;

impl LToken for DragonToken {
    fn apply(token: Self) -> Vec<Self> {
        use DragonToken as DT;
        match token {
            DT::LineF(l) => vec![
                DT::Rot(-THETA1),
                DT::LineB(l * M1),
                DT::Rot(THETA1 + THETA2),
                DT::LineF(l * M2),
                DT::Rot(-THETA2),
            ],
            DT::LineB(l) => vec![
                DT::Rot(THETA2),
                DT::LineB(l * M2),
                DT::Rot(-(THETA1 + THETA2)),
                DT::LineF(l * M1),
                DT::Rot(THETA1),
            ],
            _ => vec![token],
        }
    }
}

fn add(c1: Color, c2: Color) -> Color {
    Color {
        r: c1.r + c2.r,
        g: c1.g + c2.g,
        b: c1.b + c2.b,
        a: c1.a + c2.a,
    }
}

fn mul(c: Color, k: f32) -> Color {
    Color {
        r: (c.r as f32 * k) as u8,
        g: (c.g as f32 * k) as u8,
        b: (c.b as f32 * k) as u8,
        a: (c.a as f32 * k) as u8,
    }
}

fn lerp(c1: Color, c2: Color, k: f32) -> Color {
    add(mul(c1, 1.0 - k), mul(c2, k))
}

fn main() {
    use DragonToken as DT;
    use TurtleAction as TA;
    let turtle = TurtleDrawer::new(
        vec2![1000.0, 1000.0],
        vec2![1.0, 1.0],
        vec2![800.0, 850.0],
        Color::BLACK,
    );

    let system = LSystem::from(vec![DT::LineF(550.0 * 2.0_f32.sqrt())]);

    let sequence = system.iterate(17);

    let mut actions = vec![TA::Push, TA::SetThick(1.0), TA::Rotate(PI / 4.0)];

    actions.extend(TurtleToken::actions(sequence));

    actions.push(TA::Pop);

    // turtle.run(&actions);

    let times_forward = actions
        .iter()
        .filter(|&e| matches!(*e, TA::Move(_)))
        .count() as f32;
    let mut num = 0.0;

    let mut new_actions = Vec::new();

    let c1 = Color::RED;
    let c2 = Color::BLUE;

    for a in actions {
        if let TA::Move(_) = a {
            new_actions.extend(vec![TA::SetColor(lerp(c1, c2, num / times_forward)), a]);
            num += 1.0;
        } else {
            new_actions.push(a);
        }
    }

    turtle.run(&new_actions);
}
