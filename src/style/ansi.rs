//!# ansi
//!This module holds code to translate style objects into ansi escape sequences, mostly throught
//!impls of objects defined elsewhere.

pub trait Ansi {
    fn sequence(&self) -> String;
}
