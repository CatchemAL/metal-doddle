use colored::Colorize;
use itertools::Itertools;

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

    fn report(scoreboard: &Scoreboard) -> String {
        let mut rows = Vec::new();
        let header = ConsoleReporter::build_header_str();
        rows.push(header);

        for row in &scoreboard.rows {
            let row_str = ConsoleReporter::build_row_str(row);
            rows.push(row_str);
        }

        rows.iter().join("\n")
    }

    fn report_tail(scoreboard: &Scoreboard) -> String {
        let last_row = scoreboard.rows.last().unwrap();
        let row_str = ConsoleReporter::build_row_str(last_row);

        if scoreboard.len() <= 1 {
            let header = ConsoleReporter::build_header_str();
            [header, row_str].join("\n")
        } else {
            row_str
        }
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
        let result = ConsoleReporter::report(scoreboard);
        println!("{result}");
    }

    fn print_tail(&self, scoreboard: &Scoreboard) {
        let result = ConsoleReporter::report_tail(scoreboard);
        println!("{result}");
    }

    fn report_failure(&self, scoreboard: &Scoreboard) {
        let num = scoreboard.len();
        println!("Failed to converge after {num} iterations.");
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    fn report_failure__with_scoreboard__prints(scoreboard: Scoreboard) {
        // Arrange
        let sut = ConsoleReporter;

        // Act
        sut.report_failure(&scoreboard);
    }

    #[rstest]
    fn print_tail__with_scoreboard__prints(scoreboard: Scoreboard) {
        // Act
        let actual = ConsoleReporter::report_tail(&scoreboard);

        // Assert
        assert!(actual.len() > 10);
    }

    #[rstest]
    fn print_scoreboard__with_scoreboard__reports(scoreboard: Scoreboard) {
        // Act
        let actual = ConsoleReporter::report(&scoreboard);

        // Assert
        assert!(actual.len() > 10);
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
