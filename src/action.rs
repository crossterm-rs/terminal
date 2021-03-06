use std::time::Duration;

use crate::{Attribute, Clear, Color, Event};

/// A value that can be retrieved from the terminal.
///
/// A [Value](enum.Value.html) can be retrieved with [Terminal::get](struct.Terminal.html#method.get).
pub enum Value {
    /// Get the terminal size.
    TerminalSize,
    /// Get the cursor position.
    CursorPosition,
    /// Try to get an event within the given duration.
    /// The application will wait indefinitely when `None`.
    /// It will wait for some duration if `Some(duration)` is given.
    Event(Option<Duration>),
}

/// A result that is returned from a request for a [Value](enum.Value.html).
///
/// A [Value](enum.Value.html) can be retrieved with [Terminal::get](struct.Terminal.html#method.get).
pub enum Retrieved {
    /// The terminal size is returned number of (column, row)s.
    TerminalSize(u16, u16),
    /// The cursor position is returned (column, row).
    /// The top left cell is represented 0,0.
    CursorPosition(u16, u16),
    /// An event is returned.
    /// Timeout occurred if `None` is returned.
    Event(Option<Event>),
}

/// An action that can be performed on the terminal.
///
/// To perform an [Action](enum.Action.html) use [Terminal::act](struct.Terminal.html#method.act).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Action {
    /// Moves the terminal cursor to the given position (column, row).
    MoveCursorTo(u16, u16),
    /// Hides the terminal cursor.
    HideCursor,
    /// Shows the terminal cursor.
    ShowCursor,
    /// Enables blinking of the terminal cursor.
    EnableBlinking,
    /// Disables blinking of the terminal cursor.
    DisableBlinking,
    /// Clears the terminal screen buffer.
    ClearTerminal(Clear),
    /// Sets the terminal size (columns, rows).
    SetTerminalSize(u16, u16),
    /// Scrolls the terminal screen a given number of rows up.
    ScrollUp(u16),
    /// Scrolls the terminal screen a given number of rows down.
    ScrollDown(u16),

    /// Enables raw mode.
    EnableRawMode,
    /// Disables raw mode.
    DisableRawMode,
    /// Switches to alternate screen.
    EnterAlternateScreen,
    /// Switches back to the main screen.
    LeaveAlternateScreen,

    /// Enables mouse event capturing.
    EnableMouseCapture,
    /// Disables mouse event capturing.
    DisableMouseCapture,

    /// Sets the the foreground color.
    SetForegroundColor(Color),
    /// Sets the the background color.
    SetBackgroundColor(Color),
    /// Sets an attribute.
    SetAttribute(Attribute),
    /// Resets the colors back to default.
    ResetColor,
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        format!("{:?}", action)
    }
}
