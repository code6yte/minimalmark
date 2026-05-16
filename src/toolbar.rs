use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Button};

#[derive(Clone)]
pub struct Toolbar {
    container: GtkBox,
    buttons: Vec<Button>,
}

impl Toolbar {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        container.add_css_class("toolbar");

        let mut buttons = Vec::new();

        let bold_btn = Self::create_button("format-text-bold-symbolic", "Bold (Ctrl+B)");
        container.append(&bold_btn);
        buttons.push(bold_btn.clone());

        let italic_btn = Self::create_button("format-text-italic-symbolic", "Italic (Ctrl+I)");
        container.append(&italic_btn);
        buttons.push(italic_btn.clone());

        let strike_btn = Self::create_button("format-text-strikethrough-symbolic", "Strikethrough (Ctrl+K)");
        container.append(&strike_btn);
        buttons.push(strike_btn.clone());

        container.append(&Self::create_separator());

        let h1_btn = Self::create_button_with_label("H1", "Heading 1");
        container.append(&h1_btn);
        buttons.push(h1_btn.clone());

        let h2_btn = Self::create_button_with_label("H2", "Heading 2");
        container.append(&h2_btn);
        buttons.push(h2_btn.clone());

        let h3_btn = Self::create_button_with_label("H3", "Heading 3");
        container.append(&h3_btn);
        buttons.push(h3_btn.clone());

        container.append(&Self::create_separator());

        let ul_btn = Self::create_button("format-indent-more-symbolic", "Bullet List");
        container.append(&ul_btn);
        buttons.push(ul_btn.clone());

        let ol_btn = Self::create_button("format-indent-less-symbolic", "Numbered List");
        container.append(&ol_btn);
        buttons.push(ol_btn.clone());

        let task_btn = Self::create_button_with_label("☐", "Task List");
        container.append(&task_btn);
        buttons.push(task_btn.clone());

        container.append(&Self::create_separator());

        let link_btn = Self::create_button("insert-link-symbolic", "Link (Ctrl+L)");
        container.append(&link_btn);
        buttons.push(link_btn.clone());

        let image_btn = Self::create_button("image-x-generic-symbolic", "Image");
        container.append(&image_btn);
        buttons.push(image_btn.clone());

        let code_btn = Self::create_button_with_label("<>", "Code Block");
        container.append(&code_btn);
        buttons.push(code_btn.clone());

        let quote_btn = Self::create_button("format-justify-left-symbolic", "Blockquote");
        container.append(&quote_btn);
        buttons.push(quote_btn.clone());

        let table_btn = Self::create_button_with_label("⊞", "Table");
        container.append(&table_btn);
        buttons.push(table_btn.clone());

        container.append(&Self::create_separator());

        let fullscreen_btn = Self::create_button("view-fullscreen-symbolic", "Fullscreen (F11)");
        container.append(&fullscreen_btn);
        buttons.push(fullscreen_btn.clone());

        Self { container, buttons }
    }

    fn create_button(icon_name: &str, tooltip: &str) -> Button {
        let button = Button::from_icon_name(icon_name);
        button.set_tooltip_text(Some(tooltip));
        button.set_focus_on_click(false);
        button
    }

    fn create_button_with_label(label: &str, tooltip: &str) -> Button {
        let button = Button::with_label(label);
        button.set_tooltip_text(Some(tooltip));
        button.set_focus_on_click(false);
        button
    }

    fn create_separator() -> GtkBox {
        let sep = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .build();
        sep.set_size_request(1, 24);
        sep
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn bold_button(&self) -> &Button {
        &self.buttons[0]
    }

    pub fn italic_button(&self) -> &Button {
        &self.buttons[1]
    }

    pub fn strike_button(&self) -> &Button {
        &self.buttons[2]
    }

    pub fn h1_button(&self) -> &Button {
        &self.buttons[3]
    }

    pub fn h2_button(&self) -> &Button {
        &self.buttons[4]
    }

    pub fn h3_button(&self) -> &Button {
        &self.buttons[5]
    }

    pub fn ul_button(&self) -> &Button {
        &self.buttons[6]
    }

    pub fn ol_button(&self) -> &Button {
        &self.buttons[7]
    }

    pub fn task_button(&self) -> &Button {
        &self.buttons[8]
    }

    pub fn link_button(&self) -> &Button {
        &self.buttons[9]
    }

    pub fn image_button(&self) -> &Button {
        &self.buttons[10]
    }

    pub fn code_button(&self) -> &Button {
        &self.buttons[11]
    }

    pub fn quote_button(&self) -> &Button {
        &self.buttons[12]
    }

    pub fn table_button(&self) -> &Button {
        &self.buttons[13]
    }

    pub fn fullscreen_button(&self) -> &Button {
        &self.buttons[14]
    }
}
