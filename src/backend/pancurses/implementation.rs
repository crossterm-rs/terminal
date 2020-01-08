use crate::{
    backend::{
        pancurses::{current_style::CurrentStyle, mapping::find_closest},
        Backend,
    },
    error, Action, Attribute, Clear, Color, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton,
    Retrieved, Value,
};
use pancurses::{ToChtype, Window, COLORS};
use std::{
    collections::HashMap,
    ffi::CStr,
    fs::File,
    io,
    io::{Error, ErrorKind, Write},
    os::unix::io::IntoRawFd,
    result,
    sync::RwLock,
};

#[derive(Default)]
struct InputCache {
    last_mouse_button: Option<MouseButton>,
    stored_event: Option<Event>,
}

pub struct BackendImpl<W: Write> {
    buffer: W,
    window: pancurses::Window,

    input_cache: RwLock<InputCache>,

    // ncurses stores color values in pairs (fg, bg) color.
    // We store those pairs in this hashmap on order to keep track of the pairs we initialized.
    color_pairs: HashMap<i16, i32>,

    pub(crate) key_codes: HashMap<i32, Event>,

    // bg, fg
    current_style: CurrentStyle,
}

impl<W: Write> BackendImpl<W> {
    /// Prints the given string-like value into the window by printing each
    pub fn print<S: AsRef<str>>(&mut self, asref: S) -> error::Result<()> {
        // Here we want to
        if cfg!(windows) {
            // PDCurses does an extra intermediate CString allocation, so we just
            // print out each character one at a time to avoid that.
            asref.as_ref().chars().all(|c| self.print_char(c).is_ok());
        } else {
            // NCurses, it seems, doesn't do the intermediate allocation and also uses
            // a faster routine for printing a whole string at once.
            self.window.printw(asref.as_ref());
        }

        Ok(())
    }

    /// Prints the given character into the window.
    pub fn print_char<T: ToChtype>(&mut self, character: T) -> error::Result<()> {
        self.window.addch(character);

        Ok(())
    }

    pub fn update_input_buffer(&self, btn: Event) {
        let mut lock = self.input_cache.write().unwrap();
        lock.stored_event = Some(btn);
    }

    pub fn try_take(&self) -> Option<Event> {
        self.input_cache.write().unwrap().stored_event.take()
    }

    pub fn update_last_btn(&self, btn: MouseButton) {
        let mut lock = self.input_cache.write().unwrap();
        lock.last_mouse_button = Some(btn);
    }

    pub fn get_last_btn(&self) -> Option<MouseButton> {
        self.input_cache.read().unwrap().last_mouse_button.clone()
    }

    pub fn store_fg(&mut self, fg_color: Color) -> i32 {
        let closest_fg_color = find_closest(fg_color, COLORS() as i16);
        let closest_bg_color = find_closest(self.current_style.background, COLORS() as i16);

        self.get_or_insert(closest_fg_color, closest_fg_color, closest_bg_color)
    }

    pub fn store_bg(&mut self, bg_color: Color) -> i32 {
        let closest_fg_color = find_closest(self.current_style.foreground, COLORS() as i16);
        let closest_bg_color = find_closest(bg_color, COLORS() as i16);

        self.get_or_insert(closest_bg_color, closest_fg_color, closest_bg_color)
    }

    pub fn get_or_insert(&mut self, key: i16, fg_color: i16, bg_color: i16) -> i32 {
        if self.color_pairs.contains_key(&key) {
            self.color_pairs[&key]
        } else {
            let index = self.new_color_pair_index();

            self.color_pairs.insert(key, index);
            pancurses::init_pair(index as i16, fg_color, bg_color);
            index
        }
    }

    fn new_color_pair_index(&mut self) -> i32 {
        let n = 1 + self.color_pairs.len() as i32;

        if 256 > n {
            // We still have plenty of space for everyone.
            n
        } else {
            // resize color pairs
            let target = n - 1;
            // Remove the mapping to n-1
            self.color_pairs.retain(|_, &mut v| v != target);
            target
        }
    }
}

fn init_stdout_window() -> Window {
    // For windows we only support the default pancurses initialisation.
    // By default pancurses uses stdout.
    // TODO: support using `newterm` like `init_unix_window` so that we are not depended off stdout.
    pancurses::initscr()
}

#[cfg(unix)]
fn init_custom_window() -> Window {
    // By default pancurses use stdout.
    // We can change this by calling `new_term` with an FILE pointer to the source.
    // Which is /dev/tty in our case.
    let file = File::open("/dev/tty").unwrap();

    let c_file = unsafe {
        libc::fdopen(
            file.into_raw_fd(),
            CStr::from_bytes_with_nul_unchecked(b"w+\0").as_ptr(),
        )
    };

    if cfg!(unix)
        && std::env::var("TERM")
            .map(|var| var.is_empty())
            .unwrap_or(false)
    {
        return init_stdout_window();
    } else {
        // Create screen pointer which we will be using for this backend.
        let screen = pancurses::newterm(Some(env!("TERM")), c_file, c_file);

        // Set the created screen as active.
        pancurses::set_term(screen);

        // Get `Window` of the created screen.
        pancurses::stdscr()
    }
}

