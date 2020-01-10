pub use self::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent},
    style::{Attribute, Color},
    terminal::Clear,
};

mod event;
mod style;
mod terminal;
