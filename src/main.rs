mod word;
mod wordle_game;
mod solver;


use std::fs;
use crate::solver::Solver;
use crate::wordle_game::GameState;
use std::collections::HashSet;
use crate::solver::SolveResult;

use rayon::prelude::*;
use std::io::{self, Write};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wordle-solver")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Play Wordle against the CPU
    Play,

    /// Solve a random word
    Solve
}


fn main() {
    let cli = Cli::parse();

    match cli.command {

        Commands::Play => {
            let word_list = fs::read_to_string("wordlist.txt").expect("Could not locate word list");
            
            let answers: Vec<&str> = word_list.split_whitespace().collect();

            let valid_word_list = fs::read_to_string("validwords.txt").expect("Could not locate valid word list");
            
            let words = valid_word_list.split_whitespace();

            let mut word_set = HashSet::new();

            let mut word_list = HashSet::new();

            for word in words {
                word_set.insert(word);
                word_list.insert(word);
            }

            wordle_game::play(&word_set, "hello".as_bytes().try_into().unwrap());
               

        },

        Commands::Solve => {

            let answers = fs::read_to_string("wordlist.txt").expect("Cound not locate answers");
            let valid_word_list = fs::read_to_string("validwords.txt")
                                    .expect("Could not locate valid word list");
            
            let words: Vec<[u8; 5]> = valid_word_list.split_whitespace() 
                .map(|s| s.as_bytes().try_into().unwrap()).collect();

            let answer_set: Vec<[u8; 5]> = answers.split_whitespace() 
                .map(|s| s.as_bytes().try_into().unwrap()).collect();

            let n = answer_set.len();

            let (number_guesses, number_failed) = answer_set 
                .par_chunks(1000)
                .map(|chunk| 
                    {

                        println!("Starting Chunk!");
                        let mut number_failed = 0usize;

                        let mut number_guesses = 0usize;

                        let mut solver = Solver::new(words.clone());
                        for word in chunk {
                            let result = solver.solve(&mut GameState::new(*word), false);
                            match result {
                                SolveResult::Ok(num_guesses) => number_guesses += num_guesses as usize,
                                SolveResult::Fail => number_failed += 1
                            }; 
                        }
                        println!("FINISHED CHUNK!");

                        (number_guesses, number_failed)
                    }).reduce(|| (0,0), |(a, b), (c, d)| 
                        (a + c, b + d), 
                    );
                     


            println!("Number Attempted {}", n);

            println!("Percent Solved {}", (n - number_failed) as f32 / n as f32);

            println!("Average Guess count {}", number_guesses as f32 / (n - number_failed) as f32);





        },
    }

}
