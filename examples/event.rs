use bitflags::_core::time::Duration;

use terminal::{error, stdout, Action, Event, KeyCode, KeyEvent, Retrieved, Value};

fn main() {
    with_duration_read();
}

/// Block read indefinitely for events.
fn block_read() -> error::Result<()> {
    let terminal = stdout();

    terminal.act(Action::EnableRawMode)?;

    loop {
        if let Retrieved::Event(event) = terminal.get(Value::Event(None))? {
            match event {
                Some(Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                })) => return Ok(()),
                Some(event) => {
                    println!("{:?}\r", event);
                }
                _ => {}
            }
        }
    }
}

/// Reads events withing a certain duration.
fn with_duration_read() -> error::Result<()> {
    let terminal = stdout();

    terminal.act(Action::EnableRawMode)?;
    terminal.act(Action::EnableMouseCapture)?;

    loop {
        if let Retrieved::Event(event) =
            terminal.get(Value::Event(Some(Duration::from_millis(500))))?
        {
            match event {
                Some(Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                })) => return Ok(()),
                Some(event) => {
                    println!("{:?}\r", event);
                }
                None => println!("...\r"),
            }
        }
    }
}
