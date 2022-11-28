use std::fmt::{Debug, Display, Formatter, Result};

pub const SIZE: usize = 5;

#[derive(Clone, PartialEq, Eq)]
pub struct Word {
    pub vector: [u8; 5],
}

impl Word {
    pub fn new(value: &str) -> Word {
        assert_eq!(SIZE, value.len());
        let upper = value.to_uppercase();
        let mut vector: [u8; SIZE] = [0; SIZE];

        for (i, char) in upper.char_indices() {
            let char_ordinal = char as u8;
            let ordinal = char_ordinal - b'A';
            vector[i] = ordinal;
        }

        Word { vector }
    }

    pub fn value(&self) -> String {
        let values = self.vector.into_iter().map(|e| e + b'A').collect();
        String::from_utf8(values).unwrap()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Word")
            .field("vector", &self.value())
            .finish()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;

    #[test]
    fn test_minimum__for_ints__returns() {
        // Arrange
        let integers: Vec<i32> = vec![15, 22, 73, 4, 5];

        // Act
        let minimum = integers.into_iter().min().unwrap();

        // Assert
        assert_eq!(4, minimum);
    }

    #[test]
    fn display__via_format__displays() {
        // Arrange
        let word = Word::new("space");
        let expected = "Word is: SPACE".to_string();

        // Act
        let actual = format!("Word is: {}", word);

        // Assert
        assert_eq!(expected, actual);
    }
}
