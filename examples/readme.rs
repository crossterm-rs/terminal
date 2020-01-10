use std::io::Write;
use terminal::{error, Action, Clear, Retrieved, Value};

pub fn main() -> error::Result<()> {
    let mut terminal = terminal::stdout();

    // perform an single action.
    terminal.act(Action::ClearTerminal(Clear::All))?;

    // batch multiple actions.
    for i in 0..20 {
        terminal.batch(Action::MoveCursorTo(0, i))?;
        terminal.write(format!("{}", i).as_bytes());
    }

    // execute batch.
    terminal.flush_batch();

    // get an terminal value.
    if let Retrieved::TerminalSize(x, y) = terminal.get(Value::TerminalSize)? {
        println!("\nx: {}, y: {}", x, y);
    }

    Ok(())
}
