use std::marker::PhantomData;
use std::{
    fmt,
    fmt::{Display, Formatter},
    fs::File,
    io,
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use crossbeam_channel::{select, unbounded, Receiver};
use termion::{
    clear, color, cursor, get_tty,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen, style, terminal_size,
};

use crate::{backend::{resize, termion::cursor::position, Backend}, error, error::ErrorKind, Action, Attribute, Color, Event, Result, Value, ClearType};

/// A sequence of escape codes to enable terminal mouse support.
/// We use this directly instead of using `MouseTerminal` from termion.
const ENABLE_MOUSE_CAPTURE: &'static str = "\x1B[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h";

/// A sequence of escape codes to disable terminal mouse support.
/// We use this directly instead of using `MouseTerminal` from termion.
const DISABLE_MOUSE_CAPTURE: &'static str = "\x1B[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l";

/// Writer which writes either an foreground or background color escape code to the formatter.
struct ColorCodeWriter<T: color::Color> {
    color: T,
    is_fg: bool,
}

impl<T: color::Color> ColorCodeWriter<T> {
    pub fn new(color: T, is_fg: bool) -> ColorCodeWriter<T> {
        ColorCodeWriter { color, is_fg }
    }
}

impl<T: color::Color> Display for ColorCodeWriter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_fg {
            self.color.write_fg(f)
        } else {
            self.color.write_bg(f)
        }
    }
}

pub struct BackendImpl<W: Write> {
    _phantom: PhantomData<W>,

    // Write operations are forwarded to this type when raw mode is enabled.
    // termion wraps raw mode in an struct which requires owner ship of the buffer.
    // We can't give ownership to the buffer, because it is owned by `Terminal`.
    // Also we can't change the buffer type to `RawTerminal` at run time because of the generic type.
    buffer: Option<Box<RawTerminal<File>>>,

    input_receiver: Option<Receiver<Event>>,
    resize_receiver: Option<Receiver<()>>,

    is_raw_mode_enabled: bool,
}

impl<W: Write> BackendImpl<W> {
    /// Write the given color to the given buffer.
    pub fn w_color<T: color::Color>(
        &mut self,
        color: T,
        is_fg: bool,
        buffer: &mut W,
    ) -> io::Result<()> {
        if let Some(ref mut terminal) = self.buffer {
            write!(terminal, "{}", ColorCodeWriter::new(color, is_fg))
        } else {
            write!(buffer, "{}", ColorCodeWriter::new(color, is_fg))
        }
    }

    /// Format the given color and write it to the given buffer.
    pub fn f_color<'a>(&mut self, color: Color, is_fg: bool, buffer: &mut W) -> io::Result<()> {
        match color {
            Color::Reset => self.w_color(color::Reset, is_fg, buffer),
            Color::Black => self.w_color(color::Black, is_fg, buffer),
            Color::DarkGrey => self.w_color(color::Black, is_fg, buffer),
            Color::Red => self.w_color(color::LightRed, is_fg, buffer),
            Color::DarkRed => self.w_color(color::Red, is_fg, buffer),
            Color::Green => self.w_color(color::LightGreen, is_fg, buffer),
            Color::DarkGreen => self.w_color(color::Green, is_fg, buffer),
            Color::Yellow => self.w_color(color::LightYellow, is_fg, buffer),
            Color::DarkYellow => self.w_color(color::Yellow, is_fg, buffer),
            Color::Blue => self.w_color(color::LightBlue, is_fg, buffer),
            Color::DarkBlue => self.w_color(color::Blue, is_fg, buffer),
            Color::Magenta => self.w_color(color::LightMagenta, is_fg, buffer),
            Color::DarkMagenta => self.w_color(color::Magenta, is_fg, buffer),
            Color::Cyan => self.w_color(color::LightCyan, is_fg, buffer),
            Color::DarkCyan => self.w_color(color::Cyan, is_fg, buffer),
            Color::White => self.w_color(color::LightWhite, is_fg, buffer),
            Color::Grey => self.w_color(color::LightWhite, is_fg, buffer),
            Color::Rgb(r, g, b) => self.w_color(color::Rgb(r, g, b), is_fg, buffer),
            Color::AnsiValue(val) => self.w_color(color::AnsiValue(val), is_fg, buffer),
        }
    }

    /// Write displayable type to the given buffer.
    pub fn w_display(&mut self, displayable: &dyn Display, buffer: &mut W) -> io::Result<()> {
        if let Some(ref mut terminal) = self.buffer {
            write!(terminal, "{}", displayable)
        } else {
            write!(buffer, "{}", displayable)
        }
    }

    /// Format the given attribute and write it to the given buffer.
    pub fn f_attribute(&mut self, attribute: Attribute, buffer: &mut W) -> error::Result<()> {
        match attribute {
            Attribute::SlowBlink => self.w_display(&style::Blink, buffer)?,
            Attribute::RapidBlink => self.w_display(&style::Blink, buffer)?,
            Attribute::BlinkOff => self.w_display(&style::NoBlink, buffer)?,

            Attribute::Bold => self.w_display(&style::Bold, buffer)?,
            Attribute::BoldOff => self.w_display(&style::NoBold, buffer)?,

            Attribute::Crossed => self.w_display(&style::CrossedOut, buffer)?,
            Attribute::CrossedOff => self.w_display(&style::NoCrossedOut, buffer)?,

            Attribute::BoldItalicOff => self.w_display(&style::Faint, buffer)?,

            Attribute::Framed => self.w_display(&style::Framed, buffer)?,

            Attribute::Reversed => self.w_display(&style::Invert, buffer)?,
            Attribute::ReversedOff => self.w_display(&style::NoInvert, buffer)?,

            Attribute::Italic => self.w_display(&style::Italic, buffer)?,
            Attribute::ItalicOff => self.w_display(&style::NoItalic, buffer)?,

            Attribute::Underlined => self.w_display(&style::Underline, buffer)?,
            Attribute::UnderlinedOff => self.w_display(&style::NoUnderline, buffer)?,

            Attribute::Reset => self.w_display(&style::Reset, buffer)?,
            _ => {
                // ConcealOff, ConcealOff, Fraktur, NormalIntensity not supported.
                Err(error::ErrorKind::AttributeNotSupported(String::from(
                    attribute,
                )))?
            }
        };

        Ok(())
    }
}

