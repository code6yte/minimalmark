mod app;
mod window;
mod editor;
mod preview;
mod markdown;
mod settings;
mod shortcuts;
mod sidebar;
mod statusbar;
mod theme;
mod ctxmenu;
mod settingsdialog;

use app::MinimalMarkApp;

fn main() -> glib::ExitCode {
    let app = MinimalMarkApp::new();
    app.run()
}
