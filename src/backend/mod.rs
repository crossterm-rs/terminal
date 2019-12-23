use std::io::Write;

use crate::{error, Action, Result, Value};

#[cfg(feature = "crossterm-backend")]
pub(crate) use self::crossterm::BackendImpl;
#[cfg(feature = "termion-backend")]
pub(crate) use self::termion::BackendImpl;

#[cfg(feature = "crossterm-backend")]
mod crossterm;

#[cfg(feature = "termion-backend")]
mod termion;

#[cfg(feature = "termion-backend")]
mod resize;

/// Interface to an backend library.
pub trait Backend<W: Write> {
    fn create() -> Self;
    fn act(&mut self, action: Action, buffer: &mut W) -> error::Result<()>;
    fn batch(&mut self, action: Action, buffer: &mut W) -> error::Result<()>;
    fn flush_batch(&mut self, buffer: &mut W) -> error::Result<()>;
    fn get(&self, retrieve_operation: Value) -> error::Result<Result>;
}
