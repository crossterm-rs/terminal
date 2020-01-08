use crate::{Color, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use pancurses::{mmask_t, Input};
use std::io::Write;

impl<W: Write> super::BackendImpl<W> {
    pub fn parse_next(&self, input: pancurses::Input) -> Event {
        // Try to map the pancurses input event to an `KeyEvent` with possible modifiers.
        let key_event = self.try_parse_key(&input).map_or(
            self.try_map_shift_key(&input).map_or(
                self.try_map_ctrl_key(&input)
                    .map_or(self.try_map_ctrl_alt_key(&input), |e| Some(e)),
                |e| Some(e),
            ),
            |e| Some(e),
        );

        match key_event {
            Some(key_event) => Event::Key(key_event),
            None => {
                // TODO, if your event is not mapped, feel free to add it.
                // Although other backends have to support it as well.
                // The point of this library is to support the most of the important keys.

                self.try_map_non_key_event(&input)
                    .map_or(Event::Unknown, |e| e)
            }
        }
    }

    /// Matches on keys without modifiers, returns `None` if the key has modifiers or is not supported.
    pub fn try_parse_key(&self, input: &pancurses::Input) -> Option<KeyEvent> {
        let empty = KeyModifiers::empty();

        let key_code = match input {
            &Input::Character(c) => match c {
                '\r' | '\n' => Some(KeyCode::Enter.into()),
                '\t' => Some(KeyCode::Tab.into()),
                '\x7F' => Some(KeyCode::Backspace.into()),
                '\u{8}' => Some(KeyCode::Backspace.into()),
                c @ '\x01'..='\x1A' => Some(KeyEvent::new(
                    KeyCode::Char((c as u8 - 0x1 + b'a') as char),
                    KeyModifiers::CONTROL,
                )),
                c @ '\x1C'..='\x1F' => Some(KeyEvent::new(
                    KeyCode::Char((c as u8 - 0x1C + b'4') as char),
                    KeyModifiers::CONTROL,
                )),
                _ if (c as u32) <= 26 => Some(KeyEvent::new(
                    KeyCode::Char((b'a' - 1 + c as u8) as char),
                    KeyModifiers::CONTROL,
                )),
                '\u{1b}' => Some(KeyCode::Esc.into()),
                c => Some(KeyCode::Char(c).into()),
            },
            &Input::KeyDown => Some(KeyEvent {
                code: KeyCode::Down,
                modifiers: empty,
            }),
            &Input::KeyUp => Some(KeyEvent {
                code: KeyCode::Up,
                modifiers: empty,
            }),
            &Input::KeyLeft => Some(KeyEvent {
                code: KeyCode::Left,
                modifiers: empty,
            }),
            &Input::KeyRight => Some(KeyEvent {
                code: KeyCode::Right,
                modifiers: empty,
            }),
            &Input::KeyHome => Some(KeyEvent {
                code: KeyCode::Home,
                modifiers: empty,
            }),
            &Input::KeyBackspace => Some(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: empty,
            }),
            &Input::KeyF0 => Some(KeyEvent {
                code: KeyCode::F(0),
                modifiers: empty,
            }),
            &Input::KeyF1 => Some(KeyEvent {
                code: KeyCode::F(1),
                modifiers: empty,
            }),
            &Input::KeyF2 => Some(KeyEvent {
                code: KeyCode::F(2),
                modifiers: empty,
            }),
            &Input::KeyF3 => Some(KeyEvent {
                code: KeyCode::F(3),
                modifiers: empty,
            }),
            &Input::KeyF4 => Some(KeyEvent {
                code: KeyCode::F(4),
                modifiers: empty,
            }),
            &Input::KeyF5 => Some(KeyEvent {
                code: KeyCode::F(5),
                modifiers: empty,
            }),
            &Input::KeyF6 => Some(KeyEvent {
                code: KeyCode::F(6),
                modifiers: empty,
            }),
            &Input::KeyF7 => Some(KeyEvent {
                code: KeyCode::F(7),
                modifiers: empty,
            }),
            &Input::KeyF8 => Some(KeyEvent {
                code: KeyCode::F(8),
                modifiers: empty,
            }),
            &Input::KeyF9 => Some(KeyEvent {
                code: KeyCode::F(9),
                modifiers: empty,
            }),
            &Input::KeyF10 => Some(KeyEvent {
                code: KeyCode::F(10),
                modifiers: empty,
            }),
            &Input::KeyF11 => Some(KeyEvent {
                code: KeyCode::F(11),
                modifiers: empty,
            }),
            &Input::KeyF12 => Some(KeyEvent {
                code: KeyCode::F(12),
                modifiers: empty,
            }),
            &Input::KeyF13 => Some(KeyEvent {
                code: KeyCode::F(13),
                modifiers: empty,
            }),
            &Input::KeyF14 => Some(KeyEvent {
                code: KeyCode::F(14),
                modifiers: empty,
            }),
            &Input::KeyF15 => Some(KeyEvent {
                code: KeyCode::F(15),
                modifiers: empty,
            }),
            &Input::KeyDL => Some(KeyEvent {
                code: KeyCode::Delete,
                modifiers: empty,
            }),
            &Input::KeyIC => Some(KeyEvent {
                code: KeyCode::Insert,
                modifiers: empty,
            }),
            &Input::KeyNPage => Some(KeyEvent {
                code: KeyCode::PageDown,
                modifiers: empty,
            }),
            &Input::KeyPPage => Some(KeyEvent {
                code: KeyCode::PageUp,
                modifiers: empty,
            }),
            &Input::KeyEnter => Some(KeyEvent {
                code: KeyCode::Enter,
                modifiers: empty,
            }),
            &Input::KeyEnd => Some(KeyEvent {
                code: KeyCode::End,
                modifiers: empty,
            }),
            _ => None,
        };

        key_code.map(|e| e)
    }

    /// Matches on shift keys, returns `None` if the key does not have an SHIFT modifier or is not supported.
    pub fn try_map_shift_key(&self, input: &pancurses::Input) -> Option<KeyEvent> {
        let key_code = match input {
            &Input::KeySF => Some(KeyCode::Down),
            &Input::KeySR => Some(KeyCode::Up),
            &Input::KeySTab => Some(KeyCode::Tab),
            &Input::KeySDC => Some(KeyCode::Delete),
            &Input::KeySEnd => Some(KeyCode::End),
            &Input::KeySHome => Some(KeyCode::Home),
            &Input::KeySIC => Some(KeyCode::Insert),
            &Input::KeySLeft => Some(KeyCode::Left),
            &Input::KeySNext => Some(KeyCode::PageDown),
            &Input::KeySPrevious => Some(KeyCode::PageDown),
            &Input::KeySPrint => Some(KeyCode::End),
            &Input::KeySRight => Some(KeyCode::Right),
            &Input::KeyBTab => Some(KeyCode::BackTab),
            _ => None,
        };

        key_code.map(|e| KeyEvent::new(e, KeyModifiers::SHIFT))
    }

    /// Matches on CTRL keys, returns `None` if the key does not have an CTRL modifier or is not supported.
    pub fn try_map_ctrl_key(&self, input: &pancurses::Input) -> Option<KeyEvent> {
        let key_code = match input {
            &Input::KeyCTab => Some(KeyCode::Tab),
            _ => None,
        };

        key_code.map(|e| KeyEvent::new(e, KeyModifiers::CONTROL))
    }

    /// Matches on CTRL + ALT keys, returns `None` if the key does not have an SHIFT + ALT modifier or is not supported.
    pub fn try_map_ctrl_alt_key(&self, input: &pancurses::Input) -> Option<KeyEvent> {
        let key_code = match input {
            &Input::KeyCATab => Some(KeyCode::Tab),
            _ => None,
        };

        key_code.map(|e| KeyEvent::new(e, KeyModifiers::CONTROL | KeyModifiers::ALT))
    }

    /// Matches on non key events, returns `None` if the key is not a non-key event or is not supported.
    pub fn try_map_non_key_event(&self, input: &pancurses::Input) -> Option<Event> {
        // No key event, handle non key events e.g resize
        match input {
            &Input::KeyResize => {
                // Let pancurses adjust their structures when the
                // window is resized.
                pancurses::resize_term(0, 0);

                Some(Event::Resize)
            }
            &Input::KeyMouse => Some(self.map_mouse_event()),
            &Input::Unknown(code) => {
                Some(
                    self.key_codes
                        // pancurses does some weird keycode mapping
                        .get(&(code + 256 + 48))
                        .cloned()
                        .unwrap_or_else(|| Event::Unknown),
                )
            }
            _ => None,
        }
    }

    fn map_mouse_event(&self) -> Event {
        let mut mevent = match pancurses::getmouse() {
            Err(code) => return Event::Unknown,
            Ok(event) => event,
        };

        let shift = (mevent.bstate & pancurses::BUTTON_SHIFT as mmask_t) != 0;
        let alt = (mevent.bstate & pancurses::BUTTON_ALT as mmask_t) != 0;
        let ctrl = (mevent.bstate & pancurses::BUTTON_CTRL as mmask_t) != 0;

        let mut modifiers = KeyModifiers::empty();

        if shift {
            modifiers |= KeyModifiers::SHIFT;
        }
        if ctrl {
            modifiers |= KeyModifiers::CONTROL;
        }
        if alt {
            modifiers |= KeyModifiers::ALT;
        }

        mevent.bstate &=
            !(pancurses::BUTTON_SHIFT | pancurses::BUTTON_ALT | pancurses::BUTTON_CTRL) as mmask_t;

        let (x, y) = (mevent.x as u16, mevent.y as u16);

        if mevent.bstate == pancurses::REPORT_MOUSE_POSITION as mmask_t {
            // The event is either a mouse drag event,
            // or a weird double-release event. :S
            self.get_last_btn()
                .map(|btn| Event::Mouse(MouseEvent::Drag(btn, x, y, modifiers)))
                .unwrap_or_else(|| {
                    // We got a mouse drag, but no last mouse pressed?
                    Event::Unknown
                })
        } else {
            // Identify the button
            let mut bare_event = mevent.bstate & ((1 << 25) - 1);

            let mut event = None;
            while bare_event != 0 {
                let single_event = 1 << bare_event.trailing_zeros();
                bare_event ^= single_event;

                // Process single_event
                self.on_mouse_event(
                    single_event,
                    |e| {
                        if event.is_none() {
                            event = Some(e);
                        } else {
                            self.update_input_buffer(Event::Mouse(e));
                        }
                    },
                    x,
                    y,
                    modifiers,
                );
            }

            if let Some(event) = event {
                if let Some(btn) = event.button() {
                    self.update_last_btn(btn);
                }

                Event::Mouse(event)
            } else {
                // No event parsed?...
                Event::Unknown
            }
        }
    }

    /// Parse the given code into one or more event.
    ///
    /// If the given event code should expend into multiple events
    /// (for instance click expends into PRESS + RELEASE),
    /// the returned Vec will include those queued events.
    ///
    /// The main event is returned separately to avoid allocation in most cases.
    fn on_mouse_event<F>(
        &self,
        bare_event: mmask_t,
        mut f: F,
        x: u16,
        y: u16,
        modifiers: KeyModifiers,
    ) where
        F: FnMut(MouseEvent),
    {
        let button = self.map_mouse_button(bare_event);
        match bare_event {
            pancurses::BUTTON4_PRESSED => f(MouseEvent::ScrollUp(x, y, modifiers)),
            pancurses::BUTTON5_PRESSED => f(MouseEvent::ScrollDown(x, y, modifiers)),
            pancurses::BUTTON1_RELEASED
            | pancurses::BUTTON2_RELEASED
            | pancurses::BUTTON3_RELEASED
            | pancurses::BUTTON4_RELEASED
            | pancurses::BUTTON5_RELEASED => f(MouseEvent::Up(button, x, y, modifiers)),
            pancurses::BUTTON1_PRESSED
            | pancurses::BUTTON2_PRESSED
            | pancurses::BUTTON3_PRESSED => f(MouseEvent::Down(button, x, y, modifiers)),
            pancurses::BUTTON1_CLICKED
            | pancurses::BUTTON2_CLICKED
            | pancurses::BUTTON3_CLICKED
            | pancurses::BUTTON4_CLICKED
            | pancurses::BUTTON5_CLICKED => {
                f(MouseEvent::Down(button, x, y, modifiers));
                f(MouseEvent::Up(button, x, y, modifiers));
            }
            // Well, we disabled click detection
            pancurses::BUTTON1_DOUBLE_CLICKED
            | pancurses::BUTTON2_DOUBLE_CLICKED
            | pancurses::BUTTON3_DOUBLE_CLICKED
            | pancurses::BUTTON4_DOUBLE_CLICKED
            | pancurses::BUTTON5_DOUBLE_CLICKED => {
                for _ in 0..2 {
                    f(MouseEvent::Down(button, x, y, modifiers));
                    f(MouseEvent::Up(button, x, y, modifiers));
                }
            }
            pancurses::BUTTON1_TRIPLE_CLICKED
            | pancurses::BUTTON2_TRIPLE_CLICKED
            | pancurses::BUTTON3_TRIPLE_CLICKED
            | pancurses::BUTTON4_TRIPLE_CLICKED
            | pancurses::BUTTON5_TRIPLE_CLICKED => {
                for _ in 0..3 {
                    f(MouseEvent::Down(button, x, y, modifiers));
                    f(MouseEvent::Up(button, x, y, modifiers));
                }
            }
            _ => { // Unknown event: {:032b}", bare_event }
            }
        }
    }

    /// Returns the Key enum corresponding to the given pancurses event.
    fn map_mouse_button(&self, bare_event: mmask_t) -> MouseButton {
        match bare_event {
            pancurses::BUTTON1_RELEASED
            | pancurses::BUTTON1_PRESSED
            | pancurses::BUTTON1_CLICKED
            | pancurses::BUTTON1_DOUBLE_CLICKED
            | pancurses::BUTTON1_TRIPLE_CLICKED => MouseButton::Left,
            pancurses::BUTTON2_RELEASED
            | pancurses::BUTTON2_PRESSED
            | pancurses::BUTTON2_CLICKED
            | pancurses::BUTTON2_DOUBLE_CLICKED
            | pancurses::BUTTON2_TRIPLE_CLICKED => MouseButton::Middle,
            pancurses::BUTTON3_RELEASED
            | pancurses::BUTTON3_PRESSED
            | pancurses::BUTTON3_CLICKED
            | pancurses::BUTTON3_CLICKED
            | pancurses::BUTTON3_DOUBLE_CLICKED
            | pancurses::BUTTON3_TRIPLE_CLICKED => MouseButton::Right,
            pancurses::BUTTON4_RELEASED
            | pancurses::BUTTON4_PRESSED
            | pancurses::BUTTON4_CLICKED
            | pancurses::BUTTON4_DOUBLE_CLICKED
            | pancurses::BUTTON4_TRIPLE_CLICKED => MouseButton::Unknown,
            pancurses::BUTTON5_RELEASED
            | pancurses::BUTTON5_PRESSED
            | pancurses::BUTTON5_CLICKED
            | pancurses::BUTTON5_DOUBLE_CLICKED
            | pancurses::BUTTON5_TRIPLE_CLICKED => MouseButton::Unknown,
            _ => MouseButton::Unknown,
        }
    }
}

