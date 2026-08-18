use crate::word::Guess;
use crate::word::Letter;
use crate::word::LetterState;

use std::collections::HashSet;

use std::io::{self, Write};


pub fn play(valid_words: &HashSet<&str>, word: &str){


    let mut gamestate = GameState::new(word);

    println!("Welcome to Wordle!");


    println!("Try to guess a 5 letter word with 6 guesses!");

    println!("\n\n");


    let mut input = String::new();

    while  gamestate.status == GameStatus::INPROGRESS {
        input.clear();
        gamestate.print_state();
        print!("Enter a guess: ");

        // flush output so this is not buffered.
        io::stdout().flush().expect("Failed to flush stdout");

        // Read + validate user input 
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line for guess");

        input = input.trim().to_lowercase().to_string();

        if !is_valid_guess(&input){
            println!("Invalid Guess! Ensure your guess only contains letters 'A'-'Z'!");
            continue;
        }

        if input.len() != 5 {
            println!("{}", input.len());
            println!("Invalid Guess! Please enter a word that is 5 letters long");    
            continue;
        }

        gamestate.make_guess(&input); 
    }


    match gamestate.status {

        GameStatus::WINNER => {

            gamestate.print_state();
        },
        GameStatus::LOSER => {
            gamestate.print_state();
        },

        _ => panic!("Invalid game state!")
    }
    


}

// Validate all chars are english letters
fn is_valid_guess(guess: &str) -> bool {
    guess.chars()
        .all(|c| c.is_ascii_alphabetic())
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    INPROGRESS,
    WINNER,
    LOSER
}

pub struct GameState <'a> {
    word: &'a str,
    prev_guesses: Vec<Guess>,
    max_guesses: u8,
    guess_count: u8,
    freq_array: [u8; 26],
    pub status: GameStatus
}

impl<'a> GameState <'a> {

    pub fn new (word : &'a str) -> GameState<'a> {
        let mut freq_array: [u8; 26] = [0; 26];

        // assume everything lowercase ascii
        for c in word.to_ascii_lowercase().bytes(){
            freq_array[(c- b'a') as usize] += 1;
        }

        GameState {
            word, 
            prev_guesses: Vec::new(),
            max_guesses: 6,
            guess_count: 0,
            freq_array,
            status: GameStatus::INPROGRESS
        }
    }

    pub fn make_guess(&mut self, user_guess: &str) -> &Guess {
        self.guess_count += 1;

        // If at max guesses, we set the state to loss, but update if correct
        if self.guess_count == self.max_guesses{
            self.status = GameStatus::LOSER;
        }

        let mut number_correct = 0; 

        let mut freq_table: [u8;26] = self.freq_array;

        let letters: Vec<Letter> = 
            std::iter::zip(self.word.bytes(), user_guess.to_ascii_lowercase().bytes())
            .map(|(expected, actual)| 
        {
            let index = (actual - b'a') as usize;
            match freq_table[index] {
                0 => Letter::get_letter(actual, LetterState::ABSENT),
                _ => {
                    freq_table[index] -= 1;
                    
                    let state = match actual == expected {
                        true => { 
                            number_correct += 1;
                            LetterState::CORRECT
                        }
                        false => LetterState::PRESENT
                    };

                    Letter::get_letter(actual, state)
                }
            }
        }).collect();

        if number_correct == 5 {
            self.status = GameStatus::WINNER;
        }

        let guess =  Guess {
            letters
        };

        self.prev_guesses.push(guess);

        return self.prev_guesses.last().unwrap();
    }
            
    pub fn print_state(&self) {
        for guess in self.prev_guesses.as_slice() {
            let mut buff: [char; 11] = [' ';11];
            for (i, letter) in guess.letters.as_slice().iter().enumerate() {
                match letter.state {
                    LetterState::ABSENT => {
                        buff[i] = '⬜'; 
                        buff[i + 6] = letter.value.get_char();

                    },
                    LetterState::CORRECT => {
                        buff[i] = '🟩'; 
                        buff[i + 6] = letter.value.get_char();

                    }, 
                    LetterState::PRESENT => {
                        buff[i] = '🟨'; 
                        buff[i + 6] = letter.value.get_char();
                    }
                }
            }
            let combined_string = String::from_iter(buff);
            println!("{}", combined_string);
        }

    }
}

