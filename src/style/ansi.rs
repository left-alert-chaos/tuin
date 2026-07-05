//!# ansi
//!This module holds code to translate style objects into ansi escape sequences, mostly through
//!impls of objects defined elsewhere.

use crate::style::AnsiColor;

///Anything implementing this trait can be converted into an ANSI escape sequence.
pub trait Ansi {
    fn sequence(&self) -> String;
}
