use termion::{event, event::Key};

use crate::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};

impl From<event::MouseButton> for MouseButton {
    fn from(buttons: event::MouseButton) -> Self {
        match buttons {
            event::MouseButton::Left => MouseButton::Left,
            event::MouseButton::Right => MouseButton::Right,
            event::MouseButton::Middle => MouseButton::Middle,
            _ => { unreachable!("Wheel up and down are handled at MouseEvent level. Code should not be able to reach this.") }
        }
    }
}

fn to_0_based(x: u16, y: u16) -> (u16, u16) {
    // to 0-based position.
    (x - 1, y - 1)
}

impl From<event::MouseEvent> for MouseEvent {
    fn from(event: event::MouseEvent) -> Self {
        match event {
            event::MouseEvent::Press(btn, x, y) => {
                // to 0-based position.
                let (x, y) = to_0_based(x, y);

                if btn == event::MouseButton::WheelDown {
                    MouseEvent::ScrollDown(x, y, KeyModifiers::empty())
                } else if btn == event::MouseButton::WheelUp {
                    MouseEvent::ScrollUp(x, y, KeyModifiers::empty())
                } else {
                    MouseEvent::Down(btn.into(), x, y, KeyModifiers::empty())
                }
            }
            event::MouseEvent::Release(x, y) => {
                // to 0-based position.
                let (x, y) = to_0_based(x, y);

                MouseEvent::Up(MouseButton::Unknown, x, y, KeyModifiers::empty())
            }
            event::MouseEvent::Hold(x, y) => {
                // to 0-based position.
                let (x, y) = to_0_based(x, y);

                MouseEvent::Drag(MouseButton::Unknown, x, y, KeyModifiers::empty())
            }
        }
    }
}

impl From<event::Key> for KeyEvent {
    fn from(code: event::Key) -> Self {
        match code {
            event::Key::Backspace => KeyCode::Backspace.into(),
            event::Key::Left => KeyCode::Left.into(),
            event::Key::Right => KeyCode::Right.into(),
            event::Key::Up => KeyCode::Up.into(),
            event::Key::Down => KeyCode::Down.into(),
            event::Key::Home => KeyCode::Home.into(),
            event::Key::End => KeyCode::End.into(),
            event::Key::PageUp => KeyCode::PageUp.into(),
            event::Key::PageDown => KeyCode::PageDown.into(),
            event::Key::BackTab => KeyCode::BackTab.into(),
            event::Key::Delete => KeyCode::Delete.into(),
            event::Key::Insert => KeyCode::Insert.into(),
            event::Key::F(f) => KeyCode::F(f).into(),
            event::Key::Char('\n') => KeyCode::Enter.into(),
            event::Key::Char('\t') => KeyCode::Tab.into(),
            event::Key::Char(c) => KeyCode::Char(c).into(),
            event::Key::Null => KeyCode::Null.into(),
            event::Key::Esc => KeyCode::Esc.into(),

            Key::Alt(char) => {
                let mut modifiers = KeyModifiers::empty();
                modifiers |= KeyModifiers::ALT;

                KeyEvent::new(KeyCode::Char(char), modifiers)
            }
            Key::Ctrl(char) => {
                let mut modifiers = KeyModifiers::empty();
                modifiers |= KeyModifiers::CONTROL;

                KeyEvent::new(KeyCode::Char(char), modifiers)
            }

            Key::__IsNotComplete => KeyCode::Tab.into(),
        }
    }
}

impl From<event::Event> for Event {
    fn from(event: event::Event) -> Self {
        match event {
            event::Event::Key(key) => Event::Key(KeyEvent::from(key)),
            event::Event::Mouse(mouse) => Event::Mouse(MouseEvent::from(mouse)),
            event::Event::Unsupported(_data) => Event::Unknown,
        }
    }
}
