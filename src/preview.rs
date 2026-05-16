use gtk::prelude::*;
use gtk::{TextBuffer, ScrolledWindow, TextView};
use pulldown_cmark::{Parser, Options, Event, Tag, TagEnd, HeadingLevel};

#[derive(Clone)]
pub struct PreviewPane {
    buffer: TextBuffer,
    view: TextView,
    scroll: ScrolledWindow,
    bold_tag: gtk::TextTag,
    italic_tag: gtk::TextTag,
    strike_tag: gtk::TextTag,
    code_tag: gtk::TextTag,
    h1_tag: gtk::TextTag,
    h2_tag: gtk::TextTag,
    h3_tag: gtk::TextTag,
    quote_tag: gtk::TextTag,
    link_tag: gtk::TextTag,
}

impl PreviewPane {
    pub fn new() -> Self {
        let buffer = TextBuffer::new(None);

        let bold_tag = gtk::TextTag::builder()
            .name("pv-bold")
            .weight(700)
            .build();
        buffer.tag_table().add(&bold_tag);

        let italic_tag = gtk::TextTag::builder()
            .name("pv-italic")
            .style(gtk::pango::Style::Italic)
            .build();
        buffer.tag_table().add(&italic_tag);

        let strike_tag = gtk::TextTag::builder()
            .name("pv-strike")
            .strikethrough(true)
            .build();
        buffer.tag_table().add(&strike_tag);

        let code_tag = gtk::TextTag::builder()
            .name("pv-code")
            .family("monospace")
            .scale(0.9)
            .build();
        buffer.tag_table().add(&code_tag);

        let h1_tag = gtk::TextTag::builder()
            .name("pv-h1")
            .scale(2.0)
            .weight(700)
            .build();
        buffer.tag_table().add(&h1_tag);

        let h2_tag = gtk::TextTag::builder()
            .name("pv-h2")
            .scale(1.6)
            .weight(700)
            .build();
        buffer.tag_table().add(&h2_tag);

        let h3_tag = gtk::TextTag::builder()
            .name("pv-h3")
            .scale(1.3)
            .weight(700)
            .build();
        buffer.tag_table().add(&h3_tag);

        let quote_tag = gtk::TextTag::builder()
            .name("pv-quote")
            .scale(0.95)
            .foreground_rgba(&gtk::gdk::RGBA::new(0.43, 0.43, 0.45, 1.0))
            .style(gtk::pango::Style::Italic)
            .build();
        buffer.tag_table().add(&quote_tag);

        let link_tag = gtk::TextTag::builder()
            .name("pv-link")
            .foreground_rgba(&gtk::gdk::RGBA::new(0.0, 0.48, 1.0, 1.0))
            .underline(gtk::pango::Underline::Single)
            .build();
        buffer.tag_table().add(&link_tag);

        let view = TextView::builder()
            .buffer(&buffer)
            .editable(false)
            .cursor_visible(false)
            .wrap_mode(gtk::WrapMode::Word)
            .left_margin(24)
            .right_margin(24)
            .top_margin(24)
            .bottom_margin(24)
            .build();

        let scroll = ScrolledWindow::new();
        scroll.set_hexpand(true);
        scroll.set_vexpand(true);
        scroll.set_child(Some(&view));

        Self { buffer, view, scroll, bold_tag, italic_tag, strike_tag, code_tag, h1_tag, h2_tag, h3_tag, quote_tag, link_tag }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.scroll
    }

    pub fn update(&self, text: &str) {
        self.buffer.set_text("");

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_FOOTNOTES);

        let parser = Parser::new_ext(text, options);
        let mut tag_stack: Vec<(String, u32)> = Vec::new();
        let mut in_paragraph = false;

