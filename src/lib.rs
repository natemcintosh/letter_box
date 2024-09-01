use itertools::Itertools;

pub struct Board {
    /// An array of 16 characters representing the letters on the board.
    letters: [[char; 4]; 4],
}

impl Board {
    /// Takes in a word and returns an `Option<BoardEncodedWord>`.
    /// It returns a valid BoardEncodedWord if all the letters of the word are in the letters of the board,
    /// and each consecutive letter in the word is in a different array in the board.
    pub fn encode_word(&self, word: &str) -> Option<BoardEncodedWord> {
        let mut start = None;
        let mut end = None;
        let mut spots_filled = 0u16;
        let mut previous_row = None;

        for ch in word.chars() {
            let mut found = false;
            for (row_idx, row) in self.letters.iter().enumerate() {
                if row.contains(&ch) {
                    if let Some(prev_row) = previous_row {
                        if prev_row == row_idx {
                            return None; // Consecutive letters in the same row
                        }
                    }
                    let col_idx = row.iter().position(|&c| c == ch).unwrap();
                    let pos = (row_idx * 4 + col_idx) as u16;

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
                return None; // Letter not found on the board
            }
        }

        Some(BoardEncodedWord {
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
    #[case("ABCD", Some(BoardEncodedWord { start: 0, end: 12, spots_filled: 0b0000000100000001 }))]
    #[case("AAAA", None)]
    #[case("WXYZ", Some(BoardEncodedWord { start: 3, end: 15, spots_filled: 0b1000000000001000 }))]
    #[case("ABCA", None)]
    #[case("ABX", None)]
    fn test_encode_word(#[case] word: &str, #[case] expected: Option<BoardEncodedWord>) {
        let board = Board {
            letters: [
                ['A', 'B', 'C', 'D'],
                ['E', 'F', 'G', 'H'],
                ['I', 'J', 'K', 'L'],
                ['M', 'N', 'O', 'P'],
            ],
        };

        let result = board.encode_word(word);
        assert_eq!(result, expected);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents a word encoded with its position and filled spots on a board.
pub struct BoardEncodedWord {
    /// The starting position of the word on the board.
    start: u16,
    /// The ending position of the word on the board.
    end: u16,
    /// A bitmask representing the spots filled by the word on the board.
    spots_filled: u16,
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
pub fn valid_permutations<'a>(
    valid_words: &[BoardEncodedWord],
    max_words: usize,
) -> impl Iterator<Item = Vec<&BoardEncodedWord>> {
    valid_words
        .iter()
        .permutations(max_words)
        .filter(|p| p.iter().tuple_windows().all(|(w1, w2)| w1.end == w2.start))
        .filter(|p| p.iter().fold(0, |acc, x| acc | x.spots_filled) == u16::MAX)
}
