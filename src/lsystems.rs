use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[macro_export]
macro_rules! rules {
    ( $rand:expr; $num:expr; {$($prob:expr),+ $(,)?}; {$($rule:expr),+ $(,)?} $(;)?) => {{
        let bounds: [f32; $num] = $crate::accu::accumulate([$($prob),+]);
        let patterns = vec![$($rule),+];
        if patterns.len() != $num {
            panic!("number of rules {} doesn't match the declared number {}",patterns.len(),$num);
        }
        let mut ret = None;
        for (bound, pattern) in std::iter::zip(bounds.to_vec(), patterns) {
            if $rand < bound {
                ret = Some(pattern);
                break;
            }
        }
        ret.unwrap_or_else(|| panic!("Your random value is bigger than 1. Random value: {}",$rand))
    }};
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
