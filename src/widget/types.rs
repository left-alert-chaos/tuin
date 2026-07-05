//!# types
//!This module holds traits, enums, and structs to define widget behavior.

use std::sync::mpsc::Sender;

pub trait Widget {
    fn render(&self) -> String {
        String::from("Hello, world!")
    }
    fn manager(&mut self, sender: Sender<DrawRequest>, id: u32);
}

pub struct DrawRequest {
    pub(crate) widget_id: u32,
    pub(crate) render: String,
}

impl DrawRequest {
    fn new(widget: &dyn Widget, widget_id: u32) -> Self {
        Self {
            widget_id,
            render: widget.render(),
        }
    }
}
