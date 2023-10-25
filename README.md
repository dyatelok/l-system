# l-system

This is [L-system](https://en.wikipedia.org/wiki/L-system) implementation done using rust.

My goal was to make it as easy as posslible to write L-systems for the programmers using rust features like match expressions.

To make an L-system you should only implement two traits for for your token. Here's an example for the dragon curve:

```
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
```

You'll get the following picture by runnung a `dragon` example by `cargo r --release --example dragon`

![image](https://github.com/dyatelok/l-system/assets/92210438/268342de-053e-4dd7-aea0-c49914cfb5e3)

It's also possible to implement a stochastic system:

![image](https://github.com/dyatelok/l-system/assets/92210438/8a634038-d856-4817-b321-22fe8e7cbb4a)
![image](https://github.com/dyatelok/l-system/assets/92210438/d966e48d-3ef0-4972-8338-b6e02b606797)

See 'tree' example. Other examples are:

`fib_dragon`: ![image](https://github.com/dyatelok/l-system/assets/92210438/405621b2-8a25-4e61-af2a-0bdf542c249c)

`hilbert`: ![image](https://github.com/dyatelok/l-system/assets/92210438/95dedf62-69be-4e8d-b659-a2d2161a0f55)
