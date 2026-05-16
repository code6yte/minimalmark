use adw::prelude::*;
use adw::{PreferencesWindow, PreferencesPage, PreferencesGroup, ActionRow, ComboRow};
use crate::settings::{AppSettings, ThemeMode};
use std::rc::Rc;
use std::cell::RefCell;

pub fn show_settings(parent: &impl IsA<gtk::Window>, settings: &Rc<RefCell<AppSettings>>) {
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
        live_preview_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().live_preview = active;
            settings.borrow().save();
            false.into()
        });
    }
    editor_group.add(&live_preview_row);

    let spell_check_row = ActionRow::builder()
        .title("Spell Check")
        .subtitle("Enable system spell checking")
        .build();
    let spell_check_switch = gtk::Switch::builder()
        .valign(gtk::Align::Center)
        .active(settings.borrow().spell_check)
        .build();
    spell_check_row.add_suffix(&spell_check_switch);
    {
        let settings = settings.clone();
        spell_check_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().spell_check = active;
            settings.borrow().save();
            false.into()
        });
    }
    editor_group.add(&spell_check_row);

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
        line_numbers_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().show_line_numbers = active;
            settings.borrow().save();
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
        word_wrap_switch.connect_state_set(move |_, active| {
            settings.borrow_mut().word_wrap = active;
            settings.borrow().save();
            false.into()
        });
    }
    editor_group.add(&word_wrap_row);

    editor_page.add(&editor_group);

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
    let theme_model = gtk::StringList::new(&["System", "Light", "Dark", "Sepia"]);
    theme_row.set_model(Some(&theme_model));
    let current = match settings.borrow().theme {
        ThemeMode::System => 0,
        ThemeMode::Light => 1,
        ThemeMode::Dark => 2,
        ThemeMode::Sepia => 3,
    };
    theme_row.set_selected(current as u32);
    {
        let settings = settings.clone();
        theme_row.connect_selected_notify(move |row| {
            let idx = row.selected();
            let theme = match idx {
                1 => ThemeMode::Light,
                2 => ThemeMode::Dark,
                3 => ThemeMode::Sepia,
                _ => ThemeMode::System,
            };
            settings.borrow_mut().theme = theme;
            settings.borrow().save();
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
