use euler::*;
use l_system::lsystems::{LSystemStochastic, LTokenStochastic};
use l_system::turtle::{TurtleAction, TurtleDrawer, TurtleToken};
use raylib::color::Color;
use std::f32::consts::PI;
use std::iter;

fn to_bounds(probabilities: Vec<f32>) -> Vec<f32> {
    let length = probabilities.len();

    iter::repeat(probabilities)
        .take(length)
        .enumerate()
        .map(|(i, elem)| elem.iter().take(i + 1).sum())
        .collect()
}

fn rule(rules: Vec<(f32, Vec<TreeToken>)>, rand: f32) -> Vec<TreeToken> {
    let (probabilities, patterns): (Vec<f32>, Vec<Vec<TreeToken>>) = rules.into_iter().unzip();

    let bounds = to_bounds(probabilities);

    for (bound, rule) in iter::zip(bounds, patterns) {
        if rand < bound {
            return rule;
        }
    }

    vec![]
}

#[derive(Copy, Clone, PartialEq)]
enum TreeToken {
    Leaf,
    Rotate(f32),
    OldWood,
    NewWood,
    Push,
    Pop,
}

impl TurtleToken for TreeToken {
    fn action(token: Self) -> Vec<TurtleAction> {
        use TreeToken as TT;
        use TurtleAction as TA;
        match token {
            TT::Leaf => vec![TA::SetColor(Color::LIME), TA::Move(20.0)],
            TT::NewWood => vec![TA::SetColor(Color::BROWN), TA::Move(20.0)],
            TT::OldWood => vec![TA::SetColor(Color::BLACK), TA::Move(20.0)],
            TT::Rotate(rot) => vec![TA::Rotate(rot)],
            TT::Push => vec![TA::Push],
            TT::Pop => vec![TA::Pop],
        }
    }
}

impl LTokenStochastic for TreeToken {
    #[inline(always)]
    fn apply(token: Self, rand: f32) -> Vec<Self> {
        use TreeToken as TT;
        match token {
            //TODO move `rule` computation to the compile time or precompute
            TT::Leaf => rule(
                vec![
                    (
                        0.7,
                        vec![
                            TT::Push,
                            TT::NewWood,
                            TT::Push,
                            TT::Rotate(PI / 6.0),
                            TT::Leaf,
                            TT::Pop,
                            TT::Rotate(-PI / 6.0),
                            TT::Leaf,
                            TT::Pop,
                        ],
                    ),
                    (
                        0.3,
                        vec![
                            TT::Push,
                            TT::NewWood,
                            TT::Push,
                            TT::Rotate(PI / 6.0),
                            TT::Leaf,
                            TT::Pop,
                            TT::Push,
                            TT::Leaf,
                            TT::Pop,
                            TT::Rotate(-PI / 6.0),
                            TT::Leaf,
                            TT::Pop,
                        ],
                    ),
                ],
                rand,
            ),
            TT::NewWood => vec![TT::OldWood, TT::NewWood],
            _ => vec![token],
        }
    }
}

fn main() {
    use TreeToken as TT;
    let turtle = TurtleDrawer::new(
        vec2![1000.0, 1000.0],
        vec2![0.5, 0.5],
        vec2![500.0, 700.0],
        Color::WHITE,
    );

    let system = LSystemStochastic::from(vec![TT::Leaf]);

    let sequence = system.iterate(10, None);

    let actions = TurtleToken::actions(sequence);

    turtle.run(&actions);
}
