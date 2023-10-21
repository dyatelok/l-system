use euler::*;
use l_system::{Runner, Turtle, TurtleAction, TurtleToken};
use raylib::color::Color;
use std::f32::consts::PI;

#[derive(Copy, Clone, Eq, PartialEq)]
enum DragonToken {
    Forward,
    RotateLeft,
    RotateRight,
    X,
    Y,
    Push,
    Pop,
}

impl TurtleToken for DragonToken {
    fn apply(token: &Self) -> &[Self] {
        use DragonToken as DT;
        match *token {
            DT::Forward => &[DT::Forward],
            DT::RotateLeft => &[DT::RotateLeft],
            DT::RotateRight => &[DT::RotateRight],
            DT::X => &[DT::X, DT::RotateLeft, DT::Y, DT::Forward],
            DT::Y => &[DT::Forward, DT::X, DT::RotateRight, DT::Y],
            DT::Push => &[DT::Push],
            DT::Pop => &[DT::Pop],
        }
    }
    fn act(t: Self) -> &'static [TurtleAction] {
        use DragonToken as DT;
        use TurtleAction as TA;
        match t {
            DT::Forward => &[TA::Move(3.0)],
            DT::RotateLeft => &[TA::Rotate(PI / 2.0)],
            DT::RotateRight => &[TA::Rotate(-PI / 2.0)],
            DT::Push => &[TA::Push],
            DT::Pop => &[TA::Pop],
            _ => &[],
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
    let turtle = Turtle::new(
        vec2![1000.0, 1000.0],
        vec2![1.0, 1.0],
        vec2![300.0, 700.0],
        Color::BLACK,
    );

    let runner = Runner::from(vec![DT::Push, DT::Forward, DT::X, DT::Pop]);

    let mut actions = vec![TA::SetThick(1.0)];
    actions.extend(runner.actions(15));

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

    turtle.run(new_actions);
}
