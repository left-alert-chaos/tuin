//!# style::widget
//!This module holds code to represent widget styling.

use crate::style::{Ansi, Layer};


///Store information about a widget's styling.
pub struct Surface {
    pub(crate) text: Layer,
    pub(crate) background: Layer,
    pub(crate) pad_y: u32,
    pub(crate) pad_x: u32,
}
