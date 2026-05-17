use adw::prelude::*;
use adw::{PreferencesWindow, PreferencesPage, PreferencesGroup, ActionRow, ComboRow, SpinRow};
use crate::settings::{AppSettings, ThemeMode};
use crate::editor::EditorPane;
use std::rc::Rc;
use std::cell::RefCell;

fn get_local_monospace_fonts() -> Vec<String> {
    let mut fonts = Vec::new();
    let ctx = fontconfig::FontConfig::new();
    if let Ok(pattern) = ctx.pattern_new() {
        let _ = pattern.add_string("spacing", "mono");
        let _ = pattern.add_string("scalable", "true");
        if let Ok(mut set) = ctx.list(pattern) {
            let count = set.len();
            for i in 0..count {
                if let Ok(pattern) = set.get(i) {
                    if let Ok(family) = pattern.get_string("family") {
                        if !family.is_empty() && !fonts.contains(&family) {
                            fonts.push(family);
                            if fonts.len() >= 50 {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    if fonts.is_empty() {
        fonts.push("monospace".to_string());
    }
    fonts
}

pub fn show_settings(
    parent: &impl IsA<gtk::Window>,
    settings: &Rc<RefCell<AppSettings>>,
    editor: &EditorPane,
) {
    let local_fonts = get_local_monospace_fonts();
    let font_names: Vec<&str> = local_fonts.iter().map(|s| s.as_str()).collect();

    let win = PreferencesWindow::builder()
        .title("Preferences")
        .transient_for(parent)
        .modal(true)
        .default_width(500)
        .default_height(600)
        .build();

    // Editor page
    let editor_page = PreferencesPage::builder()
        .title("Editor")
        .build();

    let editor_group = PreferencesGroup::builder()
        .title("Editing")
        .build();

    let auto_pair_row = ActionRow::builder()
        .title("Auto-Pair Brackets")
        .subtitle("Automatically close [], (), {}, \"\", ''")
        .build();
    let auto_pair_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().auto_pair)
        .build();
    auto_pair_row.add_suffix(&auto_pair_switch);
    auto_pair_row.set_activatable_widget(Some(&auto_pair_switch));
    {
        let settings = settings.clone();
        auto_pair_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().auto_pair = active;
            settings.borrow().save();
            false.into()
        });
    }
    editor_group.add(&auto_pair_row);

    let live_preview_row = ActionRow::builder()
        .title("Live Preview (Inline Rendering)")
        .subtitle("Render markdown formatting inline (Obsidian-style)")
        .build();
    let live_preview_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().live_preview)
        .build();
    live_preview_row.add_suffix(&live_preview_switch);
    {
        let settings = settings.clone();
        let editor = editor.clone();
        live_preview_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().live_preview = active;
            settings.borrow().save();
            editor.set_live_preview(active);
            false.into()
        });
    }
    editor_group.add(&live_preview_row);

    let line_numbers_row = ActionRow::builder()
        .title("Show Line Numbers")
        .subtitle("Display line numbers in the editor gutter")
        .build();
    let line_numbers_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().show_line_numbers)
        .build();
    line_numbers_row.add_suffix(&line_numbers_switch);
    {
        let settings = settings.clone();
        let editor = editor.clone();
        line_numbers_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().show_line_numbers = active;
            settings.borrow().save();
            editor.toggle_line_numbers(active);
            false.into()
        });
    }
    editor_group.add(&line_numbers_row);

    let word_wrap_row = ActionRow::builder()
        .title("Word Wrap")
        .subtitle("Wrap long lines to fit the window")
        .build();
    let word_wrap_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().word_wrap)
        .build();
    word_wrap_row.add_suffix(&word_wrap_switch);
    {
        let settings = settings.clone();
        let editor = editor.clone();
        word_wrap_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().word_wrap = active;
            settings.borrow().save();
            editor.toggle_word_wrap(active);
            false.into()
        });
    }
    editor_group.add(&word_wrap_row);

    editor_page.add(&editor_group);

    // Font settings group
    let font_group = PreferencesGroup::builder()
        .title("Font")
        .build();

    let font_row = ComboRow::builder()
        .title("Editor Font")
        .build();
    let font_model = gtk::StringList::new(&font_names);
    font_row.set_model(Some(&font_model));
    let current_font = settings.borrow().editor_font.clone();
    let mut font_idx = 0u32;
    for i in 0..font_model.n_items() {
        if let Some(font) = font_model.string(i) {
            if font.to_string() == current_font {
                font_idx = i;
                break;
            }
        }
    }
    font_row.set_selected(font_idx);
    {
        let settings = settings.clone();
        let editor = editor.clone();
        font_row.connect_selected_notify(move |row| {
            let idx = row.selected();
            if let Some(font) = font_model.string(idx) {
                let font_str = font.to_string();
                settings.borrow_mut().editor_font = font_str.clone();
                settings.borrow().save();
                let size = settings.borrow().editor_font_size;
                let line_spacing = settings.borrow().line_spacing;
                editor.set_font(&font_str, size);
                editor.set_line_spacing(line_spacing);
            }
        });
    }
    font_group.add(&font_row);

    let font_size_row = SpinRow::builder()
        .title("Font Size")
        .subtitle("Editor font size in pixels")
        .adjustment(&gtk::Adjustment::new(12.0, 8.0, 32.0, 1.0, 2.0, 0.0))
        .value(settings.borrow().editor_font_size as f64)
        .build();
    {
        let settings = settings.clone();
        let editor = editor.clone();
        font_size_row.connect_value_notify(move |row| {
            let size = row.value() as u32;
            settings.borrow_mut().editor_font_size = size;
            settings.borrow().save();
            let font = settings.borrow().editor_font.clone();
            editor.set_font(&font, size);
        });
    }
    font_group.add(&font_size_row);

    let line_spacing_row = SpinRow::builder()
        .title("Line Spacing")
        .subtitle("Vertical space between lines")
        .adjustment(&gtk::Adjustment::new(1.6, 1.0, 3.0, 0.1, 0.2, 0.0))
        .value(settings.borrow().line_spacing)
        .build();
    {
        let settings = settings.clone();
        let editor = editor.clone();
        line_spacing_row.connect_value_notify(move |row| {
            let spacing = row.value();
            settings.borrow_mut().line_spacing = spacing;
            settings.borrow().save();
            editor.set_line_spacing(spacing);
        });
    }
    font_group.add(&line_spacing_row);

    editor_page.add(&font_group);

    // View page
    let view_page = PreferencesPage::builder()
        .title("View")
        .build();

    let view_group = PreferencesGroup::builder()
        .title("Display")
        .build();

    let typewriter_row = ActionRow::builder()
        .title("Typewriter Mode")
        .subtitle("Keep the active line vertically centered")
        .build();
    let typewriter_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().typewriter_mode)
        .build();
    typewriter_row.add_suffix(&typewriter_switch);
    {
        let settings = settings.clone();
        typewriter_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().typewriter_mode = active;
            settings.borrow().save();
            false.into()
        });
    }
    view_group.add(&typewriter_row);

    let focus_row = ActionRow::builder()
        .title("Focus Mode")
        .subtitle("Dim all but the current paragraph")
        .build();
    let focus_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().focus_mode)
        .build();
    focus_row.add_suffix(&focus_switch);
    {
        let settings = settings.clone();
        focus_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().focus_mode = active;
            settings.borrow().save();
            false.into()
        });
    }
    view_group.add(&focus_row);

    let hemingway_row = ActionRow::builder()
        .title("Hemingway Mode")
        .subtitle("Disable backspace/delete to force forward drafting")
        .build();
    let hemingway_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().hemingway_mode)
        .build();
    hemingway_row.add_suffix(&hemingway_switch);
    {
        let settings = settings.clone();
        hemingway_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().hemingway_mode = active;
            settings.borrow().save();
            false.into()
        });
    }
    view_group.add(&hemingway_row);

    view_page.add(&view_group);

    let theme_group = PreferencesGroup::builder()
        .title("Theme")
        .build();

    let theme_row = ComboRow::builder()
        .title("Theme")
        .subtitle("Choose your preferred theme")
        .build();
    let theme_model = gtk::StringList::new(&["System", "Light", "Dark"]);
    theme_row.set_model(Some(&theme_model));
    let current = match settings.borrow().theme {
        ThemeMode::System => 0,
        ThemeMode::Light => 1,
        ThemeMode::Dark => 2,
    };
    theme_row.set_selected(current as u32);
    {
        let settings = settings.clone();
        theme_row.connect_selected_notify(move |row| {
            let idx = row.selected();
            let theme = match idx {
                1 => ThemeMode::Light,
                2 => ThemeMode::Dark,
                _ => ThemeMode::System,
            };
            settings.borrow_mut().theme = theme.clone();
            settings.borrow().save();
            let style_manager = adw::StyleManager::default();
            match theme {
                ThemeMode::Light => {
                    style_manager.set_color_scheme(adw::ColorScheme::ForceLight);
                }
                ThemeMode::Dark => {
                    style_manager.set_color_scheme(adw::ColorScheme::ForceDark);
                }
                ThemeMode::System => {
                    style_manager.set_color_scheme(adw::ColorScheme::Default);
                }
            }
        });
    }
    theme_group.add(&theme_row);

    view_page.add(&theme_group);

    // Writing stats page
    let stats_page = PreferencesPage::builder()
        .title("Writing")
        .build();

    let stats_group = PreferencesGroup::builder()
        .title("Writing Goals")
        .build();

    let goals_label = gtk::Label::builder()
        .label("Daily word count and writing goals coming soon.")
        .margin_top(16)
        .margin_bottom(16)
        .css_classes(vec!["dim-label".to_string()])
        .build();
    stats_group.add(&goals_label);
    stats_page.add(&stats_group);

    let auto_save_group = PreferencesGroup::builder()
        .title("Auto-Save")
        .build();

    let auto_save_row = ActionRow::builder()
        .title("Auto-Save")
        .subtitle("Automatically save changes")
        .build();
    let auto_save_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().auto_save)
        .build();
    auto_save_row.add_suffix(&auto_save_switch);
    {
        let settings = settings.clone();
        auto_save_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().auto_save = active;
            settings.borrow().save();
            false.into()
        });
    }
    auto_save_group.add(&auto_save_row);
    stats_page.add(&auto_save_group);

    win.add(&editor_page);
    win.add(&view_page);
    win.add(&stats_page);
    win.present();
}
