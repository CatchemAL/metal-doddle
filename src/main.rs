use crate::guess::EntropyGuessFactory;
use crate::scoring::MAX_SCORE;
use crate::solver::Solver;
use crate::word::Word;
use serde_json::Value;
use std::fs;

mod guess;
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

    println!("Begin solve for solution {soln}...");
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
