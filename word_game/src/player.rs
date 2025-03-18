use std::io::{self, Write};

pub trait Player {
fn get_guess(&mut self) -> Result<String, io::Error>;
fn display_feedback(&mut self, guess: &str, matches: usize);
fn display_game_over(&mut self, secret_word: &str);
}

pub struct HumanPlayer {
// Could hold player-specific state if needed
}

impl HumanPlayer {
pub fn new() -> Self {
HumanPlayer {}
}
}

impl Player for HumanPlayer {
fn get_guess(&mut self) -> Result<String, io::Error> {
print!("Enter your guess: ");
io::stdout().flush()?;
let mut guess = String::new();
io::stdin().read_line(&mut guess)?;
Ok(guess.trim().to_uppercase()) // Assuming case-insensitive
}

fn display_feedback(&mut self, guess: &str, matches: usize) {
    println!("Your guess: {}", guess);
    println!("Matches: {}", matches);
}

fn display_game_over(&mut self, secret_word: &str) {
    println!("Game Over! The secret word was: {}", secret_word);
}
}