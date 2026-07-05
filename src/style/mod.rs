//!# style
//!This module holds data types to represent colors and styling. Instead of color numbers, we store
//!ANSI codes.

mod widget;
mod text;
mod ansi;
pub use text::*;
pub use ansi::Ansi;
