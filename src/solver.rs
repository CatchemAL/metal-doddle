use crate::boards::Scoreboard;
use crate::dictionary::Dictionary;
use crate::guess::Algorithm;
use crate::reporting::Reporter;
use crate::scoring;
use crate::scoring::MAX_SCORE;
use crate::word::Word;

pub trait Solve {
    fn solve(&self, soln: &Word, opening_guess: Word) -> Option<Scoreboard>;
}

pub struct Solver<T> {
    algorithm: T,
    reporter: Box<dyn Reporter>,
    dictionary: Dictionary,
}

impl<T: Algorithm> Solver<T> {
    pub fn new(algorithm: T, reporter: Box<dyn Reporter>, dictionary: Dictionary) -> Solver<T> {
        Solver {
            algorithm,
            reporter,
            dictionary,
        }
    }

    pub fn run(&self, soln: &Word, opening_guess: Word) -> Option<Scoreboard> {
        println!("Loading dictionaries...");
        let &Dictionary {
            ref all_words,
            ref potential_solns,
        } = &self.dictionary;

        let mut potential_solns: Vec<Word> = potential_solns.to_vec();

        println!("Begin solve for solution {soln}...\n");
        use std::time::Instant;
        let now = Instant::now();
        let mut guess = opening_guess;
        let mut scoreboard: Scoreboard = Default::default();

        const MAX_ITERS: i32 = 20;
        for _i in 0..MAX_ITERS {
            let observed_score = scoring::score(&guess, soln);
            potential_solns = self.trim_solns(&guess, observed_score, &potential_solns);
            scoreboard.add_row(soln.clone(), guess, observed_score, potential_solns.len());
            self.reporter.print_tail(&scoreboard);

            if scoreboard.is_solved() {
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}\n", elapsed);
                return Some(scoreboard);
            }

            guess = self.best_guess(all_words, &potential_solns).into();
        }

        self.reporter.report_failure(&scoreboard);
        None
    }

    pub fn best_guess(&self, all_words: &[Word], potential_solns: &[Word]) -> T::TGuess {
        if potential_solns.len() > 2 {
            return self.all_guesses(all_words, potential_solns).min().unwrap();
        }

        let num_solns = potential_solns.len();
        let guess = &potential_solns[0];

        // Fake a histogram. Anything will do here...
        let mut histogram = [0_u32; MAX_SCORE + 1];
        histogram[MAX_SCORE] = 1;
        if potential_solns.len() == 2 {
            histogram[0] = 1;
        }

        self.algorithm.make_guess(guess, num_solns, &histogram)
    }

    fn all_guesses<'a>(
        &'a self,
        all_words: &'a [Word],
        potential_solns: &'a [Word],
    ) -> impl Iterator<Item = T::TGuess> + 'a {
        all_words.iter().map(move |guess| {
            let mut histogram = [0_u32; MAX_SCORE + 1];
            for potential_soln in potential_solns {
                let score = scoring::score(guess, potential_soln) as usize;
                histogram[score] += 1;
            }
            let num_solns = potential_solns.len();
            self.algorithm.make_guess(guess, num_solns, &histogram)
        })
    }

    fn trim_solns(&self, guess: &Word, observed_score: u8, potential_solns: &[Word]) -> Vec<Word> {
        potential_solns
            .iter()
            .filter(|soln| scoring::score(guess, soln) == observed_score)
            .cloned()
            .collect()
    }
}

impl<T: Algorithm> Solve for Solver<T> {
    fn solve(&self, soln: &Word, opening_guess: Word) -> Option<Scoreboard> {
        self.run(soln, opening_guess)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use crate::{guess::MinimaxAlgorithm, reporting::NullReporter};

    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    fn report_failure__with_scoreboard__prints(dictionary: Dictionary) {
        // Arrange
        let algorithm = MinimaxAlgorithm;
        let reporter = Box::new(NullReporter);
        let sut = Solver::new(algorithm, reporter, dictionary);
        let opening_guess: Word = "SALET".into();
        let soln: Word = "POWER".into();

        // Act
        let actual = sut.solve(&soln, opening_guess);

        // Assert
        assert!(matches!(actual, Some(scoreboard) if scoreboard.len() == 3));
    }

    #[fixture]
    fn dictionary() -> Dictionary {
        let all_words = vec![
            "SALET".into(),
            "TOWER".into(),
            "SOARE".into(),
            "ROWER".into(),
            "POWER".into(),
        ];

        let potential_solns = vec!["TOWER".into(), "ROWER".into(), "POWER".into()];

        Dictionary {
            all_words,
            potential_solns,
        }
    }
}
