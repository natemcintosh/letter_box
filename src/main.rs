use clap::{App, Arg};
use itertools::Itertools;
use num_bigint::BigUint;
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

    let n_perms =
        factorial(valid_words.len() as u64) / factorial((valid_words.len() - max_words) as u64);
    println!("There are {} permutations to check", n_perms);

    valid_words
        .into_iter()
        .permutations(*max_words)
        .filter(|p| {
            p.iter()
                .tuple_windows()
                .all(|(w1, w2)| words_can_join(w1, w2))
        })
        .filter(|p| all_letters.is_subset(&get_unique_chars(p)))
        .collect_vec()
}

fn get_unique_chars(words: &Vec<&&str>) -> HashSet<char> {
    words
        .iter()
        .map(|&w| w.chars())
        .flatten()
        .collect::<HashSet<_>>()
}

fn word_is_valid(word: &str, sides: &HashMap<i32, HashSet<char>>) -> bool {
    let mut bad_side = 0;
    for l in word.chars() {
        match sides
            .iter()
            .filter(|(&side_number, _side)| side_number != bad_side)
            .filter(|(_side_number, side)| side.contains(&l))
            .map(|(&side_number, _side)| side_number) // get the key
            .last()
        {
            Some(n) => {
                bad_side = n;
            }
            None => return false,
        }
    }
    return true;
}

// fn rank_combos<'a>(mut combos: Vec<Vec<&'a &'a str>>) {
//     // combos.sort_unstable_by(|a, b| a.len().cmp(&b.len()))
//     combos.sort_unstable_by(|a, b| {
//         a.iter()
//             .map(|w| w.len())
//             .sum::<usize>()
//             .cmp(&b.iter().map(|w| w.len()).sum::<usize>())
//     })
// }

// fn num_duplicates<'a>(words: Vec<&'a &'a str>) -> u32 {
//     let mut map: HashMap<char, u32> = HashMap::new();
//     let characters = words.into_iter().map(|&w| w.chars()).flatten();
//     for c in characters {
//         if map.contains_key(&c) {
//             match map.get_mut(&c) {
//                 Some(count) => *count += 1,
//                 None => panic!("Could not retrieve element"),
//             }
//         } else {
//             map.insert(c, 0);
//         }
//     }
//
//     map.values().sum()
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

fn print_combos(combos: &Vec<Vec<&&str>>) {
    // The idea here is to print each combo as `word1 - word2\n`
    for c in combos {
        let joined = c.into_iter().join(" - ");
        println!("{}", joined)
    }
}

fn factorial(n: u64) -> BigUint {
    (1..=n).product()
}

fn main() {
    let start_time = time::Instant::now();

    let matches = App::new("letter_box")
        .version("0.1")
        .author("Nathan McIntosh")
        .about("Gives you solutions to the letter box puzzle")
        .arg(Arg::with_name("letters"))
        .arg(
            Arg::with_name("n")
                .help("How many words in your solutions")
                .required(false)
                .default_value("2"),
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
    let words = fs::read_to_string("/Users/mcintna1/Documents/dataSets/scrabble_words.txt")
        .expect("Unable to read file")
        .to_lowercase();
    println!(
        "Reading file took {:.3} seconds",
        read_time.elapsed().as_secs_f32()
    );

    let valid_check_time = time::Instant::now();
    let valid_words = words
        .lines()
        .filter(|w| word_is_valid(w, &sides))
        .collect_vec();

    println!(
        "Found {} valid words in {:.3} seconds",
        &valid_words.len(),
        valid_check_time.elapsed().as_secs_f32()
    );

    let combo_start_time = time::Instant::now();
    let combos = valid_combos(&valid_words, &sides, &n_words);
    // let ranked_combos = combos.clone();
    // rank_combos(ranked_combos);
    println!("Valid combinations are:");
    print_combos(&combos);

    println!(
        "Found valid combos in {:.3} seconds",
        combo_start_time.elapsed().as_secs_f32()
    );

    println!("Ran in {:.3} seconds", start_time.elapsed().as_secs_f32());
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
