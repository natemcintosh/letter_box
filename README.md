# letter_box
### Author: Nathan McIntosh

This rust program produces a binary which solves the [NYT Letter Boxed](https://www.nytimes.com/puzzles/letter-boxed) puzzle game. 

The strategy is as follows:
1. Create a HashMap of the sides of the box in `create_sides()`. The keys are integers from 1 to 4. The values are HashSets of the characters on that side. 
1. Read in a file of valid words. A file of valid scrabble words has been included. If I find something closer to a list of valid words in the English language, I'll swap that in. 
1. Determine which words from the list can actually fit into the box by filtering the list with the `word_is_valid()` function. 
1. With `valid_permutations()`, iterate through all of the permutations (default is length 2 permutations, but can be changed with the  `-n` argument when calling the binary) of valid words. 
    - Filter out any that cannot be joined, i.e. the last letter of one word is not the first letter of the next. 
    - Filter out any permutations where all of the letters in the box are not used
1. The remaining permutations are valid solutions to the puzzle. Print them out