use std::{io, io::Write};

use crossterm::{
    cursor, event, style, terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};

use crate::{backend::Backend, error, error::ErrorKind, Action, Event, Retrieved, Value};

pub struct BackendImpl<W: Write> {
    // The internal buffer on which operations are performed and written to.
    buffer: W,
}

impl<W: Write> Backend<W> for BackendImpl<W> {
    fn create(buffer: W) -> BackendImpl<W> {
        BackendImpl { buffer }
    }

    fn act(&mut self, action: Action) -> error::Result<()> {
        self.batch(action)?;
        self.flush_batch()
    }

    #[allow(clippy::cognitive_complexity)]
    fn batch(&mut self, action: Action) -> error::Result<()> {
        let buffer = &mut self.buffer;

        let _ = match action {
            Action::MoveCursorTo(column, row) => buffer.queue(cursor::MoveTo(column, row))?,
            Action::HideCursor => buffer.queue(cursor::Hide)?,
            Action::ShowCursor => buffer.queue(cursor::Show)?,
            Action::EnableBlinking => buffer.queue(cursor::EnableBlinking)?,
            Action::DisableBlinking => buffer.queue(cursor::DisableBlinking)?,
            Action::ClearTerminal(clear_type) => {
                buffer.queue(terminal::Clear(terminal::ClearType::from(clear_type)))?
            }
            Action::SetTerminalSize(column, row) => buffer.queue(terminal::SetSize(column, row))?,
            Action::ScrollUp(rows) => buffer.queue(terminal::ScrollUp(rows))?,
            Action::ScrollDown(rows) => buffer.queue(terminal::ScrollDown(rows))?,
            Action::EnterAlternateScreen => {
                buffer.queue(terminal::EnterAlternateScreen)?;
                buffer
            }
            Action::LeaveAlternateScreen => {
                buffer.queue(terminal::LeaveAlternateScreen)?;
                buffer
            }
            Action::SetForegroundColor(color) => {
                buffer.queue(style::SetForegroundColor(style::Color::from(color)))?
            }
            Action::SetBackgroundColor(color) => {
                buffer.queue(style::SetBackgroundColor(style::Color::from(color)))?
            }
            Action::SetAttribute(attr) => {
                buffer.queue(style::SetAttribute(style::Attribute::from(attr)))?
            }
            Action::ResetColor => buffer.queue(style::ResetColor)?,
            Action::EnableRawMode => {
                enable_raw_mode()?;
                return Ok(());
            }
            Action::DisableRawMode => {
                disable_raw_mode()?;
                return Ok(());
            }
            Action::EnableMouseCapture => buffer.queue(event::EnableMouseCapture)?,
            Action::DisableMouseCapture => buffer.queue(event::DisableMouseCapture)?,
        };

        Ok(())
    }

    fn flush_batch(&mut self) -> error::Result<()> {
        self.buffer
            .flush()
            .map_err(|_| ErrorKind::FlushingBatchFailed)
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
        io::stdout()
            .execute(terminal::LeaveAlternateScreen)
            .unwrap();
        disable_raw_mode().unwrap();
        io::stdout().execute(event::DisableMouseCapture).unwrap();
    }
}

impl<W: Write> Write for BackendImpl<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.buffer.flush()
    }
}
