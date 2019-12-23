use std::io::Write;

use terminal_adapter::{
    stdout, Action, Clear, Event, KeyCode, KeyEvent, Retrieved, TerminalLock, Value,
};

fn main() {
    let terminal = stdout();

    let mut lock = terminal.lock_mut().unwrap();

    lock.act(Action::EnterAlternateScreen).unwrap();
    lock.act(Action::EnableRawMode).unwrap();
    lock.act(Action::HideCursor).unwrap();

    write_alt_screen_msg(&mut lock);

    lock.flush_batch().unwrap();

    loop {
        if let Retrieved::Event(Some(Event::Key(key))) = lock.get(Value::Event(None)).unwrap() {
            match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                } => {
                    break;
                }
                KeyEvent {
                    code: KeyCode::Char('1'),
                    ..
                } => {
                    lock.act(Action::LeaveAlternateScreen).unwrap();
                }
                KeyEvent {
                    code: KeyCode::Char('2'),
                    ..
                } => {
                    lock.act(Action::EnterAlternateScreen).unwrap();
                    write_alt_screen_msg(&mut lock);
                }
                _ => {}
            };
        }
    }

    lock.act(Action::DisableRawMode).unwrap();
    lock.act(Action::ShowCursor).unwrap();
}

fn write_alt_screen_msg<W: Write>(screen: &mut TerminalLock<W>) {
    screen.act(Action::ClearTerminal(Clear::All)).unwrap();
    screen.act(Action::MoveCursorTo(1, 1)).unwrap();

    print!("Welcome to the alternate screen.\n\r");
    screen.act(Action::MoveCursorTo(1, 3)).unwrap();
    print!("Press '1' to switch to the main screen or '2' to switch to the alternate screen.\n\r");
    screen.act(Action::MoveCursorTo(1, 4)).unwrap();
    print!("Press 'q' to exit (and switch back to the main screen).\n\r");
}
