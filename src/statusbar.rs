use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Label};

#[derive(Clone)]
pub struct StatusBar {
    container: GtkBox,
    word_count_label: Label,
    char_count_label: Label,
    reading_time_label: Label,
    line_col_label: Label,
}

impl StatusBar {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(16)
            .build();
        container.add_css_class("statusbar");

        let word_count_label = Label::builder()
            .label("0 words")
            .build();

        let char_count_label = Label::builder()
            .label("0 chars")
            .build();

        let reading_time_label = Label::builder()
            .label("0 min read")
            .build();

        let line_col_label = Label::builder()
            .label("Ln 1, Col 1")
            .build();

        container.append(&word_count_label);
        container.append(&char_count_label);
        container.append(&reading_time_label);
        
        let spacer = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .build();
        spacer.set_hexpand(true);
        container.append(&spacer);
        
        container.append(&line_col_label);

        Self {
            container,
            word_count_label,
            char_count_label,
            reading_time_label,
            line_col_label,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn update_stats(&self, words: usize, chars: usize, reading_time: f64) {
        self.word_count_label.set_text(&format!("{} words", words));
        self.char_count_label.set_text(&format!("{} chars", chars));
        self.reading_time_label.set_text(&format!("{} min read", reading_time as usize));
    }

    pub fn update_cursor(&self, line: usize, col: usize) {
        self.line_col_label.set_text(&format!("Ln {}, Col {}", line, col));
    }
}
