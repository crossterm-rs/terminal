use bitflags::_core::time::Duration;

use terminal::{stdout, Action, Event, KeyCode, KeyEvent, Result, Value};

fn main() {
    with_duration_read();
}

/// Block read indefinitely for events.
fn block_read() {
    let terminal = stdout();

    terminal.act(Action::EnableRawMode);

    loop {
        if let Result::Event(event) = terminal.get(Value::Event(None)).unwrap() {
            match event {
                Some(Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                })) => {
                    break;
                }
                Some(event) => {
                    println!("{:?}\r", event);
                }
                _ => {}
            }
        }
    }
}

/// Reads events withing a certain duration.
fn with_duration_read() {
    let terminal = stdout();

    terminal.act(Action::EnableRawMode);
    terminal.act(Action::EnableMouseCapture);

    loop {
        if let Result::Event(event) = terminal
            .get(Value::Event(Some(Duration::from_millis(500))))
            .unwrap()
        {
            match event {
                Some(Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                })) => {
                    break;
                }
                Some(event) => {
                    println!("{:?}\r", event);
                }
                None => println!("...\r"),
            }
        }
    }
}
