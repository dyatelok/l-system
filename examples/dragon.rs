use euler::*;
use l_system::lsystems::{LSystem, LToken};
use l_system::turtle::{TurtleAction, TurtleDrawer, TurtleToken};
use raylib::color::Color;
use std::f32::consts::PI;

#[derive(Copy, Clone, PartialEq)]
enum DragonToken {
    Forward,
    Rotate(f32),
    X,
    Y,
    Push,
    Pop,
}

impl TurtleToken for DragonToken {
    fn action(t: Self) -> Vec<TurtleAction> {
        use DragonToken as DT;
        use TurtleAction as TA;
        match t {
            DT::Forward => vec![TA::Move(3.0)],
            DT::Rotate(rot) => vec![TA::Rotate(rot)],
            DT::Push => vec![TA::Push],
            DT::Pop => vec![TA::Pop],
            _ => vec![],
        }
    }
}

impl LToken for DragonToken {
    fn apply(token: Self) -> Vec<Self> {
        use DragonToken as DT;
        match token {
            DT::X => vec![DT::X, DT::Rotate(PI / 2.0), DT::Y, DT::Forward],
            DT::Y => vec![DT::Forward, DT::X, DT::Rotate(-PI / 2.0), DT::Y],
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
        vec2![300.0, 700.0],
        Color::BLACK,
    );

    let system = LSystem::from(vec![DT::Push, DT::Forward, DT::X, DT::Pop]);

    let sequence = system.iterate(15);

    let mut actions = vec![TA::SetThick(1.0)];

    actions.extend(TurtleToken::actions(sequence));

    let times_forward = actions
        .iter()
        .filter(|&e| matches!(*e, TA::Move(_)))
        .count() as f32;
    let mut num = 0.0;

    let mut new_actions = Vec::new();

    let c1 = Color::YELLOW;
    let c2 = Color::RED;

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
