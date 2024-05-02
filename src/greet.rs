use crossterm::style::{PrintStyledContent, Stylize};
use crossterm::ExecutableCommand; // Import the necessary trait
use std::io::{self, stdout, Write};

// This function is moved to `commands.rs`
pub fn greet_command() {
    let mut stdout = stdout();
    let greeting = "Hello from Rust! You've executed the 'greet' command.".green();
    stdout.execute(PrintStyledContent(greeting)).unwrap(); // Now `execute` can be used here
    stdout.flush().unwrap();
}
