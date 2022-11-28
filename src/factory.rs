use crate::guess::EntropyAlgorithm;
use crate::guess::MinimaxAlgorithm;
use crate::reporting::NullReporter;
use crate::reporting::{ConsoleReporter, Reporter};
use crate::solver::Solve;
use crate::solver::Solver;

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum SolverType {
    Entropy,
    Minimax,
}

pub fn get_solver(solver: SolverType) -> Box<dyn Solve> {
    let reporter = get_reporter(true);

    match solver {
        SolverType::Entropy => {
            let guess_factory = EntropyAlgorithm;
            let solver = Solver::new(guess_factory, reporter);
            Box::new(solver)
        }
        SolverType::Minimax => {
            let guess_factory = MinimaxAlgorithm;
            let solver = Solver::new(guess_factory, reporter);
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
