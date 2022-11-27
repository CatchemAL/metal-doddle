use crate::word::{Word, SIZE};

const NUM_INDICATORS: usize = 3;
pub const MAX_SCORE: usize = NUM_INDICATORS.pow(SIZE) - 1;

pub const GREEN: u8 = 2;
pub const AMBER: u8 = 1;
pub const GREY: u8 = 0;

pub fn score(guess: &Word, soln: &Word) -> u8 {
    let mut value: u8 = GREY;

    let guess = &guess.vector;
    let soln = &soln.vector;
    let powers: [u8; SIZE] = [81, 27, 9, 3, 1];

    for i in 0..SIZE {
        let letter = guess[i];
        if letter == soln[i] {
            value += GREEN * powers[i];
        } else {
            // We need to determine if partial match
            let mut num_times_in_soln: u8 = 0;
            for j in 0..SIZE {
                if letter == soln[j] && guess[j] != soln[j] {
                    num_times_in_soln += 1;
                }
            }

            if num_times_in_soln == 0 {
                continue;
            }

            let mut num_times_already_seen: u8 = 0;
            for j in 0..i {
                if letter == guess[j] && guess[j] != soln[j] {
                    num_times_already_seen += 1;
                }
            }

            let is_partial_match = num_times_already_seen < num_times_in_soln;
            if is_partial_match {
                value += AMBER * powers[i];
            }
        }
    }

    value
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::super::word::Word;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("SPEAK", "SPEAR", "22220")]
    #[case("SPEAK", "SPEAR", "22220")]
    #[case("STRIP", "SPEAR", "20101")]
    #[case("SOLID", "SPEAR", "20000")]
    #[case("MAGIC", "SPEAR", "01000")]
    #[case("VAPID", "SPEAR", "01100")]
    #[case("STERN", "SPEAR", "20210")]
    #[case("SPEAR", "SPEAR", "22222")]
    #[case("RAISE", "PERKY", "10001")]
    #[case("HERON", "PERKY", "02200")]
    #[case("PULLY", "PERKY", "20002")]
    #[case("PERRY", "PERKY", "22202")]
    #[case("PERKY", "PERKY", "22222")]
    #[case("CHART", "PEACH", "11200")]
    #[case("PEACH", "PEACH", "22222")]
    #[case("SPARE", "BASIC", "10100")]
    #[case("CLOUT", "BASIC", "10000")]
    #[case("FANGS", "BASIC", "02001")]
    #[case("DINKY", "BASIC", "01000")]
    #[case("MAGIC", "BASIC", "02022")]
    #[case("BASIC", "BASIC", "22222")]
    #[case("APPLE", "CRIMP", "01000")]
    #[case("SALAD", "AGATE", "01010")]
    #[case("ABACA", "AGATE", "20200")]
    #[case("BANAL", "AGATE", "01010")]
    #[case("AGATE", "AGATE", "22222")]
    #[case("MUMMY", "GAMMA", "00220")]
    #[case("MIMIC", "GAMMA", "10200")]
    #[case("MAGIC", "GAMMA", "12100")]
    #[case("HAIRY", "GAMMA", "02000")]
    #[case("FUNDS", "GAMMA", "00000")]
    #[case("GAMMA", "GAMMA", "22222")]
    #[case("ERROR", "ARGUE", "12000")]
    #[case("TEARS", "ARGUE", "01110")]
    #[case("GRAPE", "ARGUE", "12102")]
    #[case("AGREE", "ARGUE", "21102")]
    #[case("ARGUE", "ARGUE", "22222")]
    fn score_word__all_cases__scores_correctly(
        #[case] guess: &str,
        #[case] soln: &str,
        #[case] ternary_score: &str,
    ) {
        // Arrange
        let guess = Word::new(guess);
        let soln = Word::new(soln);
        let expected = isize::from_str_radix(ternary_score, 3).unwrap();

        // Act
        let actual = score(&guess, &soln) as isize;

        // Assert
        assert_eq!(expected, actual);
    }
}
