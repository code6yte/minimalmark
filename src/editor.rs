use gio::prelude::*;
use gtk::prelude::*;
use gtk::gdk;
use sourceview5::prelude::*;
use sourceview5::{LanguageManager, Buffer, View, StyleSchemeManager};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct EditorPane {
    buffer: Buffer,
    view: View,
    hide_tag: gtk::TextTag,
    bold_tag: gtk::TextTag,
    italic_tag: gtk::TextTag,
    strike_tag: gtk::TextTag,
    code_tag: gtk::TextTag,
    h1_tag: gtk::TextTag,
    h2_tag: gtk::TextTag,
    h3_tag: gtk::TextTag,
    is_source_view: Rc<Cell<bool>>,
    live_preview_enabled: Rc<Cell<bool>>,
    is_applying: Rc<Cell<bool>>,
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

        let tag_bold = gtk::TextTag::builder()
            .name("lp-bold")
            .weight(700)
            .build();
        buffer.tag_table().add(&tag_bold);

        let tag_italic = gtk::TextTag::builder()
            .name("lp-italic")
            .style(gtk::pango::Style::Italic)
            .build();
        buffer.tag_table().add(&tag_italic);

        let tag_strike = gtk::TextTag::builder()
            .name("lp-strike")
            .strikethrough(true)
            .build();
        buffer.tag_table().add(&tag_strike);

        let tag_code = gtk::TextTag::builder()
            .name("lp-code")
            .family("monospace")
            .background_rgba(&gdk::RGBA::new(0.95, 0.95, 0.97, 1.0))
            .build();
        buffer.tag_table().add(&tag_code);

        let tag_h1 = gtk::TextTag::builder()
            .name("lp-h1")
            .scale(1.8)
            .weight(700)
            .build();
        buffer.tag_table().add(&tag_h1);

        let tag_h2 = gtk::TextTag::builder()
            .name("lp-h2")
            .scale(1.5)
            .weight(700)
            .build();
        buffer.tag_table().add(&tag_h2);

        let tag_h3 = gtk::TextTag::builder()
            .name("lp-h3")
            .scale(1.2)
            .weight(700)
            .build();
        buffer.tag_table().add(&tag_h3);

        let tag_hide = gtk::TextTag::builder()
            .name("lp-hide")
            .scale(0.1)
            .foreground_rgba(&gdk::RGBA::new(0.0, 0.0, 0.0, 0.0))
            .build();
        buffer.tag_table().add(&tag_hide);

        buffer.set_text("# Start writing Markdown...\n\nType or paste your content here.");

        Self {
            buffer,
            view,
            hide_tag: tag_hide,
            bold_tag: tag_bold,
            italic_tag: tag_italic,
            strike_tag: tag_strike,
            code_tag: tag_code,
            h1_tag: tag_h1,
            h2_tag: tag_h2,
            h3_tag: tag_h3,
            is_source_view: Rc::new(Cell::new(false)),
            live_preview_enabled: Rc::new(Cell::new(true)),
            is_applying: Rc::new(Cell::new(false)),
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub fn set_live_preview(&self, enabled: bool) {
        self.live_preview_enabled.set(enabled);
        if enabled {
            self.apply_inline_preview();
        } else {
            self.remove_live_preview_tags();
        }
    }

    pub fn is_live_preview_enabled(&self) -> bool {
        self.live_preview_enabled.get()
    }

    pub fn toggle_source_view(&self) -> bool {
        let new_state = !self.is_source_view.get();
        self.is_source_view.set(new_state);
        if new_state {
            self.remove_live_preview_tags();
        } else if self.live_preview_enabled.get() {
            self.apply_inline_preview();
        }
        new_state
    }

    pub fn set_source_view(&self, enabled: bool) {
        if enabled == self.is_source_view.get() { return; }
        self.is_source_view.set(enabled);
        if enabled {
            self.remove_live_preview_tags();
        } else if self.live_preview_enabled.get() {
            self.apply_inline_preview();
        }
    }

    pub fn is_source_view(&self) -> bool {
        self.is_source_view.get()
    }

    pub fn remove_live_preview_tags(&self) {
        let tag_names = ["lp-hide", "lp-bold", "lp-italic", "lp-strike", "lp-code", "lp-h1", "lp-h2", "lp-h3"];
        for name in &tag_names {
            if let Some(tag) = self.buffer.tag_table().lookup(name) {
                let start = self.buffer.start_iter();
                let end = self.buffer.end_iter();
                self.buffer.remove_tag(&tag, &start, &end);
            }
        }
    }

    pub fn apply_inline_preview(&self) {
        if self.is_applying.get() || self.is_source_view.get() { return; }
        self.is_applying.set(true);
        self.remove_live_preview_tags();

        let text = self.get_text();
        let bytes = text.as_bytes();
        let len = text.len();
        if len == 0 {
            self.is_applying.set(false);
            return;
        }

        let cursor = self.get_cursor_iter();
        let cursor_line = cursor.line();

        // Process headings on all lines except cursor line
        let mut line_start = 0i64;
        let mut line_idx = 0i32;
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'\n' || i == len - 1 {
                let end = if i == len - 1 { i + 1 } else { i };
                if line_idx != cursor_line {
                    self.apply_heading_format(line_start as i32, end as i32);
                }
                line_start = i as i64 + 1;
                line_idx += 1;
            }
        }

        self.apply_inline_patterns(b"**", b"**", "lp-bold");
        self.apply_inline_patterns(b"__", b"__", "lp-bold");
        self.apply_inline_patterns(b"*", b"*", "lp-italic");
        self.apply_inline_patterns(b"_", b"_", "lp-italic");
        self.apply_inline_patterns(b"~~", b"~~", "lp-strike");
        self.apply_inline_patterns(b"`", b"`", "lp-code");

        self.is_applying.set(false);
    }

    fn apply_heading_format(&self, line_start: i32, line_end: i32) {
        if line_end <= line_start { return; }
        let text = self.get_text();
        let bytes = text.as_bytes();
        if line_start as usize >= bytes.len() { return; }
        let line_bytes = &bytes[line_start as usize..line_end as usize];
        let trimmed = line_bytes.iter().position(|&b| b != b' ');
        let content_start = match trimmed {
            Some(i) => line_start + i as i32,
            None => return,
        };

        let mut level = 0u32;
        let mut pos = content_start;
        while pos < line_end && bytes[pos as usize] == b'#' {
            level += 1;
            pos += 1;
        }
        if level == 0 || level > 6 || pos >= line_end || bytes[pos as usize] != b' ' {
            return;
        }

        let tag_name = match level {
            1 => "lp-h1",
            2 => "lp-h2",
            3 => "lp-h3",
            _ => return,
        };

        // Hide the # markers and space
        let start = self.buffer.iter_at_offset(content_start as i32);
        let after_hash = self.buffer.iter_at_offset((pos + 1) as i32);
        self.buffer.apply_tag(&self.hide_tag, &start, &after_hash);

        // Apply heading tag to the rest of the line
        if let Some(tag) = self.buffer.tag_table().lookup(tag_name) {
            if pos + 1 < line_end {
                let title_start = self.buffer.iter_at_offset((pos + 1) as i32);
                let title_end = self.buffer.iter_at_offset(line_end as i32);
                self.buffer.apply_tag(&tag, &title_start, &title_end);
            }
        }
    }

    fn apply_inline_patterns(&self, open: &[u8], close: &[u8], tag_name: &str) {
        let text = self.get_text();
        let bytes = text.as_bytes();
        let len = bytes.len();
        if len == 0 { return; }
        if open.is_empty() || close.is_empty() { return; }

        let Some(tag) = self.buffer.tag_table().lookup(tag_name) else { return };

        let mut i = 0;
        while i < len {
            // Find opening marker
            let open_found = bytes[i..].windows(open.len()).position(|w| w == open);
            let Some(open_pos) = open_found else { break };
            let marker_start = i + open_pos;

            // For italic, skip ** and __ (bold markers)
            if (open == b"*" || open == b"_") && marker_start + 2 <= len {
                let double = if open == b"*" { b"**" } else { b"__" };
                if marker_start + 2 <= len && &bytes[marker_start..marker_start+2] == double {
                    i = marker_start + 2;
                    continue;
                }
            }

            let content_start = marker_start + open.len();

            // Find closing marker
            let search_from = content_start;
            let close_found = bytes[search_from..].windows(close.len()).position(|w| w == close);
            let Some(close_pos) = close_found else { break };
            let marker_end = search_from + close_pos;

            // For italic, skip ** and __
            if (close == b"*" || close == b"_") && marker_end + 2 <= len {
                let double = if close == b"*" { b"**" } else { b"__" };
                if marker_end + 2 <= len && &bytes[marker_end..marker_end+2] == double {
                    i = marker_end + 2;
                    continue;
                }
            }

            let content_end = marker_end;

            // Don't apply if empty content
            if content_start >= content_end {
                i = marker_end + close.len();
                continue;
            }

            // Hide opening marker
            let s = self.buffer.iter_at_offset(marker_start as i32);
            let e = self.buffer.iter_at_offset(content_start as i32);
            self.buffer.apply_tag(&self.hide_tag, &s, &e);

            // Apply formatting tag to content
            let s = self.buffer.iter_at_offset(content_start as i32);
            let e = self.buffer.iter_at_offset(content_end as i32);
            self.buffer.apply_tag(&tag, &s, &e);

            // Hide closing marker
            let s = self.buffer.iter_at_offset(content_end as i32);
            let e = self.buffer.iter_at_offset((marker_end + close.len()) as i32);
            self.buffer.apply_tag(&self.hide_tag, &s, &e);

            i = marker_end + close.len();
        }
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
        let insert_mark = buffer.get_insert();
        let pos = buffer.iter_at_mark(&insert_mark).offset();
        let mut iter = buffer.iter_at_offset(pos);
        buffer.insert(&mut iter, text);
    }

    pub fn insert_around_selection(&self, before: &str, after: &str) {
        let buffer = self.buffer.upcast_ref::<gtk::TextBuffer>();
        if let Some((start, end)) = buffer.selection_bounds() {
            let start_off = start.offset();
            let end_off = end.offset();
            let selected_text = buffer.text(&start, &end, false);
            let new_text = format!("{}{}{}", before, selected_text, after);
            let mut start_iter = buffer.iter_at_offset(start_off);
            let mut end_iter = buffer.iter_at_offset(end_off);
            buffer.delete(&mut start_iter, &mut end_iter);
            let mut insert_iter = buffer.iter_at_offset(start_off);
            buffer.insert(&mut insert_iter, &new_text);
            let new_start = buffer.iter_at_offset(start_off);
            let new_end = buffer.iter_at_offset(start_off + before.len() as i32 + selected_text.len() as i32);
            buffer.select_range(&new_start, &new_end);
        } else {
            let insert_mark = buffer.get_insert();
            let pos = buffer.iter_at_mark(&insert_mark).offset();
            buffer.insert(&mut buffer.iter_at_offset(pos), before);
            buffer.insert(&mut buffer.iter_at_offset(pos + before.len() as i32), after);
            let cursor_pos = pos + before.len() as i32;
            buffer.place_cursor(&buffer.iter_at_offset(cursor_pos));
        }
    }

    pub fn save_to_file(&self, path: &str) -> bool {
        let text = self.buffer.text(&self.buffer.start_iter(), &self.buffer.end_iter(), false);
        let file = gio::File::for_path(path);
        match file.replace_contents(
            text.as_bytes(),
            None,
            false,
            gio::FileCreateFlags::REPLACE_DESTINATION,
            gio::Cancellable::NONE,
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn save_current_file(&self, path: &str) -> bool {
        self.save_to_file(path)
    }

    pub fn insert_at_line_start(&self, prefix: &str) {
        let buffer = self.buffer.upcast_ref::<gtk::TextBuffer>();
        let cursor_iter = self.get_cursor_iter();
        let line_num = cursor_iter.line();
        let mut line_start = buffer.iter_at_line_index(line_num, 0).unwrap();
        buffer.insert(&mut line_start, prefix);
    }

    pub fn setup_context_menu(&self, popover: gtk::Popover) {
        use gtk::GestureClick;
        let gesture = GestureClick::new();
        gesture.set_button(3);
        gesture.connect_pressed(move |g, _count, x, y| {
            let rect = gdk::Rectangle::new(x as i32, y as i32, 1, 1);
            popover.set_pointing_to(Some(&rect));
            if popover.parent().is_none() {
                if let Some(w) = g.widget() {
                    popover.set_parent(&w);
                }
            }
            popover.popup();
        });
        self.view.add_controller(gesture);
    }

    // Auto-pair brackets and quotes
    pub fn setup_auto_pair(&self) {
        use gtk::EventControllerKey;
        let buffer = self.buffer.clone();
        let controller = EventControllerKey::new();
        controller.connect_key_pressed(move |_ctrl, key, _code, _state| {
            let closing = match key {
                gtk::gdk::Key::bracketleft => Some("]"),
                gtk::gdk::Key::braceleft => Some("}"),
                gtk::gdk::Key::parenleft => Some(")"),
                gtk::gdk::Key::quoteright | gtk::gdk::Key::quotedbl => {
                    let insert_mark = buffer.get_insert();
                    let iter = buffer.iter_at_mark(&insert_mark);
                    let ch = buffer.text(&iter, &{
                        let mut next = iter.clone();
                        next.forward_char();
                        next
                    }, false).to_string();
                    // Only auto-close if not already before a closing quote
                    if ch == "\"" || ch == "'" {
                        return glib::Propagation::Proceed;
                    }
                    Some(if key == gtk::gdk::Key::quoteright { "'" } else { "\"" })
                }
                _ => return glib::Propagation::Proceed,
            };

            if let Some(c) = closing {
                let mut iter = buffer.iter_at_mark(&buffer.get_insert());
                buffer.insert(&mut iter, c);
                let mut back = buffer.iter_at_mark(&buffer.get_insert());
                back.backward_char();
                buffer.place_cursor(&back);
            }
            glib::Propagation::Proceed
        });
        self.view.add_controller(controller);
    }
}
