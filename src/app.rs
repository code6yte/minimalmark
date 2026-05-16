use gtk::prelude::*;
use gtk::Application;
use adw::{ApplicationWindow, StyleManager, prelude::AdwApplicationWindowExt};
use crate::window::MainWindow;
use crate::settings::AppSettings;

const APP_ID: &str = "io.github.minimalmark";

pub struct MinimalMarkApp {
    app: Application,
}

impl MinimalMarkApp {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id(APP_ID)
            .flags(gio::ApplicationFlags::HANDLES_OPEN)
            .build();

        Self { app }
    }

    pub fn run(&self) -> glib::ExitCode {
        let app = self.app.clone();

        app.connect_startup(|app| {
            Self::setup_css(app);
        });

        app.connect_activate(|app| {
            Self::build_ui(app);
        });

        app.connect_open(|app, files, _hint| {
            if let Some(file) = files.first() {
                Self::build_ui_with_file(app, file);
            } else {
                Self::build_ui(app);
            }
        });

        app.run()
    }

    fn setup_css(_app: &Application) {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("../data/style.css"));
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn apply_theme_from_settings() {
        use crate::settings::ThemeMode;
        let settings = AppSettings::load();
        let style_manager = StyleManager::default();
        match settings.theme {
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
    }

    fn build_ui(app: &Application) {
        Self::apply_theme_from_settings();
        let style_manager = StyleManager::default();
        let is_dark = style_manager.is_dark();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("MinimalMark")
            .default_width(1280)
            .default_height(800)
            .build();

        let main_window = MainWindow::new(&window, None, is_dark);
        window.set_content(Some(main_window.container().upcast_ref::<gtk::Widget>()));
        window.present();
    }

    fn build_ui_with_file(app: &Application, file: &gio::File) {
        Self::apply_theme_from_settings();
        let style_manager = StyleManager::default();
        let is_dark = style_manager.is_dark();

        let title = file.basename()
            .map(|p| format!("MinimalMark - {}", p.to_string_lossy()))
            .unwrap_or_else(|| "MinimalMark".into());

        let window = ApplicationWindow::builder()
            .application(app)
            .title(&title)
            .default_width(1280)
            .default_height(800)
            .build();

        let main_window = MainWindow::new(&window, Some(file), is_dark);
        window.set_content(Some(main_window.container().upcast_ref::<gtk::Widget>()));
        window.present();
    }
}
