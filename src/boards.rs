use crate::{scoring::MAX_SCORE, word::Word};

#[derive(Default)]
pub struct Scoreboard {
    pub rows: Vec<ScoreboardRow>,
}

impl Scoreboard {
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_solved(&self) -> bool {
        match self.rows.last() {
            Some(row) => row.score == MAX_SCORE as u8,
            None => false,
        }
    }

    pub fn add_row(&mut self, soln: Word, guess: Word, score: u8, num_left: usize) {
        let row = ScoreboardRow {
            n: self.len() as u32 + 1_u32,
            soln,
            guess,
            score,
            num_left,
        };

        self.rows.push(row);
    }
}

#[derive(Debug)]
pub struct ScoreboardRow {
    pub n: u32,
    pub soln: Word,
    pub guess: Word,
    pub score: u8,
    pub num_left: usize,
}
