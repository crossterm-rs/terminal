pub use self::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
pub use self::style::{Attribute, Color};
pub use self::terminal::Clear;

mod event;
mod style;
mod terminal;
