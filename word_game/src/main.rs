use std::time::Duration;
use termion::{color, style};

mod game;
mod player;
mod utils;
mod constants; // Import constants

use game::{GameError, GameState};
use player::{HumanPlayer, Player};
use constants::{TIME_LIMIT, MAX_ATTEMPTS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut player1 = HumanPlayer::new();
    let mut player2 = HumanPlayer::new();

    println!("Player 1, enter the secret word:");
    let secret_word = player1.get_guess()?;
    println!("The secret word has {} letters.", secret_word.len());

    let mut game = GameState::new(secret_word.to_uppercase(), TIME_LIMIT, MAX_ATTEMPTS);
    game.start();

    println!("\nPlayer 2, start guessing! You have {} seconds and {} attempts.", TIME_LIMIT.as_secs(), MAX_ATTEMPTS);

    while !game.has_ended() {
        let guess_result = player2.get_guess();
        match guess_result {
            Ok(guess) => {
                match game.process_guess(&guess) {
                    Ok(matches) => {
                        utils::display_colored_matches(&guess, matches, game.get_secret_word().len());
                        println!("Attempts left: {}", game.get_attempts_left());
                        if matches == game.get_secret_word().len() {
                            println!("{}{}Congratulations! You guessed the word: {}{}", color::Fg(color::Green), style::Bold, guess, style::Reset);
                            break;
                        }
                    }
                    Err(GameError::InvalidGuessLength { expected, actual }) => {
                        println!("{}{}{}", color::Fg(color::Red), format!("Invalid guess length. Expected {}, got {}.", expected, actual), style::Reset);
                    }
                    Err(GameError::TimeUp(secret)) => {
                        println!("{}{}Time's up! The word was: {}{}", color::Fg(color::Red), style::Bold, secret, style::Reset);
                        break;
                    }
                    Err(GameError::GameEnded) => {
                        println!("The game has already ended.");
                        break;
                    }
                    Err(e) => eprintln!("An unexpected error occurred: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }

    let secret_word = game.get_secret_word().to_string(); // FIX FOR E0502
    if game.has_ended() {
        if game.get_attempts_left() == 0 {
            println!(
                "{}{}You ran out of attempts! The word was: {}{}",
                color::Fg(color::Red),
                style::Bold,
                game.get_secret_word(),
                style::Reset
            );
        }
    }
    

    Ok(())
}
