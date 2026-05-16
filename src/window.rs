use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, ScrolledWindow, Paned, EventControllerKey, ToggleButton, SearchEntry};
use adw::ApplicationWindow;
use std::cell::Cell;
use std::rc::Rc;

use crate::editor::EditorPane;
use crate::preview::PreviewPane;
use crate::sidebar::Sidebar;
use crate::statusbar::StatusBar;
use crate::markdown::{render_markdown, count_stats};
use crate::shortcuts;
use crate::settings::AppSettings;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Editor,
    Preview,
    Split,
}

#[derive(Clone)]
pub struct MainWindow {
    container: GtkBox,
    editor: EditorPane,
    preview: PreviewPane,
    sidebar: Sidebar,
    statusbar: StatusBar,
    settings: AppSettings,
}

impl MainWindow {
    pub fn new(window: &ApplicationWindow, file: Option<&gio::File>, _is_dark: bool) -> Self {
        let settings = AppSettings::load();

        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .build();

        let header = gtk::HeaderBar::new();

        let new_btn = gtk::Button::from_icon_name("document-new-symbolic");
        new_btn.set_tooltip_text(Some("New (Ctrl+N)"));
        header.pack_start(&new_btn);

        let open_btn = gtk::Button::from_icon_name("document-open-symbolic");
        open_btn.set_tooltip_text(Some("Open (Ctrl+O)"));
        header.pack_start(&open_btn);

        let save_btn = gtk::Button::from_icon_name("document-save-symbolic");
        save_btn.set_tooltip_text(Some("Save (Ctrl+S)"));
        header.pack_end(&save_btn);

        let search_btn = ToggleButton::builder()
            .icon_name("edit-find-symbolic")
            .tooltip_text("Search (Ctrl+F)")
            .build();
        header.pack_end(&search_btn);

        let editor_mode_btn = ToggleButton::builder()
            .icon_name("document-edit-symbolic")
            .tooltip_text("Editor")
            .build();
        header.pack_end(&editor_mode_btn);

        let preview_mode_btn = ToggleButton::builder()
            .icon_name("document-preview-symbolic")
            .tooltip_text("Preview")
            .build();
        header.pack_end(&preview_mode_btn);

        let split_mode_btn = ToggleButton::builder()
            .icon_name("view-split-left-right-symbolic")
            .tooltip_text("Split")
            .active(true)
            .build();
        header.pack_end(&split_mode_btn);

        let main_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .build();

        let sidebar = Sidebar::new();
        main_box.append(sidebar.container());

        let separator = gtk::Separator::new(Orientation::Vertical);
        main_box.append(&separator);

        let content_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .build();

        let editor = EditorPane::new();
        let preview = PreviewPane::new();
        let statusbar = StatusBar::new();

        editor.set_font(&settings.editor_font, settings.editor_font_size);
        editor.toggle_word_wrap(settings.word_wrap);
        editor.toggle_line_numbers(settings.show_line_numbers);

        let editor_scroll = ScrolledWindow::new();
        editor_scroll.set_hexpand(true);
        editor_scroll.set_vexpand(true);
        editor_scroll.set_child(Some(editor.view().upcast_ref::<gtk::Widget>()));

        let preview_scroll = ScrolledWindow::new();
        preview_scroll.set_hexpand(true);
        preview_scroll.set_vexpand(true);
        preview_scroll.set_child(Some(preview.widget().upcast_ref::<gtk::Widget>()));

        let paned = Paned::builder()
            .orientation(Orientation::Horizontal)
            .wide_handle(false)
            .build();
        paned.set_start_child(Some(&editor_scroll));
        paned.set_end_child(Some(&preview_scroll));
        paned.set_position(600);

        content_box.append(&paned);

        let search_bar = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .margin_start(8)
            .margin_end(8)
            .margin_top(4)
            .margin_bottom(4)
            .build();
        search_bar.add_css_class("toolbar");

        let search_entry = SearchEntry::builder()
            .placeholder_text("Search...")
            .build();
        let replace_entry = SearchEntry::builder()
            .placeholder_text("Replace...")
            .build();
        let replace_btn = gtk::Button::with_label("Replace");
        let replace_all_btn = gtk::Button::with_label("All");

        search_bar.append(&search_entry);
        search_bar.append(&replace_entry);
        search_bar.append(&replace_btn);
        search_bar.append(&replace_all_btn);
        search_bar.set_visible(false);

        content_box.append(&search_bar);
        content_box.append(statusbar.container());

        main_box.append(&content_box);

        container.append(&header);
        container.append(&main_box);

        // View mode toggling
        let paned = paned.clone();
        let mode = Rc::new(Cell::new(ViewMode::Split));

        editor_mode_btn.connect_toggled({
            let paned = paned.clone();
            let editor_scroll = editor_scroll.clone();
            let pmode = preview_mode_btn.clone();
            let smode = split_mode_btn.clone();
            let mode = mode.clone();
            move |btn| {
                if btn.is_active() {
                    mode.set(ViewMode::Editor);
                    pmode.set_active(false);
                    smode.set_active(false);
                    paned.set_end_child(Option::<&gtk::Widget>::None);
                }
            }
        });

        preview_mode_btn.connect_toggled({
            let paned = paned.clone();
            let editor_scroll = editor_scroll.clone();
            let preview_scroll = preview_scroll.clone();
            let emode = editor_mode_btn.clone();
            let smode = split_mode_btn.clone();
            let mode = mode.clone();
            move |btn| {
                if btn.is_active() {
                    mode.set(ViewMode::Preview);
                    emode.set_active(false);
                    smode.set_active(false);
                    paned.set_start_child(Option::<&gtk::Widget>::None);
                    paned.set_end_child(Some(&preview_scroll));
                } else if mode.get() == ViewMode::Preview {
                    paned.set_start_child(Some(&editor_scroll));
                    paned.set_end_child(Some(&preview_scroll));
                    mode.set(ViewMode::Split);
                    smode.set_active(true);
                }
            }
        });

        split_mode_btn.connect_toggled({
            let editor_scroll = editor_scroll.clone();
            let preview_scroll = preview_scroll.clone();
            let emode = editor_mode_btn.clone();
            let pmode = preview_mode_btn.clone();
            let mode = mode.clone();
            move |btn| {
                if btn.is_active() {
                    mode.set(ViewMode::Split);
                    emode.set_active(false);
                    pmode.set_active(false);
                    paned.set_start_child(Some(&editor_scroll));
                    paned.set_end_child(Some(&preview_scroll));
                } else if mode.get() == ViewMode::Split {
                    mode.set(ViewMode::Editor);
                    emode.set_active(true);
                    paned.set_end_child(Option::<&gtk::Widget>::None);
                }
            }
        });

        // Live preview on text changes
        let preview_clone = preview.clone();
        let statusbar_clone = statusbar.clone();
        editor.buffer().connect_changed(move |buffer| {
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
            let html = render_markdown(&text);
            preview_clone.update(&html);
            let (words, chars, _, _, reading_time) = count_stats(&text);
            statusbar_clone.update_stats(words, chars, reading_time);
        });

        let statusbar_clone = statusbar.clone();
        editor.buffer().connect_mark_set(move |_buffer, iter, _| {
            let line = iter.line() + 1;
            let col = iter.line_index() + 1;
            statusbar_clone.update_cursor(line as usize, col as usize);
        });

        // Search toggle
        let search_bar_weak = search_bar.clone();
        let search_entry_weak = search_entry.clone();
        search_btn.connect_toggled(move |btn| {
            search_bar_weak.set_visible(btn.is_active());
            if btn.is_active() {
                search_entry_weak.grab_focus();
            }
        });

        // Keyboard shortcuts
        let window_clone = window.clone();
        let editor_clone = editor.clone();
        let event_controller = EventControllerKey::new();
        shortcuts::setup_shortcuts(&event_controller, move |action| {
            Self::handle_shortcut(action, &window_clone, &editor_clone);
        });
        editor.view().upcast_ref::<gtk::Widget>().add_controller(event_controller);

        // Load file if provided
        if let Some(f) = file {
            editor.load_file(f);
        }

        let text = editor.buffer().text(&editor.buffer().start_iter(), &editor.buffer().end_iter(), false);
        let html = render_markdown(&text);
        preview.update(&html);
        let (words, chars, _, _, reading_time) = count_stats(&text);
        statusbar.update_stats(words, chars, reading_time);

        Self::setup_sidebar_callbacks(&sidebar, &editor);

        Self {
            container,
            editor,
            preview,
            sidebar,
            statusbar,
            settings,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    fn setup_sidebar_callbacks(sidebar: &Sidebar, editor: &EditorPane) {
        let e = editor.clone();
        sidebar.bold_button().connect_clicked(move |_| e.insert_around_selection("**", "**"));
        let e = editor.clone();
        sidebar.italic_button().connect_clicked(move |_| e.insert_around_selection("*", "*"));
        let e = editor.clone();
        sidebar.strike_button().connect_clicked(move |_| e.insert_around_selection("~~", "~~"));
        let e = editor.clone();
        sidebar.code_button().connect_clicked(move |_| e.insert_around_selection("`", "`"));
        let e = editor.clone();
        sidebar.h1_button().connect_clicked(move |_| e.insert_at_line_start("# "));
        let e = editor.clone();
        sidebar.h2_button().connect_clicked(move |_| e.insert_at_line_start("## "));
        let e = editor.clone();
        sidebar.h3_button().connect_clicked(move |_| e.insert_at_line_start("### "));
        let e = editor.clone();
        sidebar.ul_button().connect_clicked(move |_| e.insert_at_line_start("- "));
        let e = editor.clone();
        sidebar.ol_button().connect_clicked(move |_| e.insert_at_line_start("1. "));
        let e = editor.clone();
        sidebar.task_button().connect_clicked(move |_| e.insert_at_line_start("- [ ] "));
        let e = editor.clone();
        sidebar.link_button().connect_clicked(move |_| e.insert_around_selection("[", "](url)"));
        let e = editor.clone();
        sidebar.image_button().connect_clicked(move |_| e.insert_around_selection("![alt](", ")"));
        let e = editor.clone();
        sidebar.quote_button().connect_clicked(move |_| e.insert_at_line_start("> "));
        let e = editor.clone();
        sidebar.table_button().connect_clicked(move |_| {
            e.insert_text("\n| Col 1 | Col 2 |\n|-------|-------|\n| Cell  | Cell  |\n");
        });
        let e = editor.clone();
        sidebar.codeblock_button().connect_clicked(move |_| {
            e.insert_around_selection("```\n", "\n```");
        });
        let e = editor.clone();
        sidebar.hr_button().connect_clicked(move |_| {
            e.insert_text("\n---\n");
        });
    }

    fn handle_shortcut(action: &str, window: &ApplicationWindow, editor: &EditorPane) {
        match action {
            "bold" => editor.insert_around_selection("**", "**"),
            "italic" => editor.insert_around_selection("*", "*"),
            "strikethrough" => editor.insert_around_selection("~~", "~~"),
            "link" => editor.insert_around_selection("[", "](url)"),
            "inline_code" => editor.insert_around_selection("`", "`"),
            "image" => editor.insert_around_selection("![alt](", ")"),
            "table" => editor.insert_text("\n| Col 1 | Col 2 |\n|-------|-------|\n| Cell  | Cell  |\n"),
            "save" => {},
            "fullscreen" => {
                if window.is_fullscreen() { window.unfullscreen(); }
                else { window.fullscreen(); }
            }
            _ => {}
        }
    }
}
