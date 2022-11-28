use colored::Colorize;

use crate::boards::{Scoreboard, ScoreboardRow};
use crate::scoring::{score_to_str, MAX_SCORE};

pub trait Reporter {
    fn print(&self, scoreboard: &Scoreboard);
    fn print_tail(&self, scoreboard: &Scoreboard);
    fn report_failure(&self, scoreboard: &Scoreboard);
}

pub struct NullReporter;

impl Reporter for NullReporter {
    fn print(&self, _scoreboard: &Scoreboard) {}
    fn print_tail(&self, _scoreboard: &Scoreboard) {}
    fn report_failure(&self, _scoreboard: &Scoreboard) {}
}

pub struct ConsoleReporter;

impl ConsoleReporter {
    fn build_header_str() -> String {
        "| # | Soln. | Guess | Score | Poss. |\n|---|-------|-------|-------|-------|".to_string()
    }

    fn build_row_str(row: &ScoreboardRow) -> String {
        let n = row.n;
        let soln = &row.soln;
        let guess = &row.guess;
        let score = row.score;
        let ternary = score_to_str(score);
        let num_left = row.num_left;

        let remaining = if score == MAX_SCORE as u8 {
            " ".into()
        } else {
            format!("{num_left}")
        };

        let guess = ConsoleReporter::prettify(&guess.value(), &ternary);
        let ternary = ConsoleReporter::prettify(&ternary, &ternary);

        format!("| {n} | {soln} | {guess} | {ternary} | {remaining: >5} |")
    }

    fn prettify(string: &str, mask: &str) -> String {
        let mut characters = Vec::new();
        for (c, m) in string.chars().zip(mask.chars()) {
            let colored = match m {
                '0' => c.to_string().normal(),
                '1' => c.to_string().yellow(),
                '2' => c.to_string().green(),
                _ => panic!("Unexpected character in ternary score: '{m}'"),
            };
            characters.push(colored);
        }

        format!(
            "{}{}{}{}{}",
            characters[0], characters[1], characters[2], characters[3], characters[4]
        )
    }
}

impl Reporter for ConsoleReporter {
    fn print(&self, scoreboard: &Scoreboard) {
        let header = ConsoleReporter::build_header_str();
        println!("{header}");

        for row in &scoreboard.rows {
            let row_str = ConsoleReporter::build_row_str(row);
            println!("{row_str}");
        }
    }

    fn print_tail(&self, scoreboard: &Scoreboard) {
        if scoreboard.len() == 0 {
            return;
        }

        if scoreboard.len() == 1 {
            let header = ConsoleReporter::build_header_str();
            println!("{header}");
        }

        let last_row = scoreboard.rows.last().unwrap();
        let row_str = ConsoleReporter::build_row_str(last_row);
        println!("{row_str}");
    }

    fn report_failure(&self, scoreboard: &Scoreboard) {
        let num = scoreboard.len();
        println!("Failed to converge after {num} iterations.");
    }
}
