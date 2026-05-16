use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, ScrolledWindow, EventControllerKey, ToggleButton, SearchEntry, Stack, MenuButton, PopoverMenu, StackSwitcher};
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
    window: Rc<Cell<Option<ApplicationWindow>>>,
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

        // Sidebar toggle button
        let sidebar_toggle = ToggleButton::builder()
            .icon_name("sidebar-show-symbolic")
            .tooltip_text("Toggle Outline (Ctrl+H)")
            .active(true)
            .build();
        header.pack_start(&sidebar_toggle);

        // View mode stack switcher
        let stack_switcher = StackSwitcher::builder()
            .build();

        // Settings button
        let settings_btn = gtk::Button::from_icon_name("emblem-system-symbolic");
        settings_btn.set_tooltip_text(Some("Preferences (Ctrl+,)"));
        header.pack_end(&settings_btn);

        // Menu button
        let menu_button = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Menu")
            .build();

        // Build popover menu
        let menu_model = Self::build_menu_model();
        let popover = PopoverMenu::from_model(Some(&menu_model));
        menu_button.set_popover(Some(&popover));

        header.pack_end(&menu_button);
        header.pack_end(&stack_switcher);

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

        // Clone before any moves into closures
        let stack_for_toggle = stack.clone();
        let sidebar_container_clone = sidebar_container.clone();
        let separator_clone = separator.clone();

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

        // Sidebar toggle
        sidebar_toggle.connect_toggled({
            let sidebar_container = sidebar_container_clone.clone();
            let separator = separator_clone.clone();
            let sidebar_visible = Rc::new(Cell::new(true));
            let sidebar_visible_ref = sidebar_visible.clone();
            move |btn| {
                let visible = btn.is_active();
                sidebar_container.set_visible(visible);
                separator.set_visible(visible);
                sidebar_visible_ref.set(visible);
            }
        });

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

        // Keyboard shortcuts
        let window_clone_shortcuts = window.clone();
        let editor_clone_shortcuts = editor.clone();
        let mode_clone_shortcuts = mode.clone();
        let stack_clone_shortcuts = stack.clone();

        let event_controller = EventControllerKey::new();
        shortcuts::setup_shortcuts(&event_controller, move |action| {
            Self::handle_shortcut(
                action,
                &window_clone_shortcuts,
                &editor_clone_shortcuts,
                &mode_clone_shortcuts,
                &stack_clone_shortcuts,
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
            preview_scroll,
            stack,
            mode,
            sidebar_visible: Rc::new(Cell::new(true)),
        }
    }

    fn build_menu_model() -> gio::MenuModel {
        let menu = gio::Menu::new();

        let file_section = gio::Menu::new();
        file_section.append(Some("New"), Some("app.new"));
        file_section.append(Some("Open"), Some("app.open"));
        file_section.append(Some("Save"), Some("app.save"));
        menu.append_section(None, &file_section);

        let edit_section = gio::Menu::new();
        edit_section.append(Some("Bold"), Some("win.bold"));
        edit_section.append(Some("Italic"), Some("win.italic"));
        edit_section.append(Some("Strikethrough"), Some("win.strikethrough"));
        edit_section.append(Some("Inline Code"), Some("win.inline_code"));
        menu.append_section(None, &edit_section);

        let view_section = gio::Menu::new();
        view_section.append(Some("Preferences"), Some("win.preferences"));
        view_section.append(Some("Toggle Sidebar"), Some("win.toggle_sidebar"));
        menu.append_section(None, &view_section);

        menu.upcast()
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

    fn handle_shortcut(
        action: &str,
        window: &ApplicationWindow,
        editor: &EditorPane,
        mode: &Rc<Cell<ViewMode>>,
        stack: &Stack,
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
            "save" => {}
            "fullscreen" => {
                if window.is_fullscreen() { window.unfullscreen(); }
                else { window.fullscreen(); }
            }
            _ => {}
        }
    }
}
