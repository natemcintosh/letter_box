use itertools::Itertools;

pub struct Board {
    /// An array of 12 characters representing the letters on the board.
    letters: [[char; 3]; 4],
}

impl Board {
    /// Creates a new Board from a string. Assumes the string has exactly 12 characters
    pub fn new(s: &str) -> Self {
        let chars: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(
            chars.len(),
            12,
            "Input string must have exactly 12 characters"
        );

        let mut letters = [[' '; 3]; 4];
        for i in 0..4 {
            for j in 0..3 {
                letters[i][j] = chars[i * 3 + j];
            }
        }

        Board { letters }
    }

    /// Takes in a word and returns an `Option<BoardEncodedWord>`.
    /// It returns a valid `BoardEncodedWord` if all the letters of the word are in the letters of the board,
    /// and each consecutive letter in the word is in a different array in the board.
    pub fn encode_word(&self, word: &str) -> Option<BoardEncodedWord> {
        let mut start = None;
        let mut end = None;
        let mut spots_filled = 0u16;
        let mut previous_row = None;

        for ch in word.chars() {
            let mut found = false;
            for (row_idx, row) in self.letters.iter().enumerate() {
                // If this letter is in this row
                if let Some(col_idx) = row.iter().position(|&c| c == ch) {
                    if let Some(prev_row) = previous_row {
                        if prev_row == row_idx {
                            // Consecutive letters in the same row
                            return None;
                        }
                    }
                    let pos = (row_idx * 3 + col_idx) as u8;

                    if start.is_none() {
                        start = Some(pos);
                    }
                    end = Some(pos);
                    spots_filled |= 1 << pos;
                    previous_row = Some(row_idx);
                    found = true;
                    break;
                }
            }
            if !found {
                // Letter not found on the board
                return None;
            }
        }

        Some(BoardEncodedWord {
            word: word.to_string(),
            start: start?,
            end: end?,
            spots_filled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("AEI", Some(BoardEncodedWord { word: "AEI".to_string(), start: 0, end: 8, spots_filled: 0b000_100_010_001 }))]
    #[case("ICDL", Some(BoardEncodedWord { word: "ICDL".to_string(), start: 8, end: 11, spots_filled: 0b100_100_001_100 }))]
    #[case("AAA", None)]
    #[case("ABC", None)]
    #[case("ABX", None)]
    fn test_encode_word(#[case] word: &str, #[case] expected: Option<BoardEncodedWord>) {
        let board = Board {
            letters: [
                ['A', 'B', 'C'],
                ['D', 'E', 'F'],
                ['G', 'H', 'I'],
                ['J', 'K', 'L'],
            ],
        };

        let result = board.encode_word(word);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("ABCDEFGHIJKL", [
        ['A', 'B', 'C'],
        ['D', 'E', 'F'],
        ['G', 'H', 'I'],
        ['J', 'K', 'L']
    ])]
    #[case("abcdefghijkl", [
        ['a', 'b', 'c'],
        ['d', 'e', 'f'],
        ['g', 'h', 'i'],
        ['j', 'k', 'l']
    ])]
    #[case("123456789012", [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
        ['0', '1', '2']
    ])]
    fn test_board_new_valid(#[case] input: &str, #[case] expected: [[char; 3]; 4]) {
        let board = Board::new(input);
        assert_eq!(board.letters, expected);
    }

    #[rstest]
    #[should_panic(expected = "Input string must have exactly 12 characters")]
    #[case("ABCDEFGHIJK")]
    #[should_panic(expected = "Input string must have exactly 12 characters")]
    #[case("ABCDEFGHIJKLM")]
    #[should_panic(expected = "Input string must have exactly 12 characters")]
    #[case("")]
    fn test_board_new_invalid(#[case] input: &str) {
        Board::new(input);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a word encoded with its position and filled spots on a board.
pub struct BoardEncodedWord {
    /// The original word
    pub word: String,
    /// The starting position of the word on the board.
    start: u8,
    /// The ending position of the word on the board.
    end: u8,
    /// A bitmask representing the spots filled by the word on the board.
    spots_filled: u16,
}

impl BoardEncodedWord {
    /// Displays each bit in the `spots_filled` field for debugging purposes.
    pub fn debug_spots_filled(&self) -> String {
        format!("{:012b}", self.spots_filled)
    }
}

/// Generates all valid permutations of `valid_words` up to `max_words` length.
/// A permutation is considered valid if:
/// 1. Each word's `end` position matches the next word's `start` position.
/// 2. The combined `spots_filled` of the permutation covers all spots on the board
/// # Arguments
///
/// * `valid_words` - A slice of `BoardEncodedWord` representing the valid words.
/// * `max_words` - The maximum number of words in each permutation.
///
/// # Returns
///
/// An iterator over vectors of references to `BoardEncodedWord` representing valid permutations.
pub fn valid_permutations(
    valid_words: &[BoardEncodedWord],
    max_words: usize,
) -> impl Iterator<Item = Vec<&BoardEncodedWord>> {
    valid_words
        .iter()
        .permutations(max_words)
        .filter(|p| p.iter().tuple_windows().all(|(w1, w2)| w1.end == w2.start))
        .filter(|p| p.iter().fold(0, |acc, x| acc | x.spots_filled) == 0b111111111111)
}
