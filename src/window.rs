use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, ScrolledWindow, EventControllerKey, ToggleButton, SearchEntry, Stack, StackSwitcher, FileChooserDialog, FileChooserAction, ResponseType};
use adw::ApplicationWindow;
use std::cell::Cell;
use std::rc::Rc;

use crate::editor::EditorPane;
use crate::preview::PreviewPane;
use crate::sidebar::Sidebar;
use crate::statusbar::StatusBar;
use crate::markdown::count_stats;
use crate::shortcuts;
use crate::ctxmenu::build_context_menu;
use crate::settings::AppSettings;
use crate::settingsdialog;
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Source,
    Live,
    Preview,
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
    window: Rc<RefCell<Option<ApplicationWindow>>>,
    editor_scroll: ScrolledWindow,
    preview_scroll: ScrolledWindow,
    stack: Stack,
    mode: Rc<Cell<ViewMode>>,
    sidebar_visible: Rc<Cell<bool>>,
}

impl MainWindow {
    pub fn new(window: &ApplicationWindow, file: Option<&gio::File>, _is_dark: bool) -> Self {
        let settings = Rc::new(RefCell::new(AppSettings::load()));

        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .build();

        let header = gtk::HeaderBar::new();

        // View mode stack switcher
        let stack_switcher = StackSwitcher::builder()
            .build();
        header.pack_end(&stack_switcher);

        // Settings button
        let settings_btn = gtk::Button::from_icon_name("emblem-system-symbolic");
        settings_btn.set_tooltip_text(Some("Preferences (Ctrl+,)"));
        header.pack_end(&settings_btn);

        let main_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .build();

        let sidebar = Sidebar::new();
        let sidebar_container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .width_request(200)
            .height_request(100)
            .build();
        sidebar_container.append(sidebar.container());
        main_box.append(&sidebar_container);

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

        // Stack for view modes
        let stack = Stack::new();
        stack.add_named(&editor_scroll, Some("source"));
        stack.add_named(&preview_scroll, Some("preview"));
        stack.set_transition_type(gtk::StackTransitionType::Crossfade);
        stack.set_transition_duration(200);

        content_box.append(&stack);
        stack_switcher.set_stack(Some(&stack));

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

        // View mode: default to Live
        let mode = Rc::new(Cell::new(ViewMode::Live));
        stack.set_visible_child_name("source");

        // Stack visibility changed handler
        stack.connect_visible_child_notify({
            let mode = mode.clone();
            let editor = editor.clone();
            let preview = preview.clone();
            move |stack| {
                let name = stack.visible_child_name().map(|s| s.to_string()).unwrap_or_default();
                let new_mode = match name.as_str() {
                    "source" => ViewMode::Source,
                    "preview" => ViewMode::Preview,
                    _ => ViewMode::Live,
                };
                mode.set(new_mode);
                if new_mode == ViewMode::Source {
                    editor.set_source_view(true);
                } else if new_mode == ViewMode::Live {
                    editor.set_source_view(false);
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
            if editor_preview.is_live_preview_enabled() && !editor_preview.is_source_view() {
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
            if mark.name().as_deref() == Some("insert") && editor_move.is_live_preview_enabled() && !editor_move.is_source_view() {
                editor_move.apply_inline_preview();
            }
        });

        // Settings button
        let settings_clone2 = settings.clone();
        let window_clone = window.clone();
        let editor_clone_for_settings = editor.clone();
        settings_btn.connect_clicked(move |_| {
            settingsdialog::show_settings(
                &window_clone,
                &settings_clone2,
                &editor_clone_for_settings,
            );
        });

        // Context menu
        let ctx_popover = build_context_menu(&editor);
        editor.setup_context_menu(ctx_popover.clone());

        // Create current_file early for shortcuts
        let current_file = Rc::new(RefCell::new(None::<String>));

        // Keyboard shortcuts
        let window_clone_shortcuts = window.clone();
        let editor_clone_shortcuts = editor.clone();
        let mode_clone_shortcuts = mode.clone();
        let stack_clone_shortcuts = stack.clone();
        let current_file_for_shortcuts = current_file.clone();

        let event_controller = EventControllerKey::new();
        shortcuts::setup_shortcuts(&event_controller, move |action| {
            Self::handle_shortcut(
                action,
                &window_clone_shortcuts,
                &editor_clone_shortcuts,
                &mode_clone_shortcuts,
                &stack_clone_shortcuts,
                &current_file_for_shortcuts,
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
            if let Some(path) = f.path() {
                let path_str = path.to_string_lossy().to_string();
                current_file.borrow_mut().replace(path_str.clone());
                current_file_shortcuts.set(Some(path_str));
            }
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
            window: Rc::new(RefCell::new(Some(window.clone()))),
            editor_scroll,
            preview_scroll,
            stack,
            mode,
            sidebar_visible: Rc::new(Cell::new(true)),
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn editor(&self) -> &EditorPane {
        &self.editor
    }

    pub fn settings(&self) -> &Rc<RefCell<AppSettings>> {
        &self.settings
    }

    pub fn current_file(&self) -> Rc<Cell<Option<String>>> {
        self.current_file.clone()
    }

    pub fn set_current_file(&self, path: Option<String>) {
        self.current_file.set(path);
    }

    pub fn trigger_save(&self) {
        if let Some(path) = self.current_file.borrow().clone() {
            self.editor.save_current_file(&path);
        } else {
            self.trigger_save_as();
        }
    }

    pub fn trigger_save_as(&self) {
        if let Some(win) = self.window.borrow().clone() {
            let dialog = FileChooserDialog::new(
                Some("Save File"),
                Some(&win),
                FileChooserAction::Save,
                &[("_Cancel", ResponseType::Cancel), ("_Save", ResponseType::Accept)],
            );
            dialog.set_current_name("untitled.md");

            let filter = gtk::FileFilter::new();
            filter.set_name(Some("Markdown Files"));
            filter.add_pattern("*.md");
            dialog.add_filter(&filter);

            let all_filter = gtk::FileFilter::new();
            all_filter.set_name(Some("All Files"));
            all_filter.add_pattern("*");
            dialog.add_filter(&all_filter);

            let editor = self.editor.clone();
            let current_file = self.current_file.clone();
            let window_ref = self.window.clone();

            dialog.connect_response(move |dlg, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dlg.file() {
                        if let Some(path) = file.path() {
                            let path_str = path.to_string_lossy().to_string();
                            if editor.save_current_file(&path_str) {
                                current_file.borrow_mut().replace(path_str.clone());
                                if let Some(w) = window_ref.borrow().clone() {
                                    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "MinimalMark".into());
                                    w.set_title(Some(&format!("MinimalMark - {}", name)));
                                }
                            }
                        }
                    }
                }
                dlg.close();
            });

            dialog.present();
        }
    }

    fn handle_shortcut(
        action: &str,
        window: &ApplicationWindow,
        editor: &EditorPane,
        mode: &Rc<Cell<ViewMode>>,
        stack: &Stack,
        current_file: &Rc<Cell<Option<String>>>,
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
                match mode.get() {
                    ViewMode::Source => stack.set_visible_child_name("preview"),
                    ViewMode::Preview => stack.set_visible_child_name("source"),
                    ViewMode::Live => stack.set_visible_child_name("source"),
                }
            }
            "focus_mode" => {}
            "typewriter_mode" => {}
            "command_palette" => {}
            "settings" => {}
            "save" => {
                if let Some(path) = current_file.borrow().clone() {
                    editor.save_current_file(&path);
                } else {
                    let dialog = FileChooserDialog::new(
                        Some("Save File"),
                        Some(window),
                        FileChooserAction::Save,
                        &[("_Cancel", ResponseType::Cancel), ("_Save", ResponseType::Accept)],
                    );
                    dialog.set_current_name("untitled.md");

                    let filter = gtk::FileFilter::new();
                    filter.set_name(Some("Markdown Files"));
                    filter.add_pattern("*.md");
                    dialog.add_filter(&filter);

                    let all_filter = gtk::FileFilter::new();
                    all_filter.set_name(Some("All Files"));
                    all_filter.add_pattern("*");
                    dialog.add_filter(&all_filter);

                    let editor = editor.clone();
                    let current_file = current_file.clone();
                    let window_ref = window.clone();

                    dialog.connect_response(move |dlg, response| {
                        if response == ResponseType::Accept {
                            if let Some(file) = dlg.file() {
                                if let Some(path) = file.path() {
                                    let path_str = path.to_string_lossy().to_string();
                                    if editor.save_current_file(&path_str) {
                                        current_file.borrow_mut().replace(path_str.clone());
                                        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "MinimalMark".into());
                                        window_ref.set_title(Some(&format!("MinimalMark - {}", name)));
                                    }
                                }
                            }
                        }
                        dlg.close();
                    });

                    dialog.present();
                }
            }
            "fullscreen" => {
                if window.is_fullscreen() { window.unfullscreen(); }
                else { window.fullscreen(); }
            }
            _ => {}
        }
    }
}
