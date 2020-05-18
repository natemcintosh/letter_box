use clap::{App, Arg};
use itertools::Itertools;
use std::{
    assert,
    collections::{HashMap, HashSet},
    fs, time,
};

fn valid_combos<'a>(
    valid_words: &'a Vec<&str>,
    sides: &HashMap<i32, HashSet<char>>,
    max_words: &usize,
) -> Vec<Vec<&'a &'a str>> {
    let all_letters: HashSet<_> = sides.values().flatten().cloned().collect();

    valid_words
        .into_iter()
        .permutations(*max_words)
        .filter(|p| {
            all_letters.is_subset(
                &p.into_iter()
                    .map(|&w| w.chars())
                    .flatten()
                    .collect::<HashSet<char>>(),
            )
        })
        .filter(|p| {
            p.iter()
                .tuple_windows()
                .all(|(w1, w2)| words_can_join(w1, w2))
        })
        .collect_vec()
}

fn word_is_valid(word: &str, sides: &HashMap<i32, HashSet<char>>) -> bool {
    let mut bad_side = 0;
    for l in word.chars() {
        if let Some(n) = sides
            .iter()
            .filter(|(&side_number, _side)| side_number != bad_side)
            .filter(|(_side_number, side)| side.contains(&l))
            .map(|(&side_number, _side)| side_number) // get the key
            .last()
        {
            bad_side = n;
        } else {
            return false;
        }
    }
    return true;
}

// fn all_letters_used(words: &Vec<&str>, all_letters: &HashSet<char>) -> bool {
//     let letters_used: HashSet<_> = words.iter().map(|word| word.chars()).flatten().collect();
//     all_letters.is_subset(&letters_used)
// }

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

fn main() {
    let start_time = time::Instant::now();

    let matches = App::new("letter_box")
        .version("0.1")
        .author("Nathan McIntosh")
        .about("Gives you solutions to the letter box puzzle")
        .arg(Arg::with_name("letters"))
        .get_matches();

    // let letters = "car\nimo\nupf\nhnl";
    let letters = matches.value_of("letters").expect("Could not read letters");
    println!("The letters read in are {:?}", letters);
    let sides = create_sides(letters);
    println!("sides are {:?}", sides);

    let read_time = time::Instant::now();
    let words = fs::read_to_string("/Users/mcintna1/Documents/dataSets/scrabble_words.txt")
        .expect("Unable to read file")
        .to_lowercase();
    println!(
        "Reading file took {} seconds",
        read_time.elapsed().as_secs_f32()
    );

    let valid_check_time = time::Instant::now();
    let valid_words = words
        .lines()
        .filter(|w| word_is_valid(w, &sides))
        .collect_vec();

    println!(
        "Found {} valid words in {} seconds",
        &valid_words.len(),
        valid_check_time.elapsed().as_secs_f32()
    );

    let combo_start_time = time::Instant::now();
    let combos = valid_combos(&valid_words, &sides, &2);
    println!("Valid combinations are: {:?}", combos);

    println!(
        "Found valid combos in {} seconds",
        combo_start_time.elapsed().as_secs_f32()
    );

    println!("Ran in {} seconds", start_time.elapsed().as_secs_f32());
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
    fn feature() {}
}