        for event in parser {
            match event {
                Event::Start(tag) => {
                    let tag_info = match &tag {
                        Tag::Paragraph => { in_paragraph = true; continue; }
                        Tag::Heading { level, .. } => {
                            self.buffer.insert(&mut self.buffer.end_iter(), "\n");
                            let size = match level {
                                HeadingLevel::H1 => "pv-h1",
                                HeadingLevel::H2 => "pv-h2",
                                HeadingLevel::H3 => "pv-h3",
                                _ => "pv-h3",
                            };
                            (size.to_string(), 1)
                        }
                        Tag::Strong => ("pv-bold".to_string(), 2),
                        Tag::Emphasis => ("pv-italic".to_string(), 2),
                        Tag::Strikethrough => ("pv-strike".to_string(), 2),
                        Tag::CodeBlock(_) => {
                            self.buffer.insert(&mut self.buffer.end_iter(), "\n");
                            ("pv-code".to_string(), 1)
                        }
                        Tag::List(_) => { continue; }
                        Tag::Item => { self.buffer.insert(&mut self.buffer.end_iter(), "  • "); continue; }
                        Tag::BlockQuote(_kind) => {
                            self.buffer.insert(&mut self.buffer.end_iter(), "\n");
                            ("pv-quote".to_string(), 1)
                        }
                        Tag::Link { .. } => ("pv-link".to_string(), 2),
                        _ => continue,
                    };
                    tag_stack.push(tag_info);
                }
                Event::End(tag) => {
                    match &tag {
                        TagEnd::Paragraph => {
                            if in_paragraph {
                                self.buffer.insert(&mut self.buffer.end_iter(), "\n\n");
                                in_paragraph = false;
                            }
                        }
                        TagEnd::Heading(..) => {
                            self.buffer.insert(&mut self.buffer.end_iter(), "\n\n");
                            if !tag_stack.is_empty() && (tag_stack.last().unwrap().0 == "pv-h1" || tag_stack.last().unwrap().0 == "pv-h2" || tag_stack.last().unwrap().0 == "pv-h3") {
                                tag_stack.pop();
                            }
                        }
                        TagEnd::CodeBlock => {
                            self.buffer.insert(&mut self.buffer.end_iter(), "\n");
                            if !tag_stack.is_empty() && tag_stack.last().unwrap().0 == "pv-code" {
                                tag_stack.pop();
                            }
                        }
                        TagEnd::BlockQuote(_) => {
                            self.buffer.insert(&mut self.buffer.end_iter(), "\n");
                            if !tag_stack.is_empty() && tag_stack.last().unwrap().0 == "pv-quote" {
                                tag_stack.pop();
                            }
                        }
                        TagEnd::List(_) => {
                            self.buffer.insert(&mut self.buffer.end_iter(), "\n");
                        }
                        TagEnd::Item => {}
                        TagEnd::Strong | TagEnd::Emphasis | TagEnd::Strikethrough => {
                            if !tag_stack.is_empty() {
                                let last = tag_stack.last().unwrap();
                                let name = match &tag {
                                    TagEnd::Strong => "pv-bold",
                                    TagEnd::Emphasis => "pv-italic",
                                    TagEnd::Strikethrough => "pv-strike",
                                    _ => "",
                                };
                                if last.0 == name {
                                    tag_stack.pop();
                                }
                            }
                        }
                        TagEnd::Link => {
                            if !tag_stack.is_empty() && tag_stack.last().unwrap().0 == "pv-link" {
                                tag_stack.pop();
                            }
                        }
                        _ => {}
                    }
                }
                Event::Text(t) => {
                    let start_offset = self.buffer.end_iter().offset();
                    self.buffer.insert(&mut self.buffer.end_iter(), &t);
                    let start = self.buffer.iter_at_offset(start_offset);
                    let end = self.buffer.end_iter();
                    for (tag_name, _) in &tag_stack {
                        if let Some(tag) = self.buffer.tag_table().lookup(tag_name) {
                            self.buffer.apply_tag(&tag, &start, &end);
                        }
                    }
                }
                Event::Code(t) => {
                    let start_offset = self.buffer.end_iter().offset();
                    self.buffer.insert(&mut self.buffer.end_iter(), &t);
                    let start = self.buffer.iter_at_offset(start_offset);
                    let end = self.buffer.end_iter();
                    self.buffer.apply_tag(&self.code_tag, &start, &end);
                }
                Event::HardBreak | Event::SoftBreak => {
                    self.buffer.insert(&mut self.buffer.end_iter(), "\n");
                }
                Event::Rule => {
                    self.buffer.insert(&mut self.buffer.end_iter(), "\n─────────────────────\n");
                }
                Event::FootnoteReference(_) | Event::TaskListMarker(..) => {}
                Event::Html(_) | Event::InlineHtml(_) => {
                    // HTML is stripped
                }
                Event::InlineMath(_) | Event::DisplayMath(_) => {}
            }
        }
    }
}
