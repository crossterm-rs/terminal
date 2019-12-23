use bitflags::bitflags;

/// Represents an event.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Event {
    /// A single key event with additional pressed modifiers.
    Key(KeyEvent),
    /// A singe mouse event with additional pressed modifiers.
    Mouse(MouseEvent),
    /// An resize event with new dimensions after resize (columns, rows).
    Resize(u16, u16),
    /// An event was not supported by the backend.
    Unknown,
}

/// Represents a mouse event.
///
/// # Platform-specific Notes
///
/// ## Mouse Buttons
///
/// Some platforms/terminals do not report mouse button for the
/// `MouseEvent::Up` and `MouseEvent::Drag` events. `MouseButton::Left`
/// is returned if we don't know which button was used.
///
/// ## Key Modifiers
///
/// Some platforms/terminals does not report all key modifiers
/// combinations for all mouse event types. For example - macOS reports
/// `Ctrl` + left mouse button click as a right mouse button click.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseEvent {
    /// Pressed mouse button.
    ///
    /// Contains mouse button, pressed pointer location (column, row), and additional key modifiers.
    Down(MouseButton, u16, u16, KeyModifiers),
    /// Released mouse button.
    ///
    /// Contains mouse button, released pointer location (column, row), and additional key modifiers.
    Up(MouseButton, u16, u16, KeyModifiers),
    /// Moved mouse pointer while pressing a mouse button.
    ///
    /// Contains the pressed mouse button, released pointer location (column, row), and additional key modifiers.
    Drag(MouseButton, u16, u16, KeyModifiers),
    /// Scrolled mouse wheel downwards (towards the user).
    ///
    /// Contains the scroll location (column, row), and additional key modifiers.
    ScrollDown(u16, u16, KeyModifiers),
    /// Scrolled mouse wheel upwards (away from the user).
    ///
    /// Contains the scroll location (column, row), and additional key modifiers.
    ScrollUp(u16, u16, KeyModifiers),
}

/// Represents a mouse button.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// An mouse button was not supported by the backend.
    Unknown,
}

bitflags! {
    /// Represents key modifiers (shift, control, alt).
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
    }
}

/// Represents a key event.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub struct KeyEvent {
    /// The key itself.
    pub code: KeyCode,
    /// Additional key modifiers.
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent { code, modifiers }
    }
}

impl From<KeyCode> for KeyEvent {
    fn from(code: KeyCode) -> Self {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
        }
    }
}

/// Represents a key.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyCode {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page dow key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyEvent::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyEvent::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
}
