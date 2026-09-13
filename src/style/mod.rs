//!# style
//!This module holds data types to represent colors and styling. Instead of color numbers, we store
//!ANSI codes.

mod ansi;
mod text;
mod widget;
pub use ansi::Ansi;
pub use text::*;
