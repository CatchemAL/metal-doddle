use crate::dictionary;
use crate::dictionary::Dictionary;
use crate::guess::EntropyAlgorithm;
use crate::guess::MinimaxAlgorithm;
use crate::reporting::NullReporter;
use crate::reporting::{ConsoleReporter, Reporter};
use crate::solver::Solve;
use crate::solver::Solver;
use crate::word::Word;

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum SolverType {
    Entropy,
    Minimax,
}

pub fn get_solver(solver: SolverType) -> Box<dyn Solve> {
    let reporter = get_reporter(true);

    let all_words: Vec<Word> = dictionary::get_all_words();
    let potential_solns: Vec<Word> = dictionary::get_soln_words();
    let dictionary = Dictionary {
        all_words,
        potential_solns,
    };

    match solver {
        SolverType::Entropy => {
            let algorithm = EntropyAlgorithm;
            let solver = Solver::new(algorithm, reporter, dictionary);
            Box::new(solver)
        }
        SolverType::Minimax => {
            let algorithm = MinimaxAlgorithm;
            let solver = Solver::new(algorithm, reporter, dictionary);
            Box::new(solver)
        }
    }
}

pub fn get_reporter(show_progress: bool) -> Box<dyn Reporter> {
    if show_progress {
        Box::new(ConsoleReporter)
    } else {
        Box::new(NullReporter)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use rstest::{fixture, rstest};

    use crate::{
        boards::{Scoreboard, ScoreboardRow},
        scoring::MAX_SCORE,
        word::Word,
    };

    use super::*;

    #[rstest]
    fn get_reporter__for_null__returns(scoreboard: Scoreboard) {
        // Arrange

        // Act
        let reporter = get_reporter(false);

        reporter.report_failure(&scoreboard);
    }

    #[rstest]
    fn get_solver__for_minimax__returns() {
        // Arrange
        let soln: Word = "SNAKE".into();

        // Act
        let solver = get_solver(SolverType::Minimax);
        let scoreboard = solver.solve(&soln, soln.clone());

        // Assert
        assert_eq!(1, scoreboard.unwrap().len());
    }

    #[rstest]
    fn get_solver__for_entropy__returns() {
        // Arrange
        let soln: Word = "SNAKE".into();

        // Act
        let solver = get_solver(SolverType::Entropy);
        let scoreboard = solver.solve(&soln, soln.clone());

        // Assert
        assert_eq!(1, scoreboard.unwrap().len());
    }

    #[fixture]
    fn scoreboard() -> Scoreboard {
        // Arrange
        let mut rows = Vec::new();

        let row1 = ScoreboardRow {
            n: 1,
            soln: "SNAKE".into(),
            guess: "SOARE".into(),
            score: 42,
            num_left: 123,
        };

        let row2 = ScoreboardRow {
            n: 1,
            soln: "SNAKE".into(),
            guess: "CLINT".into(),
            score: 142,
            num_left: 3,
        };

        let row3 = ScoreboardRow {
            n: 1,
            soln: "SNAKE".into(),
            guess: "SNAKE".into(),
            score: MAX_SCORE as u8,
            num_left: 1,
        };

        rows.push(row1);
        rows.push(row2);
        rows.push(row3);

        Scoreboard { rows }
    }
}
