use crate::wordle_game::GameState;
use crate::wordle_game::GameStatus;
use crate::word::*;


use std::io::{self, Write};
use std::sync::Arc;
pub struct Solver {
    // Cached map of each possible (answer, guess) -> possible feedbacks
    // The feedback is encoded in base 3 as 1 bit of 3 for each guess type.
    // 0 = miss
    // 1 = yellow
    // 2 = green
    feedback_map: Arc<Vec<u8>>,
    word_list: Vec<[u8; 5]>,
    in_candidate_list: Vec<bool>,
    candidates: Vec<usize>,
    invalid_indexes: [u8; 26], // 5 bit bitfield for if index is invalid
    best_first_guess: Option<usize>,

    invalid_letters: u32,
}

pub enum SolveResult {
    Ok(u8),
    Fail
}

impl Solver { 


   const POWER_OF_THREE: [u8; 5] = [1, 3,9,27,81];

   pub fn new(word_list: Vec<[u8; 5]>, map: Arc<Vec<u8>> ) -> Solver {


        let length = word_list.len();

        
        io::stdout().flush().expect("Failed to flush stdout");
        Solver {
            feedback_map: map,
            word_list,
            best_first_guess: None,
            candidates: Vec::new(),
            in_candidate_list: vec![true; length],
            invalid_indexes: [0; 26],
            invalid_letters: 0,
        }
    }




    

   pub fn solve(&mut self ,game: &mut GameState, print_solve: bool ) -> SolveResult {

        // reset the state of the Solver for each new solve
        self.candidates = (0..self.word_list.len()).collect();
        self.invalid_indexes = [0;26];
        self.invalid_letters = 0;

        while game.status == GameStatus::INPROGRESS {

            if print_solve {
                game.print_state();
            }

            // We want to cache the first guess that the solver picks as it is deterministic
            // for the first guess given we have no information.
            // Since the first guess takes the longest to compute give we need to search n^2 
            // of the whole word list, we cache it for performance 
            let word = if game.guess_count != 0 { 
                self.pick_word()
            } else {
                match self.best_first_guess {
                    Some(index) => index,
                    None => {
                        let best_guess = self.pick_word();
                        self.best_first_guess = Some(best_guess);
                        best_guess
                    }
                }
            };



            let guess = game.make_guess(self.word_list[word]);

            if print_solve {
                println!("Guessing: {}",
                    String::from_utf8(self.word_list[word].to_vec()).unwrap());
            }
                self.update_state(word, guess);
        }

        if print_solve {
            game.print_state();
        }


        match game.status {
            GameStatus::WINNER => SolveResult::Ok(game.guess_count),
            GameStatus::LOSER => SolveResult::Fail,
            GameStatus::INPROGRESS => unreachable!()
        }
    }

    // Update the state of the solver based on the feedback from the guess and then filter candidates
    fn update_state(&mut self, word: usize, guess: &Guess){
        let mut seen = 0;

        let mut feedback = 0u8;

        for i in 0..5 {
            let letter = guess.letters[i];

            match letter.state  {
                LetterState::PRESENT => {
                    feedback += Self::POWER_OF_THREE[i];
                    seen |= 1 << letter.value as u8;
                    self.invalid_indexes[letter.value as usize] |= 1 << i 

                },
                LetterState::CORRECT => {
                    feedback += 2 * Self::POWER_OF_THREE[i];
                    seen |= 1 << letter.value as u8;
                },
                LetterState::ABSENT => {}
            }

        }

        for i in 0..5 {
            let letter = guess.letters[i];
            
            if letter.state == LetterState::ABSENT && 
                !(seen & 1 << letter.value as u8 != 0) {
                    self.invalid_letters |= 1 << letter.value as u8;
                }
        }

        self.filter_candidates(word, feedback);

    }


    pub fn feedback(answer: [u8; 5], guess: [u8; 5]) -> u8 {
        let mut result: u8 = 0;

        let mut map = [0; 26];

        for i in 0..5 {
            map[(answer[i] - b'a') as usize] += 1;
        }

        for i in 0..5 { 
            if answer[i] == guess[i]{
                result += 2 * Self::POWER_OF_THREE[i];
                map[(guess[i] - b'a') as usize] -= 1;
            }
        }


        for i in 0..5 { 
            if answer[i] != guess[i] {
                let c = (guess[i] - b'a') as usize;
                if map[c] > 0 {
                    result += Self::POWER_OF_THREE[i];
                    map[c] -= 1;
                }
            }
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

        let mut counter = [0; 243];           

        let n = self.word_list.len();

        for i in 0..candidate_size {
            let feedback = self.feedback_map[guess * n + self.candidates[i] ] as usize;
            counter[feedback] += 1;
        }

         let entropy = -counter.iter()
             .filter(|&&count| count > 0)
             .map(|value| {
                let p = (*value as f32) / (candidate_size as f32);
                p * p.log2()
            }).sum::<f32>();

         let candidate_bonus = if self.in_candidate_list[guess] { 0.75f32 } else { 0f32 };

         entropy + candidate_bonus
    }


    /* To pick a word, we want to select a candidate from the candidate set
     *  s.t we maximize entropy (information gain)
     *
     *  So we calculate the entropy of candidate against each candidate and then
     *  we select the one that gives the most entropy.
     */

    fn pick_word(&self) -> usize{
        let mut chosen_word: usize = 0;
        let mut max_entropy = f32::MIN;


        if self.candidates.len() <= 2 {
            for &word in &self.candidates {
                let entropy = self.calculate_entropy(word);

                if entropy > max_entropy {
                    max_entropy = entropy;
                    chosen_word = word;
                }
            }
        } else {
            // More candidates: allow information-gathering guesses.
            for word in 0..self.word_list.len() {
                let entropy = self.calculate_entropy(word);

                if entropy > max_entropy {
                    max_entropy = entropy;
                    chosen_word = word;
                }
            }
        }

        chosen_word
    }

    fn filter_candidates(&mut self, guess: usize, feedback: u8) {
        let n = self.word_list.len();
        
        self.candidates = self.candidates
            .iter()
            .filter(|candidate|
                {
                    if  self.feedback_map[guess * n + **candidate] != feedback{
                        self.in_candidate_list[**candidate] = false;
                        false
                    } else {
                        true
                    }
                }).copied().collect();
    }


        

}





