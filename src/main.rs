use crossterm::{
    execute,
    terminal::{self, ClearType},
    ExecutableCommand, cursor,
};
use std::io::{self, Write};

fn main() -> crossterm::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(10, 10))?;

    writeln!(stdout, "Hello, welcome to my terminal app!")?;
    stdout.flush()?;

    // Keep the window open
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}