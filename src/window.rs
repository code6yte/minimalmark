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
use crate::markdown::count_stats;
use crate::shortcuts;
use crate::ctxmenu::{self, build_context_menu};
use crate::settings::AppSettings;
use crate::settingsdialog;
use std::cell::RefCell;

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
    settings: Rc<RefCell<AppSettings>>,
    is_focus_mode: Rc<Cell<bool>>,
    is_typewriter_mode: Rc<Cell<bool>>,
    is_hemingway_mode: Rc<Cell<bool>>,
    current_file: Rc<Cell<Option<String>>>,
    window: Rc<Cell<Option<ApplicationWindow>>>,
    editor_scroll: ScrolledWindow,
    paned: Paned,
    mode: Rc<Cell<ViewMode>>,
}

impl MainWindow {
    pub fn new(window: &ApplicationWindow, file: Option<&gio::File>, _is_dark: bool) -> Self {
        let settings = Rc::new(RefCell::new(AppSettings::load()));

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

        // Settings button
        let settings_btn = gtk::Button::from_icon_name("emblem-system-symbolic");
        settings_btn.set_tooltip_text(Some("Settings (Ctrl+,)"));
        header.pack_end(&settings_btn);

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

        {
            let s = settings.borrow();
            editor.set_font(&s.editor_font, s.editor_font_size);
            editor.toggle_word_wrap(s.word_wrap);
            editor.toggle_line_numbers(s.show_line_numbers);
            editor.set_live_preview(s.live_preview);
            if s.auto_pair {
                editor.setup_auto_pair();
            }
        }

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

        // Clone before any moves into closures
        let paned_for_toggle = paned.clone();
        let editor_scroll_for_toggle = editor_scroll.clone();
        let preview_scroll_for_toggle = preview_scroll.clone();
        let paned_for_shortcut = paned.clone();
        let editor_scroll_for_shortcut = editor_scroll.clone();
        let preview_scroll_for_shortcut = preview_scroll.clone();

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
        let mode = Rc::new(Cell::new(ViewMode::Split));

        editor_mode_btn.connect_toggled({
            let paned = paned_for_toggle.clone();
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
            let paned = paned_for_toggle.clone();
            let editor_scroll = editor_scroll_for_toggle.clone();
            let preview_scroll = preview_scroll_for_toggle.clone();
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
            let paned = paned_for_toggle.clone();
            let editor_scroll = editor_scroll_for_toggle.clone();
            let preview_scroll = preview_scroll_for_toggle.clone();
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

        // Live preview on text changes + outline update
        let preview_clone = preview.clone();
        let statusbar_clone = statusbar.clone();
        let sidebar_clone = sidebar.clone();
        let editor_preview = editor.clone();
        editor.buffer().connect_changed(move |buffer| {
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
            preview_clone.update(&text);
            let (words, chars, _, _, reading_time) = count_stats(&text);
            statusbar_clone.update_stats(words, chars, reading_time);
            sidebar_clone.update_outline(&text);
            if editor_preview.is_live_preview_enabled() {
                editor_preview.apply_inline_preview();
            }
        });

        // Update cursor position and re-apply inline preview on cursor move
        let statusbar_clone = statusbar.clone();
        let editor_move = editor.clone();
        editor.buffer().connect_mark_set(move |_buffer, iter, mark| {
            let line = iter.line() + 1;
            let col = iter.line_index() + 1;
            statusbar_clone.update_cursor(line as usize, col as usize);
            if mark.name().as_deref() == Some("insert") && editor_move.is_live_preview_enabled() {
                editor_move.apply_inline_preview();
            }
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

        // Settings button
        let settings_clone2 = settings.clone();
        let window_clone = window.clone();
        settings_btn.connect_clicked(move |_| {
            settingsdialog::show_settings(&window_clone, &settings_clone2);
        });

        // Context menu
        let ctx_popover = build_context_menu(&editor);
        editor.setup_context_menu(ctx_popover.clone());

        // Keyboard shortcuts
        let window_clone = window.clone();
        let editor_clone = editor.clone();
        let mode_clone = mode.clone();
        let paned_clone = paned_for_shortcut;
        let editor_scroll_clone = editor_scroll_for_shortcut;
        let preview_scroll_clone = preview_scroll_for_shortcut;

        let event_controller = EventControllerKey::new();
        shortcuts::setup_shortcuts(&event_controller, move |action| {
            Self::handle_shortcut(
                action,
                &window_clone,
                &editor_clone,
                &mode_clone,
                &paned_clone,
                &editor_scroll_clone,
                &preview_scroll_clone,
            );
        });
        editor.view().upcast_ref::<gtk::Widget>().add_controller(event_controller);

        // Hemingway mode - intercept backspace/delete
        {
            let is_hemingway = Rc::new(Cell::new(false));
            let h_is_hemingway = is_hemingway.clone();
            let h_controller = EventControllerKey::new();
            h_controller.connect_key_pressed(move |_ctrl, key, _code, _state| {
                if h_is_hemingway.get() && (key == gtk::gdk::Key::BackSpace || key == gtk::gdk::Key::Delete) {
                    return glib::Propagation::Stop;
                }
                glib::Propagation::Proceed
            });
            editor.view().add_controller(h_controller);
        }

        // Load file if provided
        if let Some(f) = file {
            editor.load_file(f);
            if let Some(name) = f.basename() {
                window.set_title(Some(&format!("MinimalMark - {}", name.to_string_lossy())));
            }
        }

        let text = editor.buffer().text(&editor.buffer().start_iter(), &editor.buffer().end_iter(), false);
        preview.update(&text);
        let (words, chars, _, _, reading_time) = count_stats(&text);
        statusbar.update_stats(words, chars, reading_time);
        sidebar.update_outline(&text);

        Self {
            container,
            editor,
            preview,
            sidebar,
            statusbar,
            settings,
            is_focus_mode: Rc::new(Cell::new(false)),
            is_typewriter_mode: Rc::new(Cell::new(false)),
            is_hemingway_mode: Rc::new(Cell::new(false)),
            current_file: Rc::new(Cell::new(None)),
            window: Rc::new(Cell::new(Some(window.clone()))),
            editor_scroll,
            paned,
            mode,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    fn handle_shortcut(
        action: &str,
        window: &ApplicationWindow,
        editor: &EditorPane,
        mode: &Rc<Cell<ViewMode>>,
        paned: &Paned,
        editor_scroll: &ScrolledWindow,
        preview_scroll: &ScrolledWindow,
    ) {
        match action {
            "bold" => editor.insert_around_selection("**", "**"),
            "italic" => editor.insert_around_selection("*", "*"),
            "strikethrough" => editor.insert_around_selection("~~", "~~"),
            "link" => editor.insert_around_selection("[", "](url)"),
            "inline_code" => editor.insert_around_selection("`", "`"),
            "image" => editor.insert_around_selection("![alt](", ")"),
            "table" => editor.insert_text("\n| Col 1 | Col 2 |\n|-------|-------|\n| Cell  | Cell  |\n"),
            "heading1" => editor.insert_at_line_start("# "),
            "heading2" => editor.insert_at_line_start("## "),
            "heading3" => editor.insert_at_line_start("### "),
            "bullet_list" => editor.insert_at_line_start("- "),
            "numbered_list" => editor.insert_at_line_start("1. "),
            "blockquote" => editor.insert_at_line_start("> "),
            "toggle_source" => {
                editor.toggle_source_view();
            }
            "focus_mode" => {
                // Toggle focus mode via settings
            }
            "typewriter_mode" => {
                // Toggle typewriter mode via settings
            }
            "command_palette" => {
                // Show command palette
            }
            "settings" => {
                // Show settings window
            }
            "save" => {}
            "fullscreen" => {
                if window.is_fullscreen() { window.unfullscreen(); }
                else { window.fullscreen(); }
            }
            _ => {}
        }
    }
}
