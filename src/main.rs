use clap::{App, Arg};
use itertools::Itertools;
use num_bigint::BigUint;
use std::{assert, char, collections::HashSet, fs, time};

fn valid_permutations<'a>(
    valid_words: &'a [&str],
    sides: &[[std::primitive::char; 3]; 4],
    max_words: usize,
) -> impl Iterator<Item = Vec<&'a &'a str>> {
    let all_letters: HashSet<_> = sides.iter().flatten().copied().collect();

    valid_words
        .iter()
        .permutations(max_words)
        .filter(|p| {
            p.iter()
                .tuple_windows()
                .all(|(w1, w2)| words_can_join(w1, w2))
        })
        .filter(move |p| all_letters.is_subset(&get_unique_chars(p)))
}

fn get_unique_chars(words: &[&&str]) -> HashSet<char> {
    words
        .iter()
        .flat_map(|&&w| w.chars())
        .collect::<HashSet<_>>()
}

fn word_is_valid(word: &str, sides: &[[std::primitive::char; 3]; 4]) -> bool {
    let mut last_used_side: usize = 0;
    for l in word.chars() {
        match sides
            .iter()
            .enumerate()
            .filter(|(side_num, _side)| side_num != &last_used_side)
            .filter(|(_side_num, &side)| side.contains(&l))
            .map(|(side_num, _)| side_num)
            .last()
        {
            Some(n) => {
                last_used_side = n;
            }
            None => return false,
        }
    }
    true
}

fn words_can_join(w1: &str, w2: &str) -> bool {
    let end_of_first = w1.chars().nth_back(0).expect("Could not get last char");
    let start_of_second = w2.chars().next().expect("Could not get first char");
    end_of_first == start_of_second
}

fn create_sides(letters: &str) -> [[std::primitive::char; 3]; 4] {
    assert!(
        letters.len() >= 12,
        "Did not hand in a long enough string of letters"
    );

    // (1..)
    //     .zip(
    //         letters
    //             .split_whitespace()
    //             .map(|p| p.chars().collect::<HashSet<_>>()),
    //     )
    //     .collect::<HashMap<_, _>>()

    // letters.split_whitespace().map(|p| p.chars.collect::<>())

    let mut res = [[' '; 3]; 4];
    for (side_num, side) in letters.split_whitespace().enumerate() {
        for (side_idx, c) in side.chars().enumerate() {
            res[side_num][side_idx] = c;
        }
    }
    res
}

fn factorial(n: u64) -> BigUint {
    (1..=n).product()
}

fn main() {
    let start_time = time::Instant::now();

    let matches = App::new("letter_box")
        .version("0.4.0")
        .author("Nathan McIntosh")
        .about("Gives you solutions to the letter boxed puzzle")
        .arg(Arg::with_name("letters").help(
            "The letters on each side of the box, in quotes and space separated. 
E.g. \"abc def ghi jkl\". 
Order of sides does not matter. Order of letters on sides does not matter",
        ))
        .arg(
            Arg::with_name("n")
                .short("n")
                .long("number_of_words")
                .help(
                    "How many words in your solutions. More than 2 could 
potentially take a while to run.",
                )
                .required(false)
                .default_value("2"),
        )
        .arg(
            Arg::with_name("dictionary_file")
                .short("d_file")
                .long("dictionary_file")
                .help("Path to file of words that should be used")
                .required(false)
                .default_value("american_english_dictionary.txt"),
        )
        .get_matches();

    let letters = matches.value_of("letters").expect("Could not read letters");
    let n_words: usize = matches
        .value_of("n")
        .expect("Could not get number of words")
        .parse()
        .expect("Could not parse into a number");
    let sides = create_sides(letters);

    let read_time = time::Instant::now();
    let file_path = matches
        .value_of("dictionary_file")
        .expect("Did not properly get the name of the dictionary file");
    let words = fs::read_to_string(file_path)
        .expect("Unable to read file")
        .to_lowercase();
    println!(
        "Reading file took {:.3} seconds",
        read_time.elapsed().as_secs_f32()
    );

    let valid_check_time = time::Instant::now();
    let mut valid_words = words
        .lines()
        .filter(|&w| !w.ends_with("'s"))
        .filter(|w| word_is_valid(w, &sides))
        .collect_vec();
    valid_words.sort_unstable();
    valid_words.dedup();

    println!(
        "Found {} valid words in {:.3} seconds",
        &valid_words.len(),
        valid_check_time.elapsed().as_secs_f32()
    );

    let permutation_start_time = time::Instant::now();
    valid_permutations(&valid_words, &sides, n_words).for_each(|pair| {
        let joined = pair.into_iter().join(" - ");
        println!("{}", joined);
    });

    let n_perms =
        factorial(valid_words.len() as u64) / factorial((valid_words.len() - n_words) as u64);
    let perm_run_time = permutation_start_time.elapsed().as_secs_f64();
    println!(
        "Went through {} permutations in {:.3} seconds\n",
        n_perms, perm_run_time
    );

    println!(
        "letter_box.rs -- {:.3} seconds",
        start_time.elapsed().as_secs_f32()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_words_can_join_1() {
        assert!(words_can_join("hi", "it"))
    }

    #[test]
    fn test_words_can_join_2() {
        assert_ne!(words_can_join("hold", "nope"), true)
    }

    #[test]
    #[should_panic(expected = "Did not hand in a long enough string of letters")]
    fn test_create_sides_1() {
        create_sides("hi\nbye");
    }

    #[test]
    fn test_word_is_valid_1() {
        let sides = create_sides("cmo fus nir eph");
        assert!(word_is_valid("ship", &sides));
    }

    #[test]
    fn test_word_is_valid_2() {
        let sides = create_sides("cmo fus nir eph");
        assert_eq!(word_is_valid("hello", &sides), false);
    }
}
