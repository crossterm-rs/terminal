use std::{fs::File, thread, time::Duration};

use terminal::{error, stderr, stdout, Action, Clear, Retrieved, Terminal, Value};

fn different_buffers() {
    let _stdout = stdout();
    let _stderr = stderr();
    let _file = Terminal::custom(File::create("./test.txt").unwrap());
}

/// Gets values from the terminal.
fn get_value() -> error::Result<()> {
    let stdout = stdout();

    if let Retrieved::CursorPosition(x, y) = stdout.get(Value::CursorPosition)? {
        println!("X: {}, Y: {}", x, y);
    }

    if let Retrieved::TerminalSize(column, row) = stdout.get(Value::TerminalSize)? {
        println!("columns: {}, rows: {}", column, row);
    }

    // see '/examples/event.rs'
    if let Retrieved::Event(event) = stdout.get(Value::Event(None))? {
        println!("Event: {:?}\r", event);
    }

    Ok(())
}

fn perform_action() -> error::Result<()> {
    let stdout = stdout();
    stdout.act(Action::MoveCursorTo(10, 10))
}

/// Batches multiple actions before executing.
fn batch_actions() -> error::Result<()> {
    let terminal = stdout();
    terminal.batch(Action::ClearTerminal(Clear::All))?;
    terminal.batch(Action::MoveCursorTo(5, 5))?;

    thread::sleep(Duration::from_millis(2000));
    println!("@");

    terminal.flush_batch()
}

/// Acquires lock once, and uses that lock to do actions.
fn lock_terminal() -> error::Result<()> {
    let terminal = Terminal::custom(File::create("./test.txt").unwrap());

    let mut lock = terminal.lock_mut()?;

    for i in 0..10000 {
        println!("{}", i);

        if i % 100 == 0 {
            lock.act(Action::ClearTerminal(Clear::All))?;
            lock.act(Action::MoveCursorTo(0, 0))?;
        }
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn main() {
    get_value().unwrap();
}
