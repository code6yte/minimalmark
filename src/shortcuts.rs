use gtk::{EventControllerKey, glib};

pub fn setup_shortcuts(
    event_controller: &EventControllerKey,
    callbacks: impl Fn(&str) + Clone + 'static,
) {
    let callbacks_clone = callbacks.clone();
    event_controller.connect_key_pressed(move |_controller, key, _keycode, state| {
        let ctrl = state.contains(gtk::gdk::ModifierType::CONTROL_MASK);
        let shift = state.contains(gtk::gdk::ModifierType::SHIFT_MASK);
        let alt = state.contains(gtk::gdk::ModifierType::ALT_MASK);

        let action = match (ctrl, shift, alt, key) {
            // Formatting
            (true, false, false, gtk::gdk::Key::b) => Some("bold"),
            (true, false, false, gtk::gdk::Key::i) => Some("italic"),
            (true, true, false, gtk::gdk::Key::x) => Some("strikethrough"),
            (true, false, false, gtk::gdk::Key::l) => Some("link"),
            (true, true, false, gtk::gdk::Key::c) => Some("inline_code"),
            (true, true, false, gtk::gdk::Key::i) => Some("image"),
            (true, true, false, gtk::gdk::Key::t) => Some("table"),

            // Headings
            (true, false, false, gtk::gdk::Key::_1) => Some("heading1"),
            (true, false, false, gtk::gdk::Key::_2) => Some("heading2"),
            (true, false, false, gtk::gdk::Key::_3) => Some("heading3"),

            // Lists
            (true, true, false, gtk::gdk::Key::_8) => Some("bullet_list"),
            (true, true, false, gtk::gdk::Key::_7) => Some("numbered_list"),
            (true, true, false, gtk::gdk::Key::period) => Some("blockquote"),

            // View modes
            (true, false, false, gtk::gdk::Key::slash) => Some("toggle_source"),
            (true, true, false, gtk::gdk::Key::f) => Some("focus_mode"),
            (true, true, false, gtk::gdk::Key::t) => Some("typewriter_mode"),
            (false, false, false, gtk::gdk::Key::F11) => Some("fullscreen"),

            // File operations
            (true, false, false, gtk::gdk::Key::s) => Some("save"),
            (true, true, false, gtk::gdk::Key::s) => Some("save_as"),
            (true, false, false, gtk::gdk::Key::o) => Some("open"),
            (true, false, false, gtk::gdk::Key::n) => Some("new"),
            (true, false, false, gtk::gdk::Key::f) => Some("find"),

            // Commands
            (true, false, false, gtk::gdk::Key::k) => Some("command_palette"),

            // Settings
            (true, false, false, gtk::gdk::Key::comma) => Some("settings"),

            _ => None,
        };

        if let Some(action) = action {
            callbacks_clone(action);
            return glib::Propagation::Stop;
        }

        glib::Propagation::Proceed
    });
}
