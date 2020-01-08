/// A mask that can be used to track all mouse events.
const MOUSE_EVENT_MASK: u32 = pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION;

/// A sequence of escape codes to enable terminal mouse support.
/// We use this directly instead of using `MouseTerminal` from termion.
const ENABLE_MOUSE_CAPTURE: &'static str = "\x1B[?1002h";

/// A sequence of escape codes to disable terminal mouse support.
/// We use this directly instead of using `MouseTerminal` from termion.
const DISABLE_MOUSE_CAPTURE: &'static str = "\x1B[?1002l";
