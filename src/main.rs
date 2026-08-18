mod word;
mod wordle_game;
mod solver;


use std::fs;
use crate::solver::Solver;
use crate::wordle_game::GameState;
use std::collections::HashSet;

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

            wordle_game::play(&word_set, "hello");
               

        },

        Commands::Solve => {
            println!("IN SOLVE");
        io::stdout().flush().expect("Failed to flush stdout");
            let valid_word_list = fs::read_to_string("validwords.txt").expect("Could not locate valid word list");
            
            let words = valid_word_list.split_whitespace().collect();
println!("make words");
        io::stdout().flush().expect("Failed to flush stdout");

        
println!("making solver");
        io::stdout().flush().expect("Failed to flush stdout");


            let mut solver = Solver::new(words);

 
println!("made solver");
        io::stdout().flush().expect("Failed to flush stdout");



            solver.solve(&mut GameState::new("spunk"));
        },
    }

}
