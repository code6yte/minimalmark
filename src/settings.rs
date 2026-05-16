use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: ThemeMode,
    pub editor_font: String,
    pub editor_font_size: u32,
    pub preview_width_percent: f64,
    pub auto_save: bool,
    pub auto_save_interval: u64,
    pub spell_check: bool,
    pub word_wrap: bool,
    pub show_line_numbers: bool,
    pub focus_mode: bool,
    pub sync_scrolling: bool,
    pub recent_files: Vec<String>,
    pub window_width: i32,
    pub window_height: i32,
    pub is_maximized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::System,
            editor_font: "SF Mono, JetBrains Mono, Fira Code, monospace".to_string(),
            editor_font_size: 14,
            preview_width_percent: 50.0,
            auto_save: true,
            auto_save_interval: 30,
            spell_check: true,
            word_wrap: true,
            show_line_numbers: false,
            focus_mode: false,
            sync_scrolling: true,
            recent_files: Vec::new(),
            window_width: 1280,
            window_height: 800,
            is_maximized: false,
        }
    }
}

impl AppSettings {
    pub fn config_path() -> PathBuf {
        let mut path = glib::user_config_dir();
        path.push("minimalmark");
        path.push("settings.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(&path, json);
        }
    }

    pub fn add_recent_file(&mut self, path: &str) {
        self.recent_files.retain(|p| p != path);
        self.recent_files.insert(0, path.to_string());
        if self.recent_files.len() > 10 {
            self.recent_files.truncate(10);
        }
    }
}
