fn main() {}

// use euler::*;
// use l_system::{Execute, LToken, Runner, Turtle, TurtleAction};
// use std::f32::consts::PI;

// #[derive(Copy, Clone, Eq, PartialEq)]
// enum DragonToken {
//     Forward,
//     RotateLeft,
//     RotateRight,
//     X,
//     Y,
// }

// impl LToken for DragonToken {
//     fn apply(token: &Self) -> &[Self] {
//         use DragonToken as DT;
//         match *token {
//             DT::Forward => &[DT::Forward],
//             DT::RotateLeft => &[DT::RotateLeft],
//             DT::RotateRight => &[DT::RotateRight],
//             DT::X => &[DT::X, DT::RotateLeft, DT::Y, DT::Forward],
//             DT::Y => &[DT::Forward, DT::X, DT::RotateRight, DT::Y],
//         }
//     }
// }

// impl Execute for DragonToken {
//     fn act(t: Self) -> Option<TurtleAction> {
//         use DragonToken as DT;
//         use TurtleAction as TA;
//         match t {
//             DT::Forward => Some(TA::Move(10.0)),
//             DT::RotateLeft => Some(TA::Rotate(PI / 2.0)),
//             DT::RotateRight => Some(TA::Rotate(-PI / 2.0)),
//             _ => None,
//         }
//     }
// }

// fn main() {
//     use DragonToken as DT;
//     let turtle = Turtle::new(vec2![1000.0, 1000.0], vec2![1.0, 1.0], vec2![500.0, 500.0]);

//     let runner = Runner::from(vec![
//         DT::Forward,
//         DT::X,
//         DT::RotateLeft,
//         DT::Forward,
//         DT::RotateRight,
//     ]);

//     let sequence = runner.iterate(10);

//     let actions = sequence
//         .into_iter()
//         .map(Execute::act)
//         .filter_map(|a| if let Some(b) = a { Some(b) } else { None })
//         .collect();

//     turtle.run(actions);
// }
