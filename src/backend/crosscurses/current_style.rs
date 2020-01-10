use crate::Color;
use crosscurses::Attributes;

pub(crate) struct CurrentStyle {
    pub(crate) foreground: Color,
    pub(crate) background: Color,
    pub(crate) attributes: Attributes,
}

impl CurrentStyle {
    pub(crate) fn new() -> CurrentStyle {
        CurrentStyle {
            foreground: Color::Reset,
            background: Color::Reset,
            attributes: Attributes::new(),
        }
    }
}
