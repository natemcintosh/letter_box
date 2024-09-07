use clap::Parser;
use itertools::Itertools;
use letter_box::{valid_permutations, Board, BoardEncodedWord};
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use std::{fs, time};
use thousands::Separable;

fn factorial(n: u64) -> BigUint {
    (1..=n).product()
}

#[derive(Parser)]
#[command(
    version = "0.6.0",
    author = "Nathan McIntosh",
    about = "Gives you solutions to the letter boxed puzzle"
)]
struct Cli {
    /// The letters on each side of the box, in quotes and space separated.
    /// E.g. "abc def ghi jkl".
    /// Order of sides does not matter. Order of letters on sides does not matter
    letters: String,

    /// How many words in your solutions. More than 2 could potentially take a while to run.
    #[arg(short, long, default_value = "2")]
    number_of_words: usize,

    /// Path to file of words that should be used
    #[arg(short, long, default_value = "american_english_dictionary.txt")]
    dictionary_file: String,
}

fn main() {
    let start_time = time::Instant::now();

    let cli = Cli::parse();

    let letters = &cli.letters;
    let n_words = cli.number_of_words;
    let board: Board = Board::new(letters);

    let file_path = &cli.dictionary_file;
    let words = fs::read_to_string(file_path).expect("Unable to read file");

    let valid_check_time = time::Instant::now();
    let mut start_words = words
        .lines()
        .filter(|s| !s.contains(char::is_uppercase))
        .filter(|&w| !w.ends_with("'s"))
        .collect_vec();
    start_words.sort_unstable();
    start_words.dedup();

    // Encode the words, and filter out those that don't fit
    let valid_words: Vec<BoardEncodedWord> = start_words
        .iter()
        .filter_map(|w| board.encode_word(w))
        .collect();

    println!(
        "Found {} valid words in {:.3} seconds",
        &valid_words.len(),
        valid_check_time.elapsed().as_secs_f32()
    );

    let permutation_start_time = time::Instant::now();
    valid_permutations(&valid_words, n_words).for_each(|pair| {
        let words: Vec<String> = pair
            .iter()
            .map(|encoded_word| encoded_word.word.clone())
            .collect();
        let joined = words.join(" - ");
        println!("{joined}");
    });

    let n_perms =
        factorial(valid_words.len() as u64) / factorial((valid_words.len() - n_words) as u64);
    let n_perms_u64 = n_perms.to_u64().unwrap_or(u64::MAX);
    let perm_run_time = permutation_start_time.elapsed().as_secs_f64();
    let perms_per_second = (n_perms_u64 as f64 / perm_run_time) as u64;
    println!(
        "Examined {} permutations in {:.3} seconds ({} permutations/second)\n",
        n_perms,
        perm_run_time,
        perms_per_second.separate_with_commas()
    );

    println!(
        "letter_box.rs -- {:.3} seconds",
        start_time.elapsed().as_secs_f32()
    );
}
