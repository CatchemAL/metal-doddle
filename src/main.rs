use crate::scoring::MAX_SCORE;
use crate::word::{Word, SIZE};
use std::cmp::Ordering;

mod scoring;
mod word;

fn main() {
    let guess = Word::new("RAISE");
    let soln = Word::new("TOWER");

    let all_words: Vec<Word> = todo!();
    let potential_solns: Vec<Word> = todo!();

    let guess_factory = EntropyGuessFactory;
    let solver = Solver {
        guess_factory: guess_factory,
    };

    for _i in 0..20 {
        let guess: EntropyGuess = solver.best_guess(&all_words, &potential_solns);
    }

    let score = scoring::score(&guess, &soln);

    println!("Score for guess {guess} given solution {soln} is {score}.")
}

pub struct EntropyGuess {
    word: Word,
    entropy: f64,
    is_potential_soln: bool,
}

pub trait GuessFactory {
    type TGuess: Ord;
    fn create(&self, histogram: &Vec<u32>) -> Self::TGuess;
}

pub struct EntropyGuessFactory;

impl GuessFactory for EntropyGuessFactory {
    type TGuess = EntropyGuess;
    fn create(&self, histogram: &Vec<u32>) -> EntropyGuess {
        let word = Word::new("histogram");
        let entropy = 42.0;
        let is_potential_soln = true;

        EntropyGuess::new(word, entropy, is_potential_soln)
    }
}

pub struct Solver<T> {
    guess_factory: T,
}

impl<T: GuessFactory> Solver<T> {
    fn best_guess(&self, all_words: &Vec<Word>, potential_solns: &Vec<Word>) -> T::TGuess {
        let mut histogram: Vec<u32> = vec![0; MAX_SCORE];

        all_words
            .into_iter()
            .map(|guess| {
                histogram.reset(0);
                for potential_soln in potential_solns {
                    let score = scoring::score(guess, potential_soln) as usize;
                    histogram[score] += 1;
                }
                let guess = self.guess_factory.create(&histogram);
                guess
            })
            .min()
            .unwrap()
    }
}

pub trait ResetExt {
    type Element: Copy;
    fn reset(&mut self, value: Self::Element);
}

impl<T: Copy> ResetExt for [T] {
    type Element = T;

    fn reset(&mut self, value: T) {
        for item in self {
            *item = value;
        }
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
        self.entropy.total_cmp(&other.entropy)
    }
}
