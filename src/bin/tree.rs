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
    fn act(t: Self) -> &'static [TurtleAction] {
        use TreeToken as TT;
        use TurtleAction as TA;
        match t {
            TT::Leaf => &[TA::SetColot(Color::LIME), TA::Move(10.0)],
            TT::NewWood => &[TA::SetColot(Color::BROWN), TA::Move(10.0)],
            TT::OldWood => &[TA::SetColot(Color::BLACK), TA::Move(10.0)],
            TT::RotateLeft => &[TA::Rotate(PI / 3.0)],
            TT::RotateRight => &[TA::Rotate(-PI / 3.0)],
            TT::Push => &[TA::Push],
            TT::Pop => &[TA::Pop],
        }
    }
}

fn main() {
    use TreeToken as TT;
    let turtle = Turtle::new(
        vec2![1000.0, 1000.0],
        vec2![1.0, 1.0],
        vec2![500.0, 500.0],
        Color::WHITE,
    );

    let runner = Runner::from(vec![TT::Leaf]);

    let sequence = runner.iterate(5);

    let actions = sequence
        .into_iter()
        .map(TurtleToken::act)
        .fold(Vec::new(), |mut acc, elem| {
            acc.extend_from_slice(elem);
            acc
        });

    turtle.run(actions);
}
