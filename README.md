# Wordle Solver

Implementation of Wordle + an entropy-based solver. The solver has the entire valid word list
and evaluates each guess against the remaining possible answers. Then calculating the entropy of each guess,
chooses the guess which is expected to gain the most information until it solves the puzzle, or uses all six
guesses.


## Run Instructions

```bash
# Play against the built-in answer
cargo run -- play

# Benchmark the solver against every word in wordlist.txt
cargo run --release -- solve
```

Always use `--release` for the full benchmark as the debug build is ~10x slower

## Word lists (dictionaries)

- `wordlist.txt` contains the full Wordle answer list used by the benchmark (2,315 words).
- `validwords.txt` contains all valid wordle guesses (14,855 words).

The benchmark is the source of truth for coverage. A 100% result means it prints:

```text
Number Attempted 2315
Percent Solved 1
```

and reports no failed words. The current checked implementation achieves this result:
it solves all 2,315 answers in `wordlist.txt` (100% coverage), with an average of
4.064795 guesses.

## How feedback is calculated

Each answer/guess pair is encoded as a base-3 number using the feedback for a given letter

| Value | Meaning |
| --- | --- |
| `0` | absent (gray) |
| `1` | present in another position (yellow) |
| `2` | correct position (green) |

The feedback routine follows Wordle's duplicate-letter rules:

1. Validate the answer is 5 letters long and valid answer
2. Calculate the frequency table for the answer
3. Walk through the guess and mark all correct letters green and add the encoded value to the feedback number
2. Walk through and mark all letters contained in the word, but in the wrong spot as present and add the encoded value

This prevents a guess with repeated letters from receiving more yellow/green marks
than the answer contains.

## Feedback table

The solver precomputes feedback rather than re-running the letter-matching algorithm
while choosing each guess. The table contains one `u8` feedback code per
answer/guess pair and is shared read-only by solver workers.

The current layout is indexed as:

```text
feedback_table[guess_index * word_count + answer_index]
```

Where the 'row' (guess_index * word_count) contains feedback of that guess given if the answer was the answer_indexk.

This makes the inner candidate scan for one guess cache-friendly.

## How guesses are chosen

For every possible guess, the solver groups the remaining candidates by their
encoded feedback. If `count_i` candidates produce feedback pattern `i`, it calculates
Shannon entropy:

```text
p_i = count_i / remaining_candidates
H = -sum(p_i * log2(p_i))
```

The guess with the highest entropy is preferred because it is expected to split the
remaining answer set most effectively. A small candidate bonus can be used to
prefer guessing a word that might itself be the answer. However, I found that when you have only 2 candidates
remain, the solver guesses directly from those candidates.
