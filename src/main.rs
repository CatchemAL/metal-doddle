use crate::guess::EntropyGuessFactory;
use crate::scoring::MAX_SCORE;
use crate::solver::Solver;
use crate::word::Word;

mod dictionary;
mod guess;
mod scoring;
mod solver;
mod word;

fn main() {
    let soln = Word::new("TOWER");
    let mut guess = Word::new("SOARE");

    println!("Loading dictionaries...");
    let all_words: Vec<Word> = dictionary::get_all_words();
    let mut potential_solns: Vec<Word> = dictionary::get_soln_words();

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

    println!("Failed to converge after {MAX_ITERS} iterations.")
}

fn trim_potential_solns(guess: &Word, observed_score: u8, potential_solns: &[Word]) -> Vec<Word> {
    potential_solns
        .iter()
        .filter(|soln| scoring::score(guess, soln) == observed_score)
        .map(|x| x.clone())
        .collect()
}