impl<W: Write> Backend<W> for BackendImpl<W> {
    fn create(buffer: W) -> Self {
        // The delay is the time ncurses wait after pressing ESC
        // to see if it's an escape sequence.
        // Default delay is way too long. 25 is imperceptible yet works fine.
        ::std::env::set_var("ESCDELAY", "25");

        #[cfg(windows)]
        let window = init_windows_window();

        #[cfg(unix)]
        let window = init_custom_window();

        // Some default settings
        window.keypad(true);
        pancurses::start_color();
        pancurses::use_default_colors();
        pancurses::mousemask(
            pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION,
            ::std::ptr::null_mut(),
        );

        // Initialize the default fore and background.
        let mut map = HashMap::<i16, i32>::new();
        map.insert(-1, 0);
        pancurses::init_pair(0, -1, -1);

        BackendImpl {
            window,
            input_cache: RwLock::new(InputCache::default()),
            color_pairs: map,
            key_codes: initialize_keymap(),
            current_style: CurrentStyle::new(),
            buffer,
        }
    }

    fn act(&mut self, action: Action) -> error::Result<()> {
        self.batch(action)?;
        self.flush_batch()
    }

    #[warn(unused_assignments)]
    fn batch(&mut self, action: Action) -> error::Result<()> {
        let mut r = 0;

        match action {
            Action::MoveCursorTo(x, y) => {
                r = self.window.mv(y as i32, x as i32);
            }
            Action::HideCursor => {
                r = pancurses::curs_set(0) as i32;
            }
            Action::ShowCursor => {
                r = pancurses::curs_set(1) as i32;
            }
            Action::EnableBlinking => {
                r = pancurses::set_blink(true);
            }
            Action::DisableBlinking => {
                r = pancurses::set_blink(false);
            }
            Action::ClearTerminal(clear_type) => {
                r = match clear_type {
                    Clear::All => self.window.clear(),
                    Clear::FromCursorDown => self.window.clrtobot(),
                    Clear::UntilNewLine => self.window.clrtoeol(),
                    Clear::FromCursorUp => 3, // TODO, not supported by pancurses
                    Clear::CurrentLine => 3,  // TODO, not supported by pancurses
                };
            }
            Action::SetTerminalSize(cols, rows) => {
                pancurses::resize_term(rows as i32, cols as i32);
            }
            Action::EnableRawMode => {
                r = pancurses::noecho();
                r = pancurses::raw();
                r = pancurses::nonl();
            }
            Action::DisableRawMode => {
                r = pancurses::echo();
                r = pancurses::noraw();
                r = pancurses::nl();
            }
            Action::EnableMouseCapture => {
                print!("\x1B[?1002h");
                io::stdout().flush()?;
            }
            Action::DisableMouseCapture => {
                print!("\x1B[?1002l");
                io::stdout().flush().expect("could not flush stdout");
            }
            Action::ResetColor => {
                let style = pancurses::COLOR_PAIR(0 as pancurses::chtype);
                r = self.window.attron(style);
                r = self.window.attroff(self.current_style.attributes);
                r = self.window.refresh();
            }
            Action::SetForegroundColor(color) => {
                self.current_style.foreground = color;
                let index = self.store_fg(color);
                let style = pancurses::COLOR_PAIR(index as pancurses::chtype);
                r = self.window.attron(style);
                r = self.window.refresh();
            }
            Action::SetBackgroundColor(color) => {
                self.current_style.background = color;
                let index = self.store_bg(color);
                let style = pancurses::COLOR_PAIR(index as pancurses::chtype);
                r = self.window.attron(style);
                r = self.window.refresh();
            }
            Action::SetAttribute(attr) => {
                let no_match1 = match attr {
                    Attribute::Reset => Some(pancurses::Attribute::Normal),
                    Attribute::Bold => Some(pancurses::Attribute::Bold),
                    Attribute::Italic => Some(pancurses::Attribute::Italic),
                    Attribute::Underlined => Some(pancurses::Attribute::Underline),
                    Attribute::SlowBlink | Attribute::RapidBlink => {
                        Some(pancurses::Attribute::Blink)
                    }
                    Attribute::Crossed => Some(pancurses::Attribute::Strikeout),
                    Attribute::Reversed => Some(pancurses::Attribute::Reverse),
                    Attribute::Conceal => Some(pancurses::Attribute::Invisible),
                    _ => None, // OFF attributes and Fraktur, NormalIntensity, NormalIntensity, Framed
                }
                .map(|attribute| {
                    r = self.window.attron(attribute);
                    self.current_style.attributes = self.current_style.attributes | attribute;
                });

                let no_match2 = match attr {
                    Attribute::BoldOff => Some(pancurses::Attribute::Bold),
                    Attribute::ItalicOff => Some(pancurses::Attribute::Italic),
                    Attribute::UnderlinedOff => Some(pancurses::Attribute::Underline),
                    Attribute::BlinkOff => Some(pancurses::Attribute::Blink),
                    Attribute::CrossedOff => Some(pancurses::Attribute::Strikeout),
                    Attribute::ReversedOff => Some(pancurses::Attribute::Reverse),
                    Attribute::ConcealOff => Some(pancurses::Attribute::Invisible),
                    _ => None, // OFF attributes and Fraktur, NormalIntensity, NormalIntensity, Framed
                }
                .map(|attribute| {
                    r = self.window.attroff(attribute);
                    self.current_style.attributes = self.current_style.attributes ^ attribute;
                });

                if no_match1.is_none() && no_match2.is_none() {
                    return Err(error::ErrorKind::AttributeNotSupported(String::from(attr)))?;
                } else {
                    return Ok(());
                }
            }
            Action::EnterAlternateScreen
            | Action::LeaveAlternateScreen
            | Action::ScrollUp(_)
            | Action::ScrollDown(_) => r = 3,
        };

        match r {
            0 => Ok(()),
            -1 => {
                Err(error::ErrorKind::IoError(Error::new(ErrorKind::Other, "Some error occurred while executing the action")))
            }
            3 => {
                Err(error::ErrorKind::ActionNotSupported("The action is not supported by pancurses. Either work around it or use an other backend.".to_string()))
            }
            _ => Ok(())
        }
    }

