use gtk::prelude::*;
use gtk::{TextBuffer, TextView, ScrolledWindow};

#[derive(Clone)]
pub struct PreviewPane {
    buffer: TextBuffer,
    view: TextView,
    scroll: ScrolledWindow,
}

impl PreviewPane {
    pub fn new() -> Self {
        let buffer = TextBuffer::new(None);
        let view = TextView::builder()
            .buffer(&buffer)
            .editable(false)
            .cursor_visible(false)
            .wrap_mode(gtk::WrapMode::Word)
            .left_margin(24)
            .right_margin(24)
            .top_margin(24)
            .bottom_margin(24)
            .build();

        let scroll = ScrolledWindow::new();
        scroll.set_hexpand(true);
        scroll.set_vexpand(true);
        scroll.set_child(Some(&view));

        Self { buffer, view, scroll }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.scroll
    }

    pub fn update(&self, text: &str) {
        self.buffer.set_text(text);
    }
}
