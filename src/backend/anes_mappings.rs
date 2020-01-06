use anes::parser;

use crate::{
    Event, KeyCode, KeyEvent, KeyModifiers, MouseButton,
    MouseEvent,
};

use anes::parser::Sequence;

impl From<parser::MouseButton> for MouseButton {
    fn from(buttons: parser::MouseButton) -> Self {
        match buttons {
            parser::MouseButton::Left => MouseButton::Left,
            parser::MouseButton::Right => MouseButton::Right,
            parser::MouseButton::Middle => MouseButton::Middle,
            parser::MouseButton::Any => MouseButton::Unknown
        }
    }
}

fn from_mouse(event: parser::Mouse, modifiers: KeyModifiers) -> MouseEvent {
    match event {
        parser::Mouse::Down(btn, x, y) => {
            MouseEvent::Up(btn.into(), x, y, modifiers)
        }
        parser::Mouse::Up(btn, x, y) => {
            MouseEvent::Up(btn.into(), x, y, modifiers)
        }
        parser::Mouse::Drag(btn, x, y) => {
            MouseEvent::Drag(btn.into(), x, y, modifiers)
        }
        parser::Mouse::ScrollDown(x, y) => {
            MouseEvent::ScrollUp(x, y, modifiers)
        }
        parser::Mouse::ScrollUp(x, y) => {
            MouseEvent::ScrollDown(x, y, modifiers)
        }
    }
}

impl From<parser::KeyModifiers> for KeyModifiers {
    fn from(modifiers: parser::KeyModifiers) -> Self {
        let shift = modifiers.contains(parser::KeyModifiers::SHIFT);
        let ctrl = modifiers.contains(parser::KeyModifiers::CONTROL);
        let alt = modifiers.contains(parser::KeyModifiers::ALT);

        let mut modifiers = KeyModifiers::empty();

        if shift {
            modifiers |= KeyModifiers::SHIFT;
        }
        if ctrl {
            modifiers |= KeyModifiers::CONTROL;
        }
        if alt {
            modifiers |= KeyModifiers::ALT;
        }

        modifiers
    }
}

impl From<parser::KeyCode> for KeyCode {
    fn from(code: parser::KeyCode) -> Self {
        match code {
            parser::KeyCode::Backspace => KeyCode::Backspace,
            parser::KeyCode::Enter => KeyCode::Enter,
            parser::KeyCode::Left => KeyCode::Left,
            parser::KeyCode::Right => KeyCode::Right,
            parser::KeyCode::Up => KeyCode::Up,
            parser::KeyCode::Down => KeyCode::Down,
            parser::KeyCode::Home => KeyCode::Home,
            parser::KeyCode::End => KeyCode::End,
            parser::KeyCode::PageUp => KeyCode::PageUp,
            parser::KeyCode::PageDown => KeyCode::PageDown,
            parser::KeyCode::Tab => KeyCode::Tab,
            parser::KeyCode::BackTab => KeyCode::BackTab,
            parser::KeyCode::Delete => KeyCode::Delete,
            parser::KeyCode::Insert => KeyCode::Insert,
            parser::KeyCode::F(f) => KeyCode::F(f),
            parser::KeyCode::Char(c) => KeyCode::Char(c),
            parser::KeyCode::Null => KeyCode::Null,
            parser::KeyCode::Esc => KeyCode::Esc,
        }
    }
}

impl From<parser::Sequence> for Event {
    fn from(event: parser::Sequence) -> Self {
        match event {
            parser::Sequence::Key(key, modifiers) => Event::Key(KeyEvent::new(KeyCode::from(key), KeyModifiers::from(modifiers))),
            parser::Sequence::Mouse(mouse, modifiers) => Event::Mouse(from_mouse(mouse, KeyModifiers::from(modifiers))),
            Sequence::CursorPosition(_, _) => { Event::Unknown }
        }
    }
}
