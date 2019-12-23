use std::io;
use std::io::Write;

use bitflags::_core::marker::PhantomData;
use crossterm::{
    cursor, event, style, terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};

use crate::{backend::Backend, error, error::ErrorKind, Action, Event, Retrieved, Value};

pub struct BackendImpl<W: Write> {
    _phantom: PhantomData<W>,
}

impl<W: Write> BackendImpl<W> {
    fn map_error<E>(&self, result: crossterm::Result<E>) -> error::Result<()> {
        if let Ok(_) = result {
            Ok(())
        } else {
            Err(ErrorKind::FlushingBatchFailed)
        }
    }
}

impl<W: Write> Backend<W> for BackendImpl<W> {
    fn create() -> BackendImpl<W> {
        BackendImpl {
            _phantom: PhantomData,
        }
    }

    fn act(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
        self.batch(action, buffer)?;
        self.flush_batch(buffer)
    }

    fn batch(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
        let result = match action {
            Action::MoveCursorTo(column, row) => buffer.queue(cursor::MoveTo(column, row)),
            Action::HideCursor => buffer.queue(cursor::Hide),
            Action::ShowCursor => buffer.queue(cursor::Show),
            Action::EnableBlinking => buffer.queue(cursor::EnableBlinking),
            Action::DisableBlinking => buffer.queue(cursor::DisableBlinking),
            Action::ClearTerminal(clear_type) => {
                buffer.queue(terminal::Clear(terminal::ClearType::from(clear_type)))
            }
            Action::SetTerminalSize(column, row) => buffer.queue(terminal::SetSize(column, row)),
            Action::ScrollUp(rows) => buffer.queue(terminal::ScrollUp(rows)),
            Action::ScrollDown(rows) => buffer.queue(terminal::ScrollDown(rows)),
            Action::EnterAlternateScreen => buffer.queue(terminal::EnterAlternateScreen),
            Action::LeaveAlternateScreen => buffer.queue(terminal::LeaveAlternateScreen),
            Action::SetForegroundColor(color) => {
                buffer.queue(style::SetForegroundColor(style::Color::from(color)))
            }
            Action::SetBackgroundColor(color) => {
                buffer.queue(style::SetBackgroundColor(style::Color::from(color)))
            }
            Action::SetAttribute(attr) => {
                buffer.queue(style::SetAttribute(style::Attribute::from(attr)))
            }
            Action::ResetColor => buffer.queue(style::ResetColor),
            Action::EnableRawMode => {
                enable_raw_mode()?;
                Ok(buffer)
            }
            Action::DisableRawMode => {
                disable_raw_mode()?;
                Ok(buffer)
            }
            Action::EnableMouseCapture => buffer.queue(event::EnableMouseCapture),
            Action::DisableMouseCapture => buffer.queue(event::DisableMouseCapture),
        };

        self.map_error(result)
    }

    fn flush_batch(&mut self, buffer: &mut W) -> error::Result<()> {
        buffer.flush().map_err(|_| ErrorKind::FlushingBatchFailed)
    }

    fn get(&self, retrieve_operation: Value) -> error::Result<Retrieved> {
        Ok(match retrieve_operation {
            Value::TerminalSize => {
                let size = terminal::size()?;
                Retrieved::TerminalSize(size.0, size.1)
            }
            Value::CursorPosition => {
                let position = cursor::position()?;
                Retrieved::CursorPosition(position.0, position.1)
            }
            Value::Event(duration) => {
                if let Some(duration) = duration {
                    if event::poll(duration)? {
                        let event = event::read()?;
                        Retrieved::Event(Some(Event::from(event)))
                    } else {
                        Retrieved::Event(None)
                    }
                } else {
                    let event = event::read()?;
                    Retrieved::Event(Some(Event::from(event)))
                }
            }
        })
    }
}

impl<W: Write> Drop for BackendImpl<W> {
    fn drop(&mut self) {
        let _ = io::stdout().execute(terminal::LeaveAlternateScreen);
        let _ = disable_raw_mode().unwrap();
        let _ = io::stdout().execute(event::DisableMouseCapture);
    }
}
