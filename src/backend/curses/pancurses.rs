use std::marker::PhantomData;
use std::io::Write;
use crate::backend::Backend;
use crate::{Action, error, Value, Retrieved, MouseButton, Event, Clear, Attribute, Color};
use std::io;
use std::sync::{RwLock};
use pancurses::COLORS;
use std::collections::HashMap;
use crate::backend::curses::mapping::find_closest;

const MOUSE_EVENT_MASK: u32 = pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION;

pub struct BackendImpl<W: Write> {
    _phantom: PhantomData<W>,
    window: pancurses::Window,

    last_mouse_button: Option<RwLock<MouseButton>>,
    input_buffer: Option<RwLock<Event>>,

    // ncurses stores color values in pairs (fg, bg) color.
    // We store those pairs in this hashmap on order to keep track of the pairs we initialized.
    color_pairs: HashMap<(i16, i16), i32>
}

impl<W: Write> BackendImpl<W> {
    pub fn update_input_buffer(&self, btn: Event) {
        self.input_buffer.as_ref().map(|lock| {
            let mut write = lock.write().unwrap();
            *write = btn;
        });
    }

    pub fn get_input_buffer(&self) -> Option<Event> {
        self.input_buffer.as_ref().map(|lock| {
            let read = lock.read().unwrap();
            *read
        })
    }

    pub fn update_last_btn(&self, btn: MouseButton) {
        self.last_mouse_button.as_ref().map(|lock| {
            let mut write = lock.write().unwrap();
            *write = btn;
        });
    }

    pub fn get_last_btn(&self) -> Option<MouseButton> {
        self.last_mouse_button.as_ref().map(|lock| {
            let read = lock.read().unwrap();
            *read
        })
    }
}

impl<W: Write> Backend<W> for BackendImpl<W> {
    fn create() -> Self {
        let window = pancurses::initscr();

        window.keypad(true);
        window.timeout(0);
        pancurses::start_color();
        pancurses::use_default_colors();

        // This asks the terminal to provide us with mouse drag events
        // (Mouse move when a button is pressed).
        // Replacing 1002 with 1003 would give us ANY mouse move.
        print!("\x1B[?1002h");
        io::stdout().flush().unwrap();

        BackendImpl {
            _phantom: PhantomData,
            window,
            last_mouse_button: None,
            input_buffer: None,
            color_pairs: HashMap::new()
        }
    }

    fn act(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
        unimplemented!()
    }

