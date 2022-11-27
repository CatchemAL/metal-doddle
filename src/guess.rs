use float_cmp::approx_eq;

use crate::word::Word;
use std::cmp::Ordering;

pub trait GuessFactory {
    type TGuess: Ord;
    fn create(&self, guess: &Word, num_potential_solns: usize, histogram: &[u32]) -> Self::TGuess;
}

#[derive(Debug)]
pub struct EntropyGuess {
    pub word: Word,
    entropy: f64,
    is_potential_soln: bool,
}

pub struct EntropyGuessFactory;

impl GuessFactory for EntropyGuessFactory {
    type TGuess = EntropyGuess;
    fn create(&self, guess: &Word, num_potential_solns: usize, histogram: &[u32]) -> EntropyGuess {
        let word = guess.clone();
        let is_potential_soln = *histogram.last().unwrap() == 1;

        let entropy: f64 = histogram
            .into_iter()
            .filter_map(|count| {
                if *count > 0 {
                    let probability = *count as f64 / num_potential_solns as f64;
                    let ent = probability * probability.log2();
                    Some(ent)
                } else {
                    None
                }
            })
            .sum();

        EntropyGuess::new(word, -entropy, is_potential_soln)
    }
}

impl EntropyGuess {
    fn new(word: Word, entropy: f64, is_potential_soln: bool) -> EntropyGuess {
        EntropyGuess {
            word,
            entropy,
            is_potential_soln,
        }
    }

    fn total_cmp(&self, other: &Self) -> Ordering {
        if !approx_eq!(f64, self.entropy, other.entropy, epsilon = 1e-9) {
            return self.entropy.total_cmp(&other.entropy).reverse();
        }

        if self.is_potential_soln != other.is_potential_soln {
            return if self.is_potential_soln {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        self.entropy.total_cmp(&other.entropy)
    }
}

impl PartialEq for EntropyGuess {
    fn eq(&self, other: &Self) -> bool {
        self.entropy == other.entropy
    }
}
impl PartialOrd for EntropyGuess {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.total_cmp(&other))
    }
    fn lt(&self, other: &Self) -> bool {
        matches!(self.total_cmp(&other), Ordering::Less)
    }
    fn le(&self, other: &Self) -> bool {
        matches!(self.total_cmp(&other), Ordering::Equal | Ordering::Less)
    }
    fn gt(&self, other: &Self) -> bool {
        matches!(self.total_cmp(&other), Ordering::Greater)
    }
    fn ge(&self, other: &Self) -> bool {
        matches!(self.total_cmp(&other), Ordering::Equal | Ordering::Greater)
    }
}
impl Eq for EntropyGuess {}

impl Ord for EntropyGuess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_cmp(&other)
    }
}
