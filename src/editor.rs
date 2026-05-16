use gio::prelude::*;
use gtk::prelude::*;
use sourceview5::prelude::*;
use sourceview5::{LanguageManager, Buffer, View, StyleSchemeManager};

#[derive(Clone)]
pub struct EditorPane {
    buffer: Buffer,
    view: View,
}

impl EditorPane {
    pub fn new() -> Self {
        let lang_manager = LanguageManager::default();
        let markdown_lang = lang_manager.language("markdown");

        let buffer = Buffer::new(None);
        if let Some(lang) = markdown_lang {
            buffer.set_language(Some(&lang));
        }

        let scheme_manager = StyleSchemeManager::default();
        if let Some(scheme) = scheme_manager.scheme("Adwaita") {
            buffer.set_style_scheme(Some(&scheme));
        }

        let view = View::builder()
            .buffer(&buffer)
            .monospace(true)
            .wrap_mode(gtk::WrapMode::Word)
            .left_margin(20)
            .right_margin(20)
            .top_margin(20)
            .bottom_margin(20)
            .show_line_numbers(false)
            .tab_width(4)
            .indent_width(4)
            .auto_indent(true)
            .build();

        buffer.set_text("# Start writing Markdown...\n\nType or paste your content here.");

        Self { buffer, view }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub fn load_file(&self, file: &gio::File) {
        if let Ok((contents, _)) = file.load_contents(gio::Cancellable::NONE) {
            let text = String::from_utf8_lossy(&contents);
            self.buffer.set_text(&text);
        }
    }

    pub fn save_file(&self, file: &gio::File) {
        let text = self.buffer.text(&self.buffer.start_iter(), &self.buffer.end_iter(), false);
        let _ = file.replace_contents(
            text.as_bytes(),
            None,
            false,
            gio::FileCreateFlags::REPLACE_DESTINATION,
            gio::Cancellable::NONE,
        );
    }

    pub fn get_text(&self) -> String {
        self.buffer.text(&self.buffer.start_iter(), &self.buffer.end_iter(), false)
            .to_string()
    }

    pub fn set_font(&self, font: &str, size: u32) {
        let provider = gtk::CssProvider::new();
        let css = format!(
            "textview {{ font-family: '{}'; font-size: {}px; }}",
            font, size
        );
        provider.load_from_string(&css);
        gtk::style_context_add_provider_for_display(
            &self.view.clone().upcast::<gtk::Widget>().display(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn toggle_line_numbers(&self, show: bool) {
        self.view.set_show_line_numbers(show);
    }

    pub fn toggle_word_wrap(&self, wrap: bool) {
        if wrap {
            self.view.set_wrap_mode(gtk::WrapMode::Word);
        } else {
            self.view.set_wrap_mode(gtk::WrapMode::None);
        }
    }

    fn get_cursor_iter(&self) -> gtk::TextIter {
        let buffer = self.buffer.upcast_ref::<gtk::TextBuffer>();
        let insert_mark = buffer.get_insert();
        buffer.iter_at_mark(&insert_mark)
    }

    pub fn insert_text(&self, text: &str) {
        let buffer = self.buffer.upcast_ref::<gtk::TextBuffer>();
        let mut iter = self.get_cursor_iter();
        buffer.insert(&mut iter, text);
    }

    pub fn insert_around_selection(&self, before: &str, after: &str) {
        let buffer = self.buffer.upcast_ref::<gtk::TextBuffer>();
        if let Some((start, end)) = buffer.selection_bounds() {
            buffer.insert(&mut end.clone(), after);
            buffer.insert(&mut start.clone(), before);
        } else {
            let mut iter = self.get_cursor_iter();
            buffer.insert(&mut iter, before);
            buffer.insert(&mut iter, after);
        }
    }

    pub fn insert_at_line_start(&self, prefix: &str) {
        let buffer = self.buffer.upcast_ref::<gtk::TextBuffer>();
        let cursor_iter = self.get_cursor_iter();
        let mut line_start = cursor_iter;
        line_start.set_line_index(0);
        buffer.insert(&mut line_start, prefix);
    }
}
