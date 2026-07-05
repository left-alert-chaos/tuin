//!# color
//!This module holds code to calculate color and text style codes.

use crate::style::ansi::Ansi;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Reset,
    
}

///Controls whether text is normal, bold, or dim
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
