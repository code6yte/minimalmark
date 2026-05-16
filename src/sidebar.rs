use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Label, ScrolledWindow, ListBox, ListBoxRow};

#[derive(Clone)]
pub struct Sidebar {
    container: GtkBox,
    outline_list: ListBox,
}

impl Sidebar {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .width_request(200)
            .build();
        container.add_css_class("sidebar");

        let header = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .margin_start(8)
            .margin_top(8)
            .margin_bottom(4)
            .margin_end(8)
            .build();

        let icon = Label::builder()
            .label("☰")
            .css_classes(vec!["outline-icon".to_string()])
            .build();
        header.append(&icon);

        let label = Label::builder()
            .label("Outline")
            .halign(gtk::Align::Start)
            .css_classes(vec!["outline-header".to_string()])
            .build();
        header.append(&label);

        container.append(&header);

        let outline_list = ListBox::new();
        outline_list.set_activate_on_single_click(true);
        outline_list.add_css_class("outline-list");

        let scroll = ScrolledWindow::builder()
            .vexpand(true)
            .build();
        scroll.set_child(Some(&outline_list));
        container.append(&scroll);

        let placeholder = Label::builder()
            .label("No headings")
            .css_classes(vec!["outline-placeholder".to_string()])
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .vexpand(true)
            .build();
        outline_list.append(&placeholder);

        Self { container, outline_list }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn update_outline(&self, text: &str) {
        let mut headings: Vec<(String, u32, u32)> = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if let Some(level) = trimmed.chars().take(6).position(|c| c != '#') {
                if level > 0 && level <= 6 && trimmed.len() > level + 1 && trimmed.as_bytes()[level] == b' ' {
                    let title = trimmed[level+1..].to_string();
                    headings.push((title, level as u32, i as u32));
                }
            }
        }

        // Clear existing items
        while let Some(child) = self.outline_list.first_child() {
            child.unparent();
        }

        if headings.is_empty() {
            let placeholder = Label::builder()
                .label("No headings")
                .css_classes(vec!["outline-placeholder".to_string()])
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .vexpand(true)
                .build();
            self.outline_list.append(&placeholder);
            return;
        }

        for (title, level, _line) in &headings {
            let row = ListBoxRow::new();
            let indent = if *level == 1 { "" } else { "  " };
            let prefix = if *level == 1 { "●" } else if *level == 2 { "○" } else { "–" };
            let label = Label::builder()
                .label(format!("{}{} {}", indent, prefix, title))
                .halign(gtk::Align::Start)
                .margin_start(4i32 + (level - 1) as i32 * 12)
                .margin_top(3)
                .margin_bottom(3)
                .css_classes(vec!["outline-item".to_string()])
                .build();
            row.set_child(Some(&label));
            self.outline_list.append(&row);
        }
    }
}
