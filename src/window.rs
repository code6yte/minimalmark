use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, ScrolledWindow, Paned, EventControllerKey};
use adw::ApplicationWindow;

use crate::editor::EditorPane;
use crate::preview::PreviewPane;
use crate::toolbar::Toolbar;
use crate::statusbar::StatusBar;
use crate::markdown::{render_markdown, count_stats};
use crate::shortcuts;
use crate::settings::AppSettings;

#[derive(Clone)]
pub struct MainWindow {
    container: GtkBox,
    editor: EditorPane,
    preview: PreviewPane,
    toolbar: Toolbar,
    statusbar: StatusBar,
    settings: AppSettings,
}

impl MainWindow {
    pub fn new(window: &ApplicationWindow, file: Option<&gio::File>, _is_dark: bool) -> Self {
        let settings = AppSettings::load();
        
        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .build();

        let toolbar = Toolbar::new();
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
        paned.set_position((settings.preview_width_percent / 100.0 * 1280.0) as i32);

        container.append(toolbar.container());
        container.append(&paned);
        container.append(statusbar.container());

        let preview_clone = preview.clone();
        let statusbar_clone = statusbar.clone();
        editor.buffer().connect_changed(move |buffer| {
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
            let html = render_markdown(&text);
            preview_clone.update(&html);
            
            let (words, chars, _, _, reading_time) = count_stats(&text);
            statusbar_clone.update_stats(words, chars, reading_time);
        });

        let statusbar_clone_for_cursor = statusbar.clone();
        editor.buffer().connect_mark_set(move |_buffer, iter, _| {
            let line = iter.line() + 1;
            let col = iter.line_index() + 1;
            statusbar_clone_for_cursor.update_cursor(line as usize, col as usize);
        });

        let file_clone = file.map(|f| f.clone());
        let window_clone = window.clone();
        let editor_clone_shortcuts = editor.clone();
        let event_controller = EventControllerKey::new();
        shortcuts::setup_shortcuts(&event_controller, move |action| {
            Self::handle_action(action, &window_clone, &editor_clone_shortcuts, file_clone.as_ref());
        });
        editor.view().upcast_ref::<gtk::Widget>().add_controller(event_controller);

        if let Some(f) = file {
            editor.load_file(f);
        }

        let text = editor.buffer().text(&editor.buffer().start_iter(), &editor.buffer().end_iter(), false);
        let html = render_markdown(&text);
        preview.update(&html);
        
        let (words, chars, _, _, reading_time) = count_stats(&text);
        statusbar.update_stats(words, chars, reading_time);

        Self::setup_toolbar_callbacks(&toolbar, &editor, window);

        Self {
            container,
            editor,
            preview,
            toolbar,
            statusbar,
            settings,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    fn setup_toolbar_callbacks(
        toolbar: &Toolbar,
        editor: &EditorPane,
        window: &ApplicationWindow,
    ) {
        let editor_clone = editor.clone();
        toolbar.bold_button().connect_clicked(move |_| {
            editor_clone.insert_around_selection("**", "**");
        });

        let editor_clone = editor.clone();
        toolbar.italic_button().connect_clicked(move |_| {
            editor_clone.insert_around_selection("*", "*");
        });

        let editor_clone = editor.clone();
        toolbar.strike_button().connect_clicked(move |_| {
            editor_clone.insert_around_selection("~~", "~~");
        });

        let editor_clone = editor.clone();
        toolbar.h1_button().connect_clicked(move |_| {
            editor_clone.insert_at_line_start("# ");
        });

        let editor_clone = editor.clone();
        toolbar.h2_button().connect_clicked(move |_| {
            editor_clone.insert_at_line_start("## ");
        });

        let editor_clone = editor.clone();
        toolbar.h3_button().connect_clicked(move |_| {
            editor_clone.insert_at_line_start("### ");
        });

        let editor_clone = editor.clone();
        toolbar.ul_button().connect_clicked(move |_| {
            editor_clone.insert_at_line_start("- ");
        });

        let editor_clone = editor.clone();
        toolbar.ol_button().connect_clicked(move |_| {
            editor_clone.insert_at_line_start("1. ");
        });

        let editor_clone = editor.clone();
        toolbar.task_button().connect_clicked(move |_| {
            editor_clone.insert_at_line_start("- [ ] ");
        });

        let editor_clone = editor.clone();
        toolbar.link_button().connect_clicked(move |_| {
            editor_clone.insert_around_selection("[", "](url)");
        });

        let editor_clone = editor.clone();
        toolbar.image_button().connect_clicked(move |_| {
            editor_clone.insert_around_selection("![alt](", ")");
        });

        let editor_clone = editor.clone();
        toolbar.code_button().connect_clicked(move |_| {
            editor_clone.insert_around_selection("\n```\n", "\n```\n");
        });

        let editor_clone = editor.clone();
        toolbar.quote_button().connect_clicked(move |_| {
            editor_clone.insert_at_line_start("> ");
        });

        let editor_clone = editor.clone();
        toolbar.table_button().connect_clicked(move |_| {
            let table = "\n| Column 1 | Column 2 | Column 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |\n| Cell 4   | Cell 5   | Cell 6   |\n";
            editor_clone.insert_text(table);
        });

        let window_clone = window.clone();
        toolbar.fullscreen_button().connect_clicked(move |_| {
            if window_clone.is_fullscreen() {
                window_clone.unfullscreen();
            } else {
                window_clone.fullscreen();
            }
        });
    }

    fn handle_action(
        action: &str,
        window: &ApplicationWindow,
        editor: &EditorPane,
        file: Option<&gio::File>,
    ) {
        match action {
            "bold" => editor.insert_around_selection("**", "**"),
            "italic" => editor.insert_around_selection("*", "*"),
            "strikethrough" => editor.insert_around_selection("~~", "~~"),
            "link" => editor.insert_around_selection("[", "](url)"),
            "inline_code" => editor.insert_around_selection("`", "`"),
            "image" => editor.insert_around_selection("![alt](", ")"),
            "table" => {
                let table = "\n| Column 1 | Column 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |\n";
                editor.insert_text(table);
            }
            "save" => {
                if let Some(f) = file {
                    editor.save_file(f);
                }
            }
            "fullscreen" => {
                if window.is_fullscreen() {
                    window.unfullscreen();
                } else {
                    window.fullscreen();
                }
            }
            _ => {}
        }
    }
}
