use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Button, Separator, Popover, Label};
use crate::editor::EditorPane;

fn section_header(text: &str) -> Label {
    Label::builder()
        .label(text)
        .css_classes(vec!["ctx-section".to_string()])
        .halign(gtk::Align::Start)
        .margin_start(12)
        .margin_top(8)
        .margin_bottom(4)
        .build()
}

fn menu_btn<F>(label: &str, tooltip: &str, on_click: F) -> Button
where
    F: Fn() + 'static,
{
    let btn = Button::builder()
        .label(label)
        .tooltip_text(tooltip)
        .css_classes(vec!["ctx-button".to_string()])
        .halign(gtk::Align::Fill)
        .build();
    btn.connect_clicked(move |_| on_click());
    btn
}

pub fn build_context_menu(editor: &EditorPane) -> Popover {
    let popover = Popover::builder()
        .has_arrow(true)
        .width_request(200)
        .build();
    popover.add_css_class("ctx-menu");

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(1)
        .build();

    vbox.append(&section_header("Format"));

    {
        let e = editor.clone();
        vbox.append(&menu_btn("Bold  Ctrl+B", "Bold", move || e.insert_around_selection("**", "**")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Italic  Ctrl+I", "Italic", move || e.insert_around_selection("*", "*")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Strikethrough  Ctrl+Shift+X", "Strikethrough", move || e.insert_around_selection("~~", "~~")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Inline Code  Ctrl+Shift+C", "Inline Code", move || e.insert_around_selection("`", "`")));
    }

    vbox.append(&Separator::new(Orientation::Horizontal));

    vbox.append(&section_header("Headings"));

    {
        let e = editor.clone();
        vbox.append(&menu_btn("Heading 1  Ctrl+1", "Heading 1", move || e.insert_at_line_start("# ")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Heading 2  Ctrl+2", "Heading 2", move || e.insert_at_line_start("## ")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Heading 3  Ctrl+3", "Heading 3", move || e.insert_at_line_start("### ")));
    }

    vbox.append(&Separator::new(Orientation::Horizontal));

    vbox.append(&section_header("Lists"));

    {
        let e = editor.clone();
        vbox.append(&menu_btn("Bullet List  Ctrl+Shift+8", "Bullet List", move || e.insert_at_line_start("- ")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Numbered List  Ctrl+Shift+7", "Numbered List", move || e.insert_at_line_start("1. ")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Task List", "Task List", move || e.insert_at_line_start("- [ ] ")));
    }

    vbox.append(&Separator::new(Orientation::Horizontal));

    vbox.append(&section_header("Insert"));

    {
        let e = editor.clone();
        vbox.append(&menu_btn("Link  Ctrl+L", "Link", move || e.insert_around_selection("[", "](url)")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Image", "Image", move || e.insert_around_selection("![alt](", ")")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Blockquote", "Blockquote", move || e.insert_at_line_start("> ")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Table", "Table", move || e.insert_text("\n| Col 1 | Col 2 |\n|-------|-------|\n| Cell  | Cell  |\n")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Code Block", "Code Block", move || e.insert_around_selection("```\n", "\n```")));
    }
    {
        let e = editor.clone();
        vbox.append(&menu_btn("Horizontal Rule", "Horizontal Rule", move || e.insert_text("\n---\n")));
    }

    popover.set_child(Some(&vbox));
    popover
}
