use gtk::{EventControllerKey, glib};

pub fn setup_shortcuts(
    event_controller: &EventControllerKey,
    callbacks: impl Fn(&str) + Clone + 'static,
) {
    let callbacks_clone = callbacks.clone();
    event_controller.connect_key_pressed(move |_controller, key, _keycode, state| {
        let ctrl = state.contains(gtk::gdk::ModifierType::CONTROL_MASK);
        let shift = state.contains(gtk::gdk::ModifierType::SHIFT_MASK);

        let action = match (ctrl, shift, key) {
            (true, false, gtk::gdk::Key::b) => Some("bold"),
            (true, false, gtk::gdk::Key::i) => Some("italic"),
            (true, false, gtk::gdk::Key::k) => Some("strikethrough"),
            (true, false, gtk::gdk::Key::l) => Some("link"),
            (true, true, gtk::gdk::Key::k) => Some("inline_code"),
            (true, true, gtk::gdk::Key::i) => Some("image"),
            (true, true, gtk::gdk::Key::t) => Some("table"),
            (true, false, gtk::gdk::Key::s) => Some("save"),
            (true, true, gtk::gdk::Key::s) => Some("save_as"),
            (true, false, gtk::gdk::Key::o) => Some("open"),
            (true, false, gtk::gdk::Key::n) => Some("new"),
            (true, false, gtk::gdk::Key::f) => Some("find"),
            (true, false, gtk::gdk::Key::F11) => Some("fullscreen"),
            (false, false, gtk::gdk::Key::F11) => Some("fullscreen"),
            _ => None,
        };

        if let Some(action) = action {
            callbacks_clone(action);
            return glib::Propagation::Stop;
        }

        glib::Propagation::Proceed
    });
}
