use crate::scoring::MAX_SCORE;
use crate::solver::Solver;
use crate::word::Word;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs;

mod scoring;
mod solver;
mod word;

fn get_all_words() -> Vec<Word> {
    let path = "./dictionaries/dictionary-full-official.json";
    get_words(path)
}

fn get_soln_words() -> Vec<Word> {
    let path = "./dictionaries/dictionary-answers-official.json";
    get_words(path)
}

fn get_words(path: &str) -> Vec<Word> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json = serde_json::from_str(&data).expect("JSON was not well-formatted");

    if let Value::Array(vector) = json {
        return vector
            .into_iter()
            .map(|value| match value {
                Value::String(word) => Word::new(&word),
                _ => panic!("JSON element was not a valid string"),
            })
            .collect();
    }

    panic!("JSON was not a valid vector")
}

fn main() {
    let soln = Word::new("TOWER");
    let mut guess = Word::new("SOARE");

    println!("Loading dictionaries...");
    let all_words: Vec<Word> = get_all_words();
    let mut potential_solns: Vec<Word> = get_soln_words();

    let guess_factory = EntropyGuessFactory;
    let solver = Solver::new(guess_factory);

    println!("Begin solve...");
    const MAX_ITERS: i32 = 20;
    for _i in 0..MAX_ITERS {
        let observed_score = scoring::score(&guess, &soln);
        println!("Playing guess {guess} (Score = {observed_score})");
        if observed_score as usize == MAX_SCORE {
            println!("Solved!");
            return;
        }

        potential_solns = trim_potential_solns(&guess, observed_score, &potential_solns);
        guess = solver.best_guess(&all_words, &potential_solns).word;
    }

    let score = scoring::score(&guess, &soln);

    println!("Score for guess {guess} given solution {soln} is {score}.")
}

fn trim_potential_solns(guess: &Word, observed_score: u8, potential_solns: &[Word]) -> Vec<Word> {
    potential_solns
        .iter()
        .filter(|soln| scoring::score(guess, soln) == observed_score)
        .map(|x| x.clone())
        .collect()
}

#[derive(Debug)]
pub struct EntropyGuess {
    word: Word,
    entropy: f64,
    is_potential_soln: bool,
}

pub trait GuessFactory {
    type TGuess: Ord;
    fn create(&self, guess: &Word, num_potential_solns: usize, histogram: &[u32]) -> Self::TGuess;
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
