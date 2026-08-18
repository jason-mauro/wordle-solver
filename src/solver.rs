use crate::wordle_game::GameState;
use crate::wordle_game::GameStatus;
use crate::word::*;

use std::collections::HashMap;

use std::io::{self, Write};

pub struct Solver<'a> {
    // Cached map of each possible (answer, guess) -> possible feedbacks
    // The feedback is encoded in base 3 as 1 bit of 3 for each guess type.
    // 0 = miss
    // 1 = yellow
    // 2 = green
    feedback_map: HashMap<(usize, usize), u8>,

    word_list: Vec<&'a str>,
    candidates: Vec<usize>,
    invalid_indexes: [u8; 26], // 5 bit bitfield for if index is invalid  

    invalid_letters: u32,


}
    
impl<'a> Solver<'a> { 



   pub fn new(word_list: Vec<&'a str>) -> Solver<'a> {

        let mut map = HashMap::new();

        for (word_index, word) in word_list.as_slice().iter().enumerate(){ 
            for (guess_index, guess) in word_list.as_slice().iter().enumerate() { 
                map.insert((word_index, guess_index), Self::feedback(word, guess));
            }
        }

        println!("MAKING NEW");

        io::stdout().flush().expect("Failed to flush stdout");
        Solver {
            feedback_map: map,
            word_list,
            candidates: Vec::new(),
            invalid_indexes: [0; 26],
            invalid_letters: 0
        }
    }


   pub fn solve(&mut self ,game: &mut GameState){
        // reset the state of the Solver for each new solve
        self.candidates = (0..self.word_list.len()).collect();
        self.invalid_indexes = [0;26];
        self.invalid_letters = 0;



        while game.status == GameStatus::INPROGRESS {
            game.print_state();
            let word = self.pick_word();

            let guess = game.make_guess(self.word_list[word]);

            self.update_state(guess);

            println!("IN SOLVING");
        }

        game.print_state();



        


    }

    // Update the state of the solver based on the feedback from the guess and then filter candidates
    fn update_state(&mut self, guess: &Guess){
        let mut seen = 0;
        for (index, letter) in guess.letters.iter().enumerate() {
            match letter.state {
                LetterState::ABSENT => {
                    if !(seen & 1 << letter.value as u8 != 0){
                        self.invalid_letters |= 1 << letter.value as u8;
                    }
                },
                LetterState::PRESENT => {
                    seen |= 1 << letter.value as u8;
                    self.invalid_indexes[letter.value as usize] |= 1 << index
                },
                LetterState::CORRECT => {
                    seen |= 1 << letter.value as u8;

                },
            }
        }

        self.filter_candidates();

    }


    fn feedback(answer: &str, guess: &str) -> u8{

        let mut curr_power = 0;

        let mut result: u8 = 0;

        let mut map = HashMap::new();

        for c in answer.bytes() {
            *map.entry(c).or_insert(0) += 1;
        }

        for (expected, actual) in answer.bytes().zip(guess.bytes()) {
            if expected == actual {
                *map.entry(expected).or_insert(0) -= 1;
                result += 2 * ((3 as i32).pow(curr_power) as u8);
            } else if let Some(count) = map.get(&actual) {
                match *count {
                    0 =>{},
                    _ => {
                        *map.entry(actual).or_insert(1) -= 1;
                        result += (3 as i32).pow(curr_power) as u8 
                    }
                }
            } 
            curr_power += 1;
        }
        result
    }


    
    /*  To pick a word, we want to be able to calculate the entropy of a guess and how much
     *  information that we can gain from it. Meaning that a guess is considered "best" if it 
     *  eliminates the most candidates from the candidate set. Calcualting this can be something
     *  like: 
     *
     *  Entropy = -Sum(p_i * log(p_i)) (sum of probabilities based on the possible feedback)
     *
     *  where p_i is the probability of a given feedback given the number of candidates
     *
     *  We want to maximize the entropy for information gain 
     *      
     */


    pub fn calculate_entropy(&self, guess: usize) -> f32 {
        let candidate_size = self.candidates.len();

        let mut counter: HashMap<u8, u32> = HashMap::new();
            
        for candidate in self.candidates.as_slice() {
           match self.feedback_map.get(&(*candidate, guess)) {
                Some(value) => {
                    let count_ref = counter.entry(*value).or_insert(0);
                    *count_ref += 1;
                },
                None => panic!("missing feedback entry")
           }
        }

         -counter.values().map(|value| {
                let p = (*value as f32) / (candidate_size as f32);
                p * p.log2()
            }).sum::<f32>()
    }


    /* To pick a word, we want to select a candidate from the candidate set
     *  s.t we maximize entropy (information gain)
     *
     *  So we calculate the entropy of candidate against each candidate and then
     *  we select the one that gives the most entropy.
     */

    fn pick_word(&self) -> usize{
        let mut choosen_word: usize = 0;
        let mut max_entropy = f32::MIN;

        if self.candidates.is_empty() {
            panic!("No candidates!");
        }



        // choose the word with the most entropy
        for word in self.candidates.as_slice() {
            let entropy = self.calculate_entropy(*word);
            if entropy > max_entropy {
                max_entropy = entropy;
                choosen_word = *word
            }
            max_entropy = f32::max(max_entropy, entropy);
        }

        choosen_word
    }

    fn filter_candidates(&mut self) {
        println!("Candidates before {}", self.candidates.len());
        
        self.candidates = self.candidates
            .iter()
            .filter(|candidate|
                {
                    let word = self.word_list[**candidate];

                    // Either we have a letter which is invalid (not in the answer)
                    // OR 
                    // we have a letter which is in the word, but is not valid at that index

                    for (index, c) in word.bytes().enumerate() {
                        let letter_index = (c - b'a') as usize;
                        if self.invalid_letters & 1 << letter_index != 0 ||
                            self.invalid_indexes[letter_index] & 1 << index != 0 {
                            return false;
                        } 
                    }
                true
                }).copied().collect()
    }


        

}





