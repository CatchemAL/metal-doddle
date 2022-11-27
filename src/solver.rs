use crate::guess::GuessFactory;
use crate::scoring;
use crate::word::Word;
use crate::MAX_SCORE;

pub struct Solver<T> {
    guess_factory: T,
}

// impl<T> Solver<T> {
//     pub fn new(guess_factory: T) -> Solver<T> {
//         Solver { guess_factory }
//     }
// }

impl<T: GuessFactory> Solver<T> {
    pub fn new(guess_factory: T) -> Solver<T> {
        Solver { guess_factory }
    }

    pub fn best_guess(&self, all_words: &[Word], potential_solns: &[Word]) -> T::TGuess {
        if potential_solns.len() > 2 {
            return self.all_guesses(all_words, potential_solns).min().unwrap();
        }

        let num_solns = potential_solns.len();
        let guess = &potential_solns[0];

        // Fake a histogram. Anything will do here...
        let mut histogram: Vec<u32> = vec![0; MAX_SCORE + 1];
        histogram[MAX_SCORE] = 1;
        if potential_solns.len() == 2 {
            histogram[0] = 1;
        }

        self.guess_factory.create(guess, num_solns, &histogram)
    }

    fn all_guesses<'a>(
        &'a self,
        all_words: &'a [Word],
        potential_solns: &'a [Word],
    ) -> impl Iterator<Item = T::TGuess> + 'a {
        let mut histogram: Vec<u32> = vec![0; MAX_SCORE + 1];
        all_words.into_iter().map(move |guess| {
            histogram.reset(0);
            for potential_soln in potential_solns {
                let score = scoring::score(guess, potential_soln) as usize;
                histogram[score] += 1;
            }
            let num_solns = potential_solns.len();
            let guess = self.guess_factory.create(guess, num_solns, &histogram);
            guess
        })
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