pub fn find_closest(color: Color, max_colors: i16) -> i16 {
    // translate ansi value to rgb
    let color = if let Color::AnsiValue(val) = color {
        Color::from(val)
    } else {
        color
    };

    // translate to closest supported color.
    match color {
        // Dark colors
        Color::Black => pancurses::COLOR_BLACK,
        Color::DarkRed => pancurses::COLOR_RED,
        Color::DarkGreen => pancurses::COLOR_GREEN,
        Color::DarkYellow => pancurses::COLOR_YELLOW,
        Color::DarkBlue => pancurses::COLOR_BLUE,
        Color::DarkMagenta => pancurses::COLOR_MAGENTA,
        Color::DarkCyan => pancurses::COLOR_CYAN,
        Color::Grey => pancurses::COLOR_WHITE,

        // Light colors
        Color::Red => 9 % max_colors,
        Color::Green => 10 % max_colors,
        Color::Yellow => 11 % max_colors,
        Color::Blue => 12 % max_colors,
        Color::Magenta => 13 % max_colors,
        Color::Cyan => 14 % max_colors,
        Color::White => 15 % max_colors,
        Color::Rgb(r, g, b) if max_colors >= 256 => {
            // If r = g = b, it may be a grayscale value!
            if r == g && g == b && r != 0 && r < 250 {
                // Grayscale
                // (r = g = b) = 8 + 10 * n
                // (r - 8) / 10 = n
                let n = (r - 8) / 10;
                i16::from(232 + n)
            } else {
                // Generic RGB
                let r = 6 * u16::from(r) / 256;
                let g = 6 * u16::from(g) / 256;
                let b = 6 * u16::from(b) / 256;
                (16 + 36 * r + 6 * g + b) as i16
            }
        }
        Color::Rgb(r, g, b) => {
            let r = if r > 127 { 1 } else { 0 };
            let g = if g > 127 { 1 } else { 0 };
            let b = if b > 127 { 1 } else { 0 };
            (r + 2 * g + 4 * b) as i16
        }
        _ => -1, // -1 represents default color
    }
}
