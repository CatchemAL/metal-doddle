use crate::factory::SolverType;
use crate::word::Word;
use clap::Parser;

mod boards;
mod dictionary;
mod factory;
mod guess;
mod reporting;
mod scoring;
mod solver;
mod word;

fn main() {
    let args = Args::parse();
    let soln = Word::new(&args.answer);
    let guess = Word::new(&args.guess);

    let solver = factory::get_solver(args.solver);
    solver.solve(&soln, guess);
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    answer: String,

    /// Number of times to greet
    #[arg(short, long, default_value = "SALET")]
    guess: String,

    #[arg(short, long, default_value = "entropy", ignore_case = true)]
    solver: SolverType,
}
