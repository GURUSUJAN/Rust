use termion::{color, style};

pub fn display_colored_matches(guess: &str, matches: usize, word_length: usize) {
    let percentage = if word_length > 0 {
        (matches as f32 / word_length as f32) * 100.0
    } else {
        0.0
    };

    let color = match percentage as i32 {
        0..=20 => color::Fg(color::Red).to_string(),
        21..=40 => color::Fg(color::Yellow).to_string(),
        41..=60 => color::Fg(color::Rgb(255, 165, 0)).to_string(), // Orange
        61..=80 => color::Fg(color::Green).to_string(),
        81..=100 => color::Fg(color::LightGreen).to_string(),
        _ => color::Fg(color::White).to_string(), // Default
    };

    println!("Guessed: {}{}{}", color, guess, style::Reset);
    println!("Matches: {}{}{}", color, matches, style::Reset);
}