    fn batch(&mut self, action: Action, buffer: &mut W) -> error::Result<()> {
        match action {
            Action::MoveCursorTo(x, y) => { self.window.mv(x as i32, y as i32) },
            Action::HideCursor => pancurses::curs_set(0) as i32,
            Action::ShowCursor => pancurses::curs_set(1) as i32,
            Action::EnableBlinking =>  pancurses::set_blink(true),
            Action::DisableBlinking =>   pancurses::set_blink(false),
            Action::ClearTerminal(clear_type) => {
                match clear_type {
                    Clear::All => self.window.clear(),
                    Clear::FromCursorDown => self.window.clrtobot(),
                    Clear::UntilNewLine => self.window.clrtoeol(),
                    Clear::FromCursorUp => 0, // TODO, not supported by pancurses
                    Clear::CurrentLine => 0, // TODO, not supported by pancurses
                };

                0
            }
            Action::SetTerminalSize(cols, rows) => { pancurses::resize_term(rows as i32, cols as i32) },
            Action::ScrollUp(_) => { 0 }, // TODO, not supported by pancurses
            Action::ScrollDown(_) => { 0 }, // TODO, not supported by pancurses
            Action::EnableRawMode => {
                pancurses::noecho();
                pancurses::raw();
                pancurses::nonl()
            },
            Action::DisableRawMode => {
                pancurses::echo();
                pancurses::noraw();
                pancurses::nl()
            },
            Action::EnterAlternateScreen => {
                // TODO,
                0i32
            },
            Action::LeaveAlternateScreen => {
                // TODO,
                0i32
            },
            Action::EnableMouseCapture => {
                pancurses::mousemask(
                    MOUSE_EVENT_MASK,
                    ::std::ptr::null_mut(),
                ) as i32
            },
            Action::DisableMouseCapture => {
                pancurses::mousemask(
                    !MOUSE_EVENT_MASK,
                    ::std::ptr::null_mut(),
                )  as i32
            },
            Action::SetForegroundColor(color) => {
                pancurses::init_pair(0, find_closest(Color::Blue, COLORS() as i16), -1);
                self.window.color_set(0)
            },
            Action::SetBackgroundColor(color) => {
                let color = find_closest(color, pancurses::COLORS() as i16);

                self.window.color_set(0)
            },
            Action::SetAttribute(attr) => {
                let no_match1: Option<()> = match attr {
                    Attribute::Reset => Some(pancurses::Attribute::Normal),
                    Attribute::Bold  => Some(pancurses::Attribute::Bold),
                    Attribute::Italic => Some(pancurses::Attribute::Italic),
                    Attribute::Underlined => Some(pancurses::Attribute::Underline),
                    Attribute::SlowBlink | Attribute::RapidBlink  => Some(pancurses::Attribute::Blink),
                    Attribute::Crossed  => Some(pancurses::Attribute::Overline),
                    Attribute::Reversed => Some(pancurses::Attribute::Reverse),
                    Attribute::Conceal => Some(pancurses::Attribute::Invisible),
                    _ => None // OFF attributes and Fraktur, NormalIntensity, NormalIntensity, Framed
                }.map(|attribute| {
                    self.window.attron(attribute);
                });

                let no_match2: Option<()> = match attr {
                    Attribute::BoldOff  => Some(pancurses::Attribute::Bold),
                    Attribute::ItalicOff => Some(pancurses::Attribute::Italic),
                    Attribute::UnderlinedOff => Some(pancurses::Attribute::Underline),
                    Attribute::BlinkOff  => Some(pancurses::Attribute::Blink),
                    Attribute::CrossedOff  => Some(pancurses::Attribute::Overline),
                    Attribute::ReversedOff => Some(pancurses::Attribute::Reverse),
                    Attribute::ConcealOff => Some(pancurses::Attribute::Invisible),
                    _ => None // OFF attributes and Fraktur, NormalIntensity, NormalIntensity, Framed
                }.map(|attribute| {
                    self.window.attroff(attribute);
                });

                if no_match1.is_none() && no_match2.is_none() {
                    return  Err(error::ErrorKind::AttributeNotSupported(String::from(
                        attr,
                    )))?
                } else {
                    return Ok(())
                }
            },
            Action::ResetColor => { 0 }, // TODO
        };

        Ok(())
    }

    fn flush_batch(&mut self, buffer: &mut W) -> error::Result<()> {
        unimplemented!()
    }

    fn get(&self, retrieve_operation: Value) -> error::Result<Retrieved> {
       match retrieve_operation {
           Value::TerminalSize => {
               // Coordinates are reversed here
               let (y, x) = self.window.get_max_yx();
               Ok(Retrieved::TerminalSize(x as u16, y as u16))
           },
           Value::CursorPosition => {
               let (y, x) = self.window.get_cur_yx();
               Ok(Retrieved::CursorPosition(y as u16, x as u16))
           },
           Value::Event(duration) => { // TODO: FIX DELAY

               if let Some(event) = self.get_input_buffer() {
                   return Ok(Retrieved::Event(Some(event)));
               }

               if let Some(input) = self.window.getch() {
                   self.parse_next(input);
               }

               Ok(Retrieved::Event(Some(Event::Resize)))
           },
       }
    }
}

impl<W:Write> Drop for BackendImpl<W> {
    fn drop(&mut self) {
        print!("\x1B[?1002l");
        io::stdout().flush().expect("could not flush stdout");
        pancurses::endwin();
    }
}