    fn flush_batch(&mut self) -> error::Result<()> {
        self.window.refresh();
        Ok(())
    }

    fn get(&self, retrieve_operation: Value) -> error::Result<Retrieved> {
        match retrieve_operation {
            Value::TerminalSize => {
                // Coordinates are reversed here
                let (y, x) = self.window.get_max_yx();
                Ok(Retrieved::TerminalSize(x as u16, y as u16))
            }
            Value::CursorPosition => {
                let (y, x) = self.window.get_cur_yx();
                Ok(Retrieved::CursorPosition(y as u16, x as u16))
            }
            Value::Event(duration) => {
                if let Some(event) = self.try_take() {
                    return Ok(Retrieved::Event(Some(event)));
                }

                let duration = duration.map_or(-1, |f| f.as_millis() as i32);

                self.window.timeout(duration);

                if let Some(input) = self.window.getch() {
                    return Ok(Retrieved::Event(Some(self.parse_next(input))));
                }

                Ok(Retrieved::Event(None))
            }
        }
    }
}

impl<W: Write> Drop for BackendImpl<W> {
    fn drop(&mut self) {
        let _ = self.act(Action::DisableMouseCapture);
        pancurses::endwin();
    }
}

impl<W: Write> Write for BackendImpl<W> {
    fn write(&mut self, buf: &[u8]) -> result::Result<usize, io::Error> {
        let string = std::str::from_utf8(buf).unwrap();
        let len = string.len();
        self.print(string).unwrap();
        Ok(len)
    }

    fn flush(&mut self) -> result::Result<(), io::Error> {
        self.window.refresh();
        Ok(())
    }
}

fn initialize_keymap() -> HashMap<i32, Event> {
    let mut map = HashMap::default();

    fill_key_codes(&mut map, pancurses::keyname);

    map
}

fn fill_key_codes<F>(target: &mut HashMap<i32, Event>, f: F)
where
    F: Fn(i32) -> Option<String>,
{
    let mut key_names = HashMap::<&str, KeyCode>::new();
    key_names.insert("DC", KeyCode::Delete);
    key_names.insert("DN", KeyCode::Down);
    key_names.insert("END", KeyCode::End);
    key_names.insert("HOM", KeyCode::Home);
    key_names.insert("IC", KeyCode::Insert);
    key_names.insert("LFT", KeyCode::Left);
    key_names.insert("NXT", KeyCode::PageDown);
    key_names.insert("PRV", KeyCode::PageUp);
    key_names.insert("RIT", KeyCode::Right);
    key_names.insert("UP", KeyCode::Up);

    for code in 512..1024 {
        let name = match f(code) {
            Some(name) => name,
            None => continue,
        };

        if !name.starts_with('k') {
            continue;
        }

        let (key_name, modifier) = name[1..].split_at(name.len() - 2);
        let key = match key_names.get(key_name) {
            Some(&key) => key,
            None => continue,
        };

        let event = match modifier {
            "3" => Event::Key(KeyEvent {
                code: key,
                modifiers: KeyModifiers::ALT,
            }),
            "4" => Event::Key(KeyEvent {
                code: key,
                modifiers: KeyModifiers::ALT | KeyModifiers::SHIFT,
            }),
            "5" => Event::Key(KeyEvent {
                code: key,
                modifiers: KeyModifiers::CONTROL,
            }),
            "6" => Event::Key(KeyEvent {
                code: key,
                modifiers: KeyModifiers::CONTROL | KeyModifiers::CONTROL,
            }),
            "7" => Event::Key(KeyEvent {
                code: key,
                modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
            }),
            _ => continue,
        };

        target.insert(code, event);
    }
}
