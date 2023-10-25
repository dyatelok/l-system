use euler::*;
use l_system::lsystems::{LSystem, LToken};
use l_system::turtle::{TurtleAction, TurtleDrawer, TurtleToken};
use raylib::color::Color;
use std::f32::consts::PI;

#[derive(Copy, Clone, PartialEq)]
enum HilbertToken {
    Forward,
    RotateL,
    RotateR,
    MetaL,
    MetaR
}

impl TurtleToken for HilbertToken {
    fn action(t: Self) -> Vec<TurtleAction> {
        use HilbertToken as HT;
        use TurtleAction as TA;
        match t {
            HT::Forward => vec![TA::Move(3.5)],
            HT::RotateL => vec![TA::Rotate(PI / 2.0)],
            HT::RotateR => vec![TA::Rotate(-PI / 2.0)],
            _ => vec![],
        }
    }
}

impl LToken for HilbertToken {
    fn apply(token: Self) -> Vec<Self> {
        use HilbertToken as HT;
        match token {
            HT::MetaR => vec![HT::RotateR, HT::MetaL, HT::Forward, HT::RotateL, HT::MetaR, HT::Forward, HT::MetaR, HT::RotateL, HT::Forward, HT::MetaL, HT::RotateR],
            HT::MetaL => vec![HT::RotateL, HT::MetaR, HT::Forward, HT::RotateR, HT::MetaL, HT::Forward, HT::MetaL, HT::RotateR, HT::Forward, HT::MetaR, HT::RotateL],
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
    use HilbertToken as HT;
    use TurtleAction as TA;
    let turtle = TurtleDrawer::new(
        vec2![1000.0, 1000.0],
        vec2![1.0, 1.0],
        vec2![50.0, 950.0],
        Color::BLACK,
    );

    let system = LSystem::from(vec![HT::MetaR]);

    let sequence = system.iterate(8);

    let mut actions = vec![TA::SetThick(1.0), TA::Push];

    actions.extend(TurtleToken::actions(sequence));

    actions.push(TA::Pop);
    
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
