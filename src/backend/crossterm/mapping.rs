use std::{error::Error, io};

use crossterm::{event, style, terminal};

use crate::{
    error::ErrorKind, Attribute, ClearType, Color, Event, KeyCode, KeyEvent, KeyModifiers,
    MouseButton, MouseEvent,
};

impl From<Attribute> for style::Attribute {
    fn from(attribute: Attribute) -> Self {
        match attribute {
            Attribute::Reset => style::Attribute::Reset,
            Attribute::Bold => style::Attribute::Bold,
            Attribute::BoldItalicOff => style::Attribute::Dim,
            Attribute::Italic => style::Attribute::Italic,
            Attribute::Underlined => style::Attribute::Underlined,
            Attribute::SlowBlink => style::Attribute::SlowBlink,
            Attribute::RapidBlink => style::Attribute::RapidBlink,
            Attribute::Reversed => style::Attribute::Reverse,
            Attribute::Conceal => style::Attribute::Hidden,
            Attribute::Crossed => style::Attribute::CrossedOut,
            Attribute::Fraktur => style::Attribute::Fraktur,
            Attribute::BoldOff => style::Attribute::NoBold,
            Attribute::NormalIntensity => style::Attribute::NormalIntensity,
            Attribute::ItalicOff => style::Attribute::NoItalic,
            Attribute::UnderlinedOff => style::Attribute::NoUnderline,
            Attribute::BlinkOff => style::Attribute::NoBlink,
            Attribute::ReversedOff => style::Attribute::NoReverse,
            Attribute::ConcealOff => style::Attribute::NoHidden,
            Attribute::CrossedOff => style::Attribute::NotCrossedOut,
            Attribute::Framed => style::Attribute::Framed,
            Attribute::__Nonexhaustive => style::Attribute::__Nonexhaustive,
        }
    }
}

impl From<Color> for style::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => style::Color::Reset,
            Color::Black => style::Color::Black,
            Color::DarkGrey => style::Color::DarkGrey,
            Color::Red => style::Color::Red,
            Color::DarkRed => style::Color::DarkRed,
            Color::Green => style::Color::Green,
            Color::DarkGreen => style::Color::DarkGreen,
            Color::Yellow => style::Color::Yellow,
            Color::DarkYellow => style::Color::DarkYellow,
            Color::Blue => style::Color::Blue,
            Color::DarkBlue => style::Color::DarkBlue,
            Color::Magenta => style::Color::Magenta,
            Color::DarkMagenta => style::Color::DarkMagenta,
            Color::Cyan => style::Color::Cyan,
            Color::DarkCyan => style::Color::DarkCyan,
            Color::White => style::Color::White,
            Color::Grey => style::Color::Grey,
            Color::Rgb(r, g, b) => style::Color::Rgb { r, g, b },
            Color::AnsiValue(val) => style::Color::AnsiValue(val),
        }
    }
}

impl From<ClearType> for terminal::ClearType {
    fn from(clear_type: ClearType) -> Self {
        match clear_type {
            ClearType::All => terminal::ClearType::All,
            ClearType::FromCursorDown => terminal::ClearType::FromCursorDown,
            ClearType::FromCursorUp => terminal::ClearType::FromCursorUp,
            ClearType::CurrentLine => terminal::ClearType::CurrentLine,
            ClearType::UntilNewLine => terminal::ClearType::UntilNewLine,
        }
    }
}

impl From<event::MouseButton> for MouseButton {
    fn from(buttons: event::MouseButton) -> Self {
        match buttons {
            event::MouseButton::Left => MouseButton::Left,
            event::MouseButton::Right => MouseButton::Right,
            event::MouseButton::Middle => MouseButton::Middle,
        }
    }
}

impl From<event::MouseEvent> for MouseEvent {
    fn from(event: event::MouseEvent) -> Self {
        match event {
            event::MouseEvent::Down(btn, x, y, modifiers) => {
                MouseEvent::Up(btn.into(), x, y, modifiers.into())
            }
            event::MouseEvent::Up(btn, x, y, modifiers) => {
                MouseEvent::Up(btn.into(), x, y, modifiers.into())
            }
            event::MouseEvent::Drag(btn, x, y, modifiers) => {
                MouseEvent::Drag(btn.into(), x, y, modifiers.into())
            }
            event::MouseEvent::ScrollDown(x, y, modifiers) => {
                MouseEvent::ScrollUp(x, y, modifiers.into())
            }
            event::MouseEvent::ScrollUp(x, y, modifiers) => {
                MouseEvent::ScrollDown(x, y, modifiers.into())
            }
        }
    }
}

impl From<event::KeyModifiers> for KeyModifiers {
    fn from(modifiers: event::KeyModifiers) -> Self {
        let shift = modifiers.contains(event::KeyModifiers::SHIFT);
        let ctrl = modifiers.contains(event::KeyModifiers::CONTROL);
        let alt = modifiers.contains(event::KeyModifiers::ALT);

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

        return modifiers;
    }
}

impl From<event::KeyCode> for KeyCode {
    fn from(code: event::KeyCode) -> Self {
        match code {
            event::KeyCode::Backspace => KeyCode::Backspace,
            event::KeyCode::Enter => KeyCode::Enter,
            event::KeyCode::Left => KeyCode::Left,
            event::KeyCode::Right => KeyCode::Right,
            event::KeyCode::Up => KeyCode::Up,
            event::KeyCode::Down => KeyCode::Down,
            event::KeyCode::Home => KeyCode::Home,
            event::KeyCode::End => KeyCode::End,
            event::KeyCode::PageUp => KeyCode::PageUp,
            event::KeyCode::PageDown => KeyCode::PageDown,
            event::KeyCode::Tab => KeyCode::Tab,
            event::KeyCode::BackTab => KeyCode::BackTab,
            event::KeyCode::Delete => KeyCode::Delete,
            event::KeyCode::Insert => KeyCode::Insert,
            event::KeyCode::F(f) => KeyCode::F(f),
            event::KeyCode::Char(c) => KeyCode::Char(c),
            event::KeyCode::Null => KeyCode::Null,
            event::KeyCode::Esc => KeyCode::Esc,
        }
    }
}

impl From<event::KeyEvent> for KeyEvent {
    fn from(event: event::KeyEvent) -> Self {
        KeyEvent {
            code: KeyCode::from(event.code),
            modifiers: KeyModifiers::from(event.modifiers),
        }
    }
}

impl From<event::Event> for Event {
    fn from(event: event::Event) -> Self {
        match event {
            event::Event::Key(key) => Event::Key(KeyEvent::from(key)),
            event::Event::Mouse(mouse) => Event::Mouse(MouseEvent::from(mouse)),
            event::Event::Resize(x, y) => Event::Resize,
        }
    }
}

impl From<crossterm::ErrorKind> for ErrorKind {
    fn from(error: crossterm::ErrorKind) -> Self {
        match error {
            crossterm::ErrorKind::IoError(e) => ErrorKind::IoError(e),
            e => ErrorKind::IoError(io::Error::new(io::ErrorKind::Other, e.description())),
        }
    }
}
