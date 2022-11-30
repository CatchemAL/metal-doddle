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

impl From<String> for Word {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        Self::new(value)
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
    fn display__via_format__displays() {
        // Arrange
        let word = Word::new("space");
        let expected = "Word is: SPACE".to_string();

        // Act
        let actual = format!("Word is: {word}");

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn debug__via_format__debugs() {
        // Arrange
        let word = Word::new("space");
        let expected = "Word { vector: \"SPACE\" }".to_string();

        // Act
        let actual = format!("{word:?}");

        // Assert
        assert_eq!(expected, actual);
    }
}
