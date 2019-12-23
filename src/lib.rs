#![deny(unused_imports, unused_must_use)]

pub use self::{
    action::{Action, Retrieved, Value},
    enums::{
        Attribute, Clear, Color, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent,
    },
    terminal::{stderr, stdout, Terminal, TerminalLock},
};

pub mod error;

pub(crate) mod action;
pub(crate) mod backend;
pub(crate) mod enums;
pub(crate) mod terminal;
