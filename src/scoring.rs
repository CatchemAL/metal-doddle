use crate::word::{Word, SIZE};
use radix_fmt::radix_3;

const NUM_INDICATORS: usize = 3;
pub const MAX_SCORE: usize = NUM_INDICATORS.pow(SIZE as u32) - 1;

pub const GREEN: u8 = 2;
pub const AMBER: u8 = 1;
pub const GREY: u8 = 0;

pub fn score_to_str(score: u8) -> String {
    format!("{:0>5}", radix_3(score).to_string())
}

#[allow(dead_code)]
pub fn str_to_score(ternary: &str) -> u8 {
    usize::from_str_radix(ternary, 3).unwrap() as u8
}

#[inline]
pub fn score(guess: &Word, soln: &Word) -> u8 {
    let mut value: u8 = GREY;

    let guess = &guess.vector;
    let soln = &soln.vector;
    let powers: [u8; SIZE] = [81, 27, 9, 3, 1];
    let mut misplaced = [0_u8; 26];

    for (i, (&g, &s)) in guess.iter().zip(soln).enumerate() {
        if g == s {
            value += GREEN * powers[i];
        } else {
            misplaced[s as usize] += 1;
        }
    }

    for (i, (&g, &s)) in guess.iter().zip(soln).enumerate() {
        if g == s {
            continue;
        } else if misplaced[g as usize] > 0 {
            value += AMBER * powers[i];
            misplaced[g as usize] -= 1;
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
        let expected = str_to_score(ternary_score);

        // Act
        let actual = score(&guess, &soln);

        // Assert
        assert_eq!(expected, actual);
    }

    #[rstest]
    fn convert_ternary__both_ways__roundtrips() {
        for i in 0..MAX_SCORE {
            // Arrange
            let expected = i as u8;

            // Act
            let ternary = score_to_str(expected);
            let actual = str_to_score(&ternary);

            // Assert
            assert_eq!(expected, actual);
        }
    }
}
