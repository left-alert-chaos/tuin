//!# color
//!This module holds code to calculate color and text style codes.

use crate::style::ansi::Ansi;

///Represents the background or the foreground of the terminal.
pub enum Layer {
    Foreground(Vec<Box<dyn Ansi>>),
    Background(Vec<Box<dyn Ansi>>),
}

///Represents ANSI colors. The u32 values are foreground codes; to get background codes add 10.
///Ironically, it doesn't implement Ansi. Instead, it's used by other types to store color.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum AnsiColor {
    Reset = 0,
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    Default = 39,
}

impl AnsiColor {
    pub fn code(&self, layer: Layer) -> u32 {
        if self == &AnsiColor::Reset {
            return 0;
        }

        match layer {
            Layer::Foreground(_) => *self as u32,
            Layer::Background(_) => *self as u32 + 10,
        }
    }
}

///Controls whether text is normal, bold, or dim
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Brightness {
    Bold,
    Dim,
    Reset,
}

impl Ansi for Brightness {
    fn sequence(&self) -> String {
        match self {
            Brightness::Bold => String::from("\x1b[1m"),
            Brightness::Dim => String::from("\x1b[2m"),
            Brightness::Reset => String::from("\x1b[22m"),
        }
    }
}

///Controls whether text is italicized or regular
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Italic {
    Italicized,
    Reset,
}

impl Ansi for Italic {
    fn sequence(&self) -> String {
        match self {
            Italic::Italicized => String::from("\x1b[3m"),
            Italic::Reset => String::from("\x1b[23m"),
        }
    }
}

///Controls whether text is underlined
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Underline {
    Underlined,
    Reset,
}

impl Ansi for Underline {
    fn sequence(&self) -> String {
        match self {
            Underline::Underlined => String::from("\x1b[4m"),
            Underline::Reset => String::from("\x1b[24m"),
        }
    }
}

///Controls whether text blinks
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Blink {
    Blinking,
    Reset,
}

impl Ansi for Blink {
    fn sequence(&self) -> String {
        match self {
            Blink::Blinking => String::from("\x1b[5m"),
            Blink::Reset => String::from("\x1b[25m"),
        }
    }
}

///Controls whether colors are inverted
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Reverse {
    Reversed,
    Reset,
}

impl Ansi for Reverse {
    fn sequence(&self) -> String {
        match self {
            Reverse::Reversed => String::from("\x1b[7m"),
            Reverse::Reset => String::from("\x1b[27m"),
        }
    }
}

///Controls whether text is showing
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Hide {
    Hidden,
    Reset,
}

impl Ansi for Hide {
    fn sequence(&self) -> String {
        match self {
            Hide::Hidden => String::from("\x1b[8m"),
            Hide::Reset => String::from("\x1b[28m"),
        }
    }
}

///Controls whether text is struck through
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Strikethrough {
    Struck,
    Reset,
}

impl Ansi for Strikethrough {
    fn sequence(&self) -> String {
        match self {
            Strikethrough::Struck => String::from("\x1b[9m"),
            Strikethrough::Reset => String::from("\x1b[29m"),
        }
    }
}
