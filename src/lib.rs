use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
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
