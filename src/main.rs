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

use app::MinimalMarkApp;

fn main() -> glib::ExitCode {
    let app = MinimalMarkApp::new();
    app.run()
}
