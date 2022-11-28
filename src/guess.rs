use float_cmp::approx_eq;

use crate::word::Word;
use std::cmp::Ordering;

pub trait Algorithm {
    type TGuess: Ord + Into<Word>;
    fn make_guess(&self, guess: &Word, num_solns: usize, histogram: &[u32]) -> Self::TGuess;
}

#[derive(Debug)]
pub struct EntropyGuess {
    word: Word,
    entropy: f64,
    is_potential_soln: bool,
}

impl EntropyGuess {
    fn new(word: Word, entropy: f64, is_potential_soln: bool) -> EntropyGuess {
        EntropyGuess {
            word,
            entropy,
            is_potential_soln,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if !approx_eq!(f64, self.entropy, other.entropy, epsilon = 1e-9) {
            // Reverse comparison: High entropy is a lower guess
            // so that it can particpate in guesses.min()
            return other.entropy.total_cmp(&self.entropy);
        }

        if self.is_potential_soln != other.is_potential_soln {
            return if self.is_potential_soln {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        Ordering::Equal
    }
}

impl From<EntropyGuess> for Word {
    fn from(item: EntropyGuess) -> Self {
        item.word
    }
}

pub struct EntropyAlgorithm;

impl Algorithm for EntropyAlgorithm {
    type TGuess = EntropyGuess;
    fn make_guess(&self, guess: &Word, num_solns: usize, histogram: &[u32]) -> EntropyGuess {
        let word = guess.clone();
        let is_potential_soln = *histogram.last().unwrap() == 1;

        let entropy: f64 = histogram
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let probability = count as f64 / num_solns as f64;
                probability * probability.log2()
            })
            .sum();

        EntropyGuess::new(word, -entropy, is_potential_soln)
    }
}

// todo add macro for comparisons
impl PartialEq for EntropyGuess {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Equal)
    }
}
impl PartialOrd for EntropyGuess {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
    fn lt(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Less)
    }
    fn le(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Equal | Ordering::Less)
    }
    fn gt(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Greater)
    }
    fn ge(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Equal | Ordering::Greater)
    }
}
impl Eq for EntropyGuess {}
impl Ord for EntropyGuess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(&other)
    }
}

#[derive(Debug)]
pub struct MinimaxGuess {
    word: Word,
    largest_bucket: u32,
    is_potential_soln: bool,
}

impl MinimaxGuess {
    fn new(word: Word, largest_bucket: u32, is_potential_soln: bool) -> MinimaxGuess {
        MinimaxGuess {
            word,
            largest_bucket,
            is_potential_soln,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.largest_bucket != other.largest_bucket {
            return self.largest_bucket.cmp(&other.largest_bucket);
        }

        if self.is_potential_soln != other.is_potential_soln {
            return if self.is_potential_soln {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        Ordering::Equal
    }
}

impl From<MinimaxGuess> for Word {
    fn from(item: MinimaxGuess) -> Self {
        item.word
    }
}

pub struct MinimaxAlgorithm;

impl Algorithm for MinimaxAlgorithm {
    type TGuess = MinimaxGuess;
    fn make_guess(&self, guess: &Word, _num_solns: usize, histogram: &[u32]) -> MinimaxGuess {
        let word = guess.clone();
        let is_potential_soln = *histogram.last().unwrap() == 1;

        let largest_bucket = histogram.iter().copied().max().unwrap_or(0_u32);

        MinimaxGuess::new(word, largest_bucket, is_potential_soln)
    }
}

impl PartialEq for MinimaxGuess {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Equal)
    }
}
impl PartialOrd for MinimaxGuess {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
    fn lt(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Less)
    }
    fn le(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Equal | Ordering::Less)
    }
    fn gt(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Greater)
    }
    fn ge(&self, other: &Self) -> bool {
        matches!(self.cmp(&other), Ordering::Equal | Ordering::Greater)
    }
}
impl Eq for MinimaxGuess {}

impl Ord for MinimaxGuess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(&other)
    }
}
