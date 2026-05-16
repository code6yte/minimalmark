use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};

pub struct RenderResult {
    pub html: String,
    pub has_math: bool,
    pub has_mermaid: bool,
}

pub fn render_markdown(text: &str) -> RenderResult {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(text, options);
    
    let mut has_math = false;
    let mut has_mermaid = false;
    let mut processed_events: Vec<Event<'_>> = Vec::new();
    
    for event in parser {
        match &event {
            Event::Code(code) => {
                if code.starts_with("$$") && code.ends_with("$$") {
                    has_math = true;
                    let math_content = code.trim_start_matches("$$").trim_end_matches("$$");
                    processed_events.push(Event::Html(format!("<div class=\"katex-display\">{}</div>", math_content).into()));
                    continue;
                }
                if code.starts_with("$") && code.ends_with("$") {
                    has_math = true;
                    let math_content = code.trim_start_matches("$").trim_end_matches("$");
                    processed_events.push(Event::Html(format!("<span class=\"katex-inline\">{}</span>", math_content).into()));
                    continue;
                }
            }
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang))) => {
                if lang.to_string().contains("mermaid") {
                    has_mermaid = true;
                }
            }
            _ => {}
        }
        processed_events.push(event);
    }

    let mut html_output = String::with_capacity(text.len() * 2);
    html::push_html(&mut html_output, processed_events.into_iter());
    
    RenderResult {
        html: html_output,
        has_math,
        has_mermaid,
    }
}

pub fn extract_headings(text: &str) -> Vec<(usize, String)> {
    let mut headings = Vec::new();
    let parser = Parser::new_ext(text, Options::empty());
    
    for event in parser {
        if let Event::Start(Tag::Heading { level, .. }) = event {
            let level_num = match level {
                pulldown_cmark::HeadingLevel::H1 => 1,
                pulldown_cmark::HeadingLevel::H2 => 2,
                pulldown_cmark::HeadingLevel::H3 => 3,
                pulldown_cmark::HeadingLevel::H4 => 4,
                pulldown_cmark::HeadingLevel::H5 => 5,
                pulldown_cmark::HeadingLevel::H6 => 6,
            };
            
            if let Some(Event::Text(t)) = parser.clone().next() {
                headings.push((level_num, t.to_string()));
            }
        }
    }
    
    headings
}

pub fn count_stats(text: &str) -> (usize, usize, usize, usize, f64) {
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let sentences = text.split(|c| c == '.' || c == '!' || c == '?').count().saturating_sub(1);
    let paragraphs = text.split("\n\n").filter(|p| !p.trim().is_empty()).count();
    let reading_time = (words as f64 / 200.0).ceil();
    
    (words, chars, sentences, paragraphs, reading_time)
}
