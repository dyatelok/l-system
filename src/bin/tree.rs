use euler::*;
use l_system::{Runner, Turtle, TurtleAction, TurtleToken};
use raylib::color::Color;
use std::f32::consts::PI;

#[derive(Copy, Clone, Eq, PartialEq)]
enum TreeToken {
    Leaf,
    RotateLeft,
    RotateRight,
    OldWood,
    NewWood,
    Push,
    Pop,
}

impl TurtleToken for TreeToken {
    fn apply(token: &Self) -> &[Self] {
        use TreeToken as TT;
        match *token {
            TT::Leaf => &[
                TT::Push,
                TT::NewWood,
                TT::Push,
                TT::RotateLeft,
                TT::Leaf,
                TT::Pop,
                TT::RotateRight,
                TT::Leaf,
                TT::Pop,
            ],
            TT::NewWood => &[TT::OldWood, TT::NewWood],
            TT::RotateLeft => &[TT::RotateLeft],
            TT::RotateRight => &[TT::RotateRight],
            TT::OldWood => &[TT::OldWood],
            TT::Push => &[TT::Push],
            TT::Pop => &[TT::Pop],
        }
    }
    fn act(token: Self) -> &'static [TurtleAction] {
        use TreeToken as TT;
        use TurtleAction as TA;
        match token {
            TT::Leaf => &[TA::SetColor(Color::LIME), TA::Move(20.0)],
            TT::NewWood => &[TA::SetColor(Color::BROWN), TA::Move(20.0)],
            TT::OldWood => &[TA::SetColor(Color::BLACK), TA::Move(20.0)],
            TT::RotateLeft => &[TA::Rotate(PI / 6.0)],
            TT::RotateRight => &[TA::Rotate(-PI / 6.0)],
            TT::Push => &[TA::Push],
            TT::Pop => &[TA::Pop],
        }
    }
}

fn main() {
    use TreeToken as TT;
    let turtle = Turtle::new(
        vec2![1000.0, 1000.0],
        vec2![0.5, 0.5],
        vec2![500.0, 700.0],
        Color::WHITE,
    );

    let runner = Runner::from(vec![TT::Leaf]);

    let actions = runner.actions(10);

    turtle.run(actions);
}