impl<W: Write> Backend<W> for BackendImpl<W> {
    fn create() -> Self {
        let (input_sender, input_receiver) = unbounded::<Event>();
        let (resize_sender, resize_receiver) = unbounded();

        let running = Arc::new(AtomicBool::new(true));

        #[cfg(unix)]
        resize::start_resize_thread(resize_sender, Arc::clone(&running));

        // termion is blocking by default, read input from a separate thread.
        thread::spawn(move || {
            let input = termion::get_tty().unwrap();
            let mut events = input.events();

            while let Some(Ok(event)) = events.next() {
                // If we can't send, then receiving side closed, stop thread.
                if input_sender.send(Event::from(event)).is_err() {
                    break;
                }
            }

            running.store(false, Ordering::Relaxed);
        });

        BackendImpl {
            _phantom: PhantomData,
            buffer: None,
            resize_receiver: Some(resize_receiver),
            input_receiver: Some(input_receiver),
            is_raw_mode_enabled: false,
        }
    }

    fn act(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
        self.batch(action, buffer)?;
        self.flush_batch(buffer)
    }

    fn batch(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
        match action {
            Action::MoveCursorTo(column, row) => {
                self.w_display(&cursor::Goto(column + 1, row + 1), buffer)?
            }
            Action::HideCursor => self.w_display(&cursor::Hide, buffer)?,
            Action::ShowCursor => self.w_display(&cursor::Show, buffer)?,
            Action::ClearTerminal(clear_type) => {
                match clear_type {
                    ClearType::All => {self.w_display(&clear::All, buffer)?;},
                    ClearType::FromCursorDown => self.w_display(&clear::AfterCursor, buffer)?,
                    ClearType::FromCursorUp => self.w_display(&clear::BeforeCursor, buffer)?,
                    ClearType::CurrentLine => self.w_display(&clear::CurrentLine, buffer)?,
                    ClearType::UntilNewLine => self.w_display(&clear::UntilNewline, buffer)?,
                }
            },
            Action::EnterAlternateScreen => self.w_display(&screen::ToAlternateScreen, buffer)?,
            Action::LeaveAlternateScreen => self.w_display(&screen::ToMainScreen, buffer)?,
            Action::SetForegroundColor(color) => self.f_color(color, true, buffer)?,
            Action::SetBackgroundColor(color) => self.f_color(color, false, buffer)?,
            Action::SetAttribute(attr) => self.f_attribute(attr, buffer)?,
            Action::ResetColor => self.w_display(
                &format!("{}{}", color::Reset.fg_str(), color::Reset.bg_str()),
                buffer,
            )?,
            Action::EnableRawMode => {
                self.buffer = Some(Box::new(termion::get_tty()?.into_raw_mode().unwrap()));
                self.is_raw_mode_enabled = true;
            }
            Action::DisableRawMode => {
                if let Some(_) = &self.buffer {
                    self.buffer = None;
                    self.is_raw_mode_enabled = false;
                }
            }
            Action::EnableMouseCapture => {
                buffer.write(ENABLE_MOUSE_CAPTURE.as_bytes())?;
            }
            Action::DisableMouseCapture => {
                buffer.write(DISABLE_MOUSE_CAPTURE.as_bytes())?;
            }
            _ => {
                // ScrollUp, ScrollDown, SetTerminalSize, EnableBlinking, DisableBlinking are not supported.
                Err(error::ErrorKind::ActionNotSupported(String::from(action)))?
            }
        };

        self.flush_batch(buffer)
    }

    fn flush_batch(&mut self, buffer: &mut W) -> error::Result<()> {
        buffer.flush().map_err(|_| ErrorKind::FlushingBatchFailed)
    }

    fn get(&self, retrieve_operation: Value) -> error::Result<Result> {
        Ok(match retrieve_operation {
            Value::TerminalSize => {
                let size = terminal_size()?;
                Result::TerminalSize(size.0, size.1)
            }
            Value::CursorPosition => {
                // if raw mode is disabled, we need to enable and disable it.
                // Otherwise the position is written to the console window.
                let (x, y) = if self.is_raw_mode_enabled {
                    position()?
                } else {
                    get_tty()?.into_raw_mode()?;
                    position()?
                };

                Result::CursorPosition(x, y)
            }
            Value::Event(duration) => {
                if let Some(ref input_receiver) = self.input_receiver {
                    if let Some(ref resize_receiver) = self.resize_receiver {
                        let event = if let Some(duration) = duration {
                            select! {
                               recv(input_receiver) -> event => event.ok(),
                               recv(resize_receiver) -> _ => Some(Event::Resize),
                               default(duration) => None,
                            }
                        } else {
                            select! {
                               recv(input_receiver) -> event => event.ok(),
                               recv(resize_receiver) -> _ => Some(Event::Resize),
                            }
                        };
                        return Ok(event.map_or(Result::Event(None), |event| {
                            Result::Event(Some(Event::from(event)))
                        }));
                    };
                };

                Result::Event(None)
            }
        })
    }
}
