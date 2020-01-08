/// Represents an color.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Color {
    /// Resets the terminal color.
    Reset,
    /// Black color.
    Black,
    /// Dark grey color.
    DarkGrey,
    /// Light red color.
    Red,
    /// Dark red color.
    DarkRed,
    /// Light green color.
    Green,
    /// Dark green color.
    DarkGreen,
    /// Light yellow color.
    Yellow,
    /// Dark yellow color.
    DarkYellow,
    /// Light blue color.
    Blue,
    /// Dark blue color.
    DarkBlue,
    /// Light magenta color.
    Magenta,
    /// Dark magenta color.
    DarkMagenta,
    /// Light cyan color.
    Cyan,
    /// Dark cyan color.
    DarkCyan,
    /// White color.
    White,
    /// Grey color.
    Grey,
    /// An RGB color. See [RGB color model](https://en.wikipedia.org/wiki/RGB_color_model) for more info.
    ///
    /// Most UNIX terminals and Windows 10 supported only.
    /// See [Platform-specific notes](enum.Color.html#platform-specific-notes) for more info.
    Rgb(u8, u8, u8),

    /// An ANSI color. See [256 colors - cheat sheet](https://jonasjacek.github.io/colors/) for more info.
    ///
    /// Most UNIX terminals and Windows 10 supported only.
    /// See [Platform-specific notes](enum.Color.html#platform-specific-notes) for more info.
    AnsiValue(u8),
}

impl From<u8> for Color {
    fn from(n: u8) -> Self {
        match n {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,

            8 => Color::Black,
            9 => Color::DarkRed,
            10 => Color::DarkGreen,
            11 => Color::DarkYellow,
            12 => Color::DarkBlue,
            13 => Color::DarkMagenta,
            14 => Color::DarkCyan,
            15 => Color::Grey,

            // parsing: https://stackoverflow.com/questions/27159322/rgb-values-of-the-colors-in-the-ansi-extended-colors-index-17-255
            _ if n > 15  && n < 232 => {
                let rgb_r = ((n - 16) / 36) * 51;
                let rgb_g = (((n - 16) % 36) / 6) * 51;
                let rgb_b = ((n - 16) % 6) * 51;

                Color::Rgb(rgb_r, rgb_g, rgb_b)
            }
            _ if n >= 232 => {
                let value = (n - 232) * 10 + 8;
                Color::Rgb(value, value, value)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Attribute {
    /// Resets all the attributes.
    Reset,

    /// Increases the text intensity.
    Bold,
    /// Decreases the text intensity.
    BoldOff,

    /// Emphasises the text.
    Italic,
    /// Turns off the `Italic` attribute.
    ItalicOff,

    /// Underlines the text.
    Underlined,
    /// Turns off the `Underlined` attribute.
    UnderlinedOff,

    /// Makes the text blinking (< 150 per minute).
    SlowBlink,
    /// Makes the text blinking (>= 150 per minute).
    RapidBlink,
    /// Turns off the text blinking (`SlowBlink` or `RapidBlink`).
    BlinkOff,

    /// Crosses the text.
    Crossed,
    /// Turns off the `CrossedOut` attribute.
    CrossedOff,

    /// Swaps foreground and background colors.
    Reversed,
    /// Turns off the `Reverse` attribute.
    ReversedOff,

    /// Hides the text (also known as hidden).
    Conceal,
    /// Turns off the `Hidden` attribute.
    ConcealOff,

    /// Sets the [Fraktur](https://en.wikipedia.org/wiki/Fraktur) typeface.
    ///
    /// Mostly used for [mathematical alphanumeric symbols](https://en.wikipedia.org/wiki/Mathematical_Alphanumeric_Symbols).
    Fraktur,

    /// Turns off the `Bold` attribute.
    NormalIntensity,

    /// Switches the text back to normal intensity (no bold, italic).
    BoldItalicOff,
    /// Makes the text framed.
    Framed,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<Attribute> for String {
    fn from(attr: Attribute) -> Self {
        format!("{:?}", attr)
    }
}
