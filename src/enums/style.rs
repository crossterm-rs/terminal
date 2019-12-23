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
