# letter_box
### Author: Nathan McIntosh
---
### About
This rust program produces a binary which solves the [NYT Letter Boxed](https://www.nytimes.com/puzzles/letter-boxed) puzzle game.

---
### Compiling and Running
1. Make sure you have the [Rust programing language](https://www.rust-lang.org) installed
1. Clone this repo to a location on your computer: `git clone https://github.com/natemcintosh/letter_box.git`
1. Build in release mode: `cargo build --release`
1. Run the produced binary asking for help to see all options: `./target/release/letter_box -h`
1. Run the binary on the puzzle inputs for the day, e.g.: `./target/release/letter_box "abc def ghi jkl"`

---
### Strategy for solving the puzzle
1. Create an array of the sides of the box in `create_sides()`. The keys are integers from 1 to 4. The values are HashSets of the characters on that side.
1. Read in a file of valid words. A file of words from the American English Dictionary is included.
1. Determine which words from the list can actually fit into the box by filtering the list with the `word_is_valid()` function.
1. With `valid_permutations()`, iterate through all of the permutations (default is length 2 permutations, but can be changed with the  `-n` argument when calling the binary) of valid words.
    - Filter out any that cannot be joined, i.e. the last letter of one word is not the first letter of the next.
    - Filter out any permutations where all of the letters in the box are not used
1. The remaining permutations are valid solutions to the puzzle. Print them out