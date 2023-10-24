use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

mod rules_macro {
    #[macro_export]
    macro_rules! rules_helper {
        () => {{(vec![1.0],vec![])}};
        ( $rule:expr ) => {{
            (vec![1.0],vec![$rule])
        }};
        ( $( $rule:expr ),+ $(,)? ) => {{
            let mut bounds: Vec<f32> = Vec::new();
            let mut rules = Vec::new();
            let mut summ: f32 = 0.0;
            $(
                let (prob,rule) = $rule;
                summ += prob;
                bounds.push(summ);
                rules.push(rule);
            )*
            if summ != 1.0 {
                panic!("probabilies must add to 1.0, but the summ is {}",summ);
            }
            (bounds,rules)
        }};
    }

    #[macro_export]
    macro_rules! rules {
        ( $rand:expr; $($rule:expr),+ $(,)?) => {{
            let bounds_patterns = $crate::rules_helper!($($rule),+);
            let (bounds, patterns) = bounds_patterns;
            let mut ret = None;
            for (bound, pattern) in std::iter::zip(bounds, patterns) {
                if $rand < bound {
                    ret = Some(pattern);
                    break;
                }
            }
            ret.unwrap_or_else(|| panic!("Your random value is bigger than 1. Random value: {}",$rand))
        }};
    }
}

pub trait LTokenStochastic: Sized {
    fn apply(token: Self, rand: f32) -> Vec<Self>;
}

pub struct LSystemStochastic<T: LTokenStochastic + Clone> {
    axiom: Vec<T>,
}

impl<T: LTokenStochastic + Clone> LSystemStochastic<T> {
    pub fn from(axiom: Vec<T>) -> Self {
        Self { axiom }
    }
}

impl<T: LTokenStochastic + Clone> LSystemStochastic<T> {
    fn apply_once(tokens: &mut Vec<T>, rng: &mut ChaCha8Rng) {
        *tokens = Self::apply_once_helper(std::mem::take(tokens), rng);
    }
    fn apply_once_helper(tokens: Vec<T>, rng: &mut ChaCha8Rng) -> Vec<T> {
        tokens
            .into_iter()
            .flat_map(|a| LTokenStochastic::apply(a, rng.gen()))
            .collect()
    }
    pub fn iterate(&self, n: usize, seed: Option<u64>) -> Vec<T> {
        let mut tokens = self.axiom.clone();
        let mut rng = ChaCha8Rng::seed_from_u64(seed.unwrap_or(random()));
        for _ in 0..n {
            Self::apply_once(&mut tokens, &mut rng);
        }
        tokens
    }
}

pub trait LToken: Sized {
    fn apply(token: Self) -> Vec<Self>;
}

pub struct LSystem<T: LToken + Clone> {
    axiom: Vec<T>,
}

impl<T: LToken + Clone> LSystem<T> {
    pub fn from(axiom: Vec<T>) -> Self {
        Self { axiom }
    }
    fn apply_once(tokens: &mut Vec<T>) {
        *tokens = Self::apply_once_helper(std::mem::take(tokens));
    }
    fn apply_once_helper(tokens: Vec<T>) -> Vec<T> {
        tokens.into_iter().flat_map(|a| LToken::apply(a)).collect()
    }
    pub fn iterate(&self, depth: usize) -> Vec<T> {
        let mut tokens = self.axiom.clone();
        for _ in 0..depth {
            Self::apply_once(&mut tokens);
        }
        tokens
    }
}
