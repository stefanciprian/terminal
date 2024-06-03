mod env_vars;
mod greet;
mod websocket;
mod websocket_client;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{PrintStyledContent, Stylize},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    let mut input_buffer = String::new();

    // Clear the terminal and set cursor position
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(5, 1))?;

    // Set the text color to green "hacker style"
    let styled_message = "Hello, welcome to Terminal!".green();
    stdout.execute(PrintStyledContent(styled_message))?;

    // Inform the user about how to exit
    let exit_message =
        "\nType your commands here. Press ENTER to process. Press ESC or CTRL+C to exit.\n"
            .dark_grey();
    stdout.execute(PrintStyledContent(exit_message))?;

    // Flush changes to terminal
    stdout.flush()?;

    // Display commands that can be executed
    let commands_message = "\nCommands: greet, websocket, websocket2\n".dark_grey();

    stdout.execute(PrintStyledContent(commands_message))?;
    // Read and process input until CTRL+C is pressed
    loop {
        if event::poll(std::time::Duration::from_secs(1))? {
            match event::read()? {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Char(c) => {
                            input_buffer.push(c);
                        }
                        KeyCode::Enter => {
                            let input_message = format!("\nYou typed: '{}'\n", input_buffer);
                            let styled_input = input_message.green();
                            stdout.execute(PrintStyledContent(styled_input))?;

                            if input_buffer.trim() == "greet" {
                                println!("You've executed the 'greet' command.");
                                greet::greet_command(); // Call the greet_command from the commands module
                            }

                            if input_buffer.trim() == "env_vars" {
                                println!("You've executed the 'env_vars' command.");
                                env_vars::env_vars_command(); // Call the env_vars_command from the commands module  
                            }

                            if input_buffer.trim() == "websocket" {
                                println!("You've executed the 'websocket' command.");
                                websocket::send_websocket_message_command(); // Call the send_websocket_message_command from the commands module
                            }

                            if input_buffer.trim() == "websocket2" {
                                websocket_client::test(); // Call the test from the websocket_client module
                            }

                            if input_buffer.trim() == "exit" {
                                break;
                            }

                            input_buffer.clear(); // Clear the buffer after processing
                        }
                        KeyCode::Esc => break, // Optionally handle ESC key to exit
                        _ => (),
                    }
                    stdout.flush()?;
                }
                _ => (),
            }
        }
    }

    Ok(())
}
