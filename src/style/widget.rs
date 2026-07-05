//!# style::widget
//!This module holds code to represent widget styling.

///Store information about a widget's styling.
struct Surface {
    pub(crate) bg: String,
    pub(crate) text: String,
    pub(crate) pad_y: u32,
    pub(crate) pad_x: u32,
}
