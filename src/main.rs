use itertools::Itertools;
use std::{
    assert,
    collections::{HashMap, HashSet},
    fs, time,
};

fn valid_combos<'a>(
    valid_words: &'a Vec<String>,
    sides: &HashMap<i32, HashSet<char>>,
    max_words: &usize,
) -> Vec<Vec<&'a String>> {
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
        let filtered_sides = sides
            .iter()
            .filter(|(&side_number, _side)| side_number != bad_side)
            .filter(|(_side_number, side)| side.contains(&l))
            .collect::<HashMap<&i32, &HashSet<char>>>();

        assert!(
            filtered_sides.len() <= 1,
            "Found matching letters on more than one side: {:?}",
            filtered_sides
        );

        if let Some(n) = filtered_sides.keys().last() {
            bad_side = **n;
        } else {
            return false;
        }
    }
    true
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
        .zip(letters.lines().map(|p| p.chars().collect::<HashSet<_>>()))
        .collect::<HashMap<_, _>>()
}

fn main() {
    let start_time = time::Instant::now();

    let letters = "car\nimo\nupf\nhnl";
    let sides = create_sides(letters);
    println!("mymap is {:?}", sides);

    let valid_words = fs::read_to_string("/Users/mcintna1/Documents/dataSets/scrabble_words.txt")
        .expect("Unable to read file")
        .lines()
        .map(|l| l.to_lowercase())
        .filter(|w| word_is_valid(w, &sides))
        .collect_vec();

    println!("Found {} valid words", &valid_words.len());

    let combos = valid_combos(&valid_words, &sides, &2);
    println!("Valid combinations are: {:?}", combos);

    println!("Ran in {} seconds", start_time.elapsed().as_secs_f32());
}
