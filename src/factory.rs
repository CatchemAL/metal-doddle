use crate::guess::Algorithm;
use crate::guess::EntropyAlgorithm;
use crate::reporting::NullReporter;
use crate::reporting::{ConsoleReporter, Reporter};
use crate::solver::Solver;

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum SolverType {
    Entropy,
    Minimax,
}

pub fn get_solver(solver: SolverType) -> Solver<impl Algorithm> {
    let reporter = get_reporter(true);

    match solver {
        SolverType::Entropy => {
            let guess_factory = EntropyAlgorithm;
            Solver::new(guess_factory, reporter)
        }
        SolverType::Minimax => {
            let guess_factory = EntropyAlgorithm;
            Solver::new(guess_factory, reporter)
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
