use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(text, options);
    let mut html_output = String::with_capacity(text.len() * 2);
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn count_stats(text: &str) -> (usize, usize, usize, usize, f64) {
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let sentences = text.split(|c| c == '.' || c == '!' || c == '?').count().saturating_sub(1);
    let paragraphs = text.split("\n\n").filter(|p| !p.trim().is_empty()).count();
    let reading_time = (words as f64 / 200.0).ceil();
    
    (words, chars, sentences, paragraphs, reading_time)
}
