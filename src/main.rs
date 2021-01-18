use clap::{App, Arg};
use itertools::Itertools;
use num_bigint::BigUint;
use std::{
    assert,
    collections::{HashMap, HashSet},
    fs, time,
};

fn valid_permutations<'a>(
    valid_words: &'a Vec<&str>,
    sides: &HashMap<i32, HashSet<char>>,
    max_words: &usize,
) -> impl Iterator<Item = Vec<&'a &'a str>> {
    let all_letters: HashSet<_> = sides.values().flatten().cloned().collect();

    let n_perms =
        factorial(valid_words.len() as u64) / factorial((valid_words.len() - max_words) as u64);
    println!("There are {} permutations to check\n", n_perms);

    valid_words
        .iter()
        .permutations(*max_words)
        .filter(|p| {
            p.iter()
                .tuple_windows()
                .all(|(w1, w2)| words_can_join(w1, w2))
        })
        .filter(move |p| all_letters.is_subset(&get_unique_chars(p)))
}

fn get_unique_chars(words: &Vec<&&str>) -> HashSet<char> {
    words
        .iter()
        .map(|&w| w.chars())
        .flatten()
        .collect::<HashSet<_>>()
}

fn word_is_valid(word: &str, sides: &HashMap<i32, HashSet<char>>) -> bool {
    let mut last_used_side = 0;
    for l in word.chars() {
        match sides
            .iter()
            .filter(|(&side_number, _side)| side_number != last_used_side)
            .filter(|(_side_number, side)| side.contains(&l))
            .map(|(&side_number, _side)| side_number) // get the key
            .last()
        {
            Some(n) => {
                last_used_side = n;
            }
            None => return false,
        }
    }
    return true;
}

fn words_can_join(w1: &str, w2: &str) -> bool {
    let end_of_first = w1.chars().nth_back(0).expect("Could not get last char");
    let start_of_second = w2.chars().nth(0).expect("Could not get first char");
    end_of_first == start_of_second
}

fn create_sides(letters: &str) -> HashMap<i32, HashSet<char>> {
    assert!(
        letters.len() >= 12,
        "Did not hand in a long enough string of letters"
    );

    (1..)
        .zip(
            letters
                .split_whitespace()
                .map(|p| p.chars().collect::<HashSet<_>>()),
        )
        .collect::<HashMap<_, _>>()
}

fn factorial(n: u64) -> BigUint {
    (1..=n).product()
}

fn main() {
    let start_time = time::Instant::now();

    let matches = App::new("letter_box")
        .version("0.3.0")
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
    println!("The letters read in are {:?}", letters);
    let sides = create_sides(letters);
    println!("sides are {:?}", sides);

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
    let valid_words = words
        .lines()
        .filter(|&w| !w.ends_with("'s"))
        .filter(|w| word_is_valid(w, &sides))
        .collect_vec();

    println!(
        "Found {} valid words in {:.3} seconds",
        &valid_words.len(),
        valid_check_time.elapsed().as_secs_f32()
    );

    let permutation_start_time = time::Instant::now();
    let v = valid_permutations(&valid_words, &sides, &n_words);
    v.for_each(|pair| {
        let joined = pair.into_iter().join(" - ");
        println!("{}", joined)
    });

    println!(
        "\nFound valid permutations in {:.3} seconds",
        permutation_start_time.elapsed().as_secs_f32()
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
