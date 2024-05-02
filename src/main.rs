use crossterm::{
    execute,
    terminal::{self, ClearType},
    style::{PrintStyledContent, Stylize},
    event::{self, KeyCode, KeyEvent, Event},
    cursor,
    ExecutableCommand,
};
use std::io::{self, Write, stdout, stdin};

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    let mut input_buffer = String::new();

    // Clear the terminal and set cursor position
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(10, 10))?;

    // Set the text color to green "hacker style"
    let styled_message = "Hello, welcome to Terminal!".green();
    stdout.execute(PrintStyledContent(styled_message))?;

    // Inform the user about how to exit
    let exit_message = "\nType your commands here. Press ENTER to process. Press ESC or CTRL+C to exit.\n".dark_grey();
    stdout.execute(PrintStyledContent(exit_message))?;

    // Flush changes to terminal
    stdout.flush()?;

    // Read and process input until CTRL+C is pressed
    loop {
        if event::poll(std::time::Duration::from_secs(1))? {
            match event::read()? {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Char(c) => {
                            input_buffer.push(c);
                        },
                        KeyCode::Enter => {
                            let input_message = format!("\nYou typed: '{}'\n", input_buffer);
                            let styled_input = input_message.green();
                            stdout.execute(PrintStyledContent(styled_input))?;
                            input_buffer.clear();  // Clear the buffer after processing
                        },
                        KeyCode::Esc => break,  // Optionally handle ESC key to exit
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
