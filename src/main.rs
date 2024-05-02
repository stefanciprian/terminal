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

    // Clear the terminal and set cursor position
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(5, 5))?;

    // Set the text color to green "hacker style"
    let styled_message = "Hello, welcome to my terminal app!".green();
    stdout.execute(PrintStyledContent(styled_message))?;

    // Inform the user about how to exit
    let exit_message = "\nPress CTRL+C to exit.".dark_grey();
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
                            let input_message = format!("\nYou pressed: '{}'", c);
                            let styled_input = input_message.green();
                            stdout.execute(PrintStyledContent(styled_input))?;
                        }
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
