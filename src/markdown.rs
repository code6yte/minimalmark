pub fn count_stats(text: &str) -> (usize, usize, usize, usize, f64) {
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let sentences = text.split(|c| c == '.' || c == '!' || c == '?').count().saturating_sub(1);
    let paragraphs = text.split("\n\n").filter(|p| !p.trim().is_empty()).count();
    let reading_time = (words as f64 / 200.0).ceil();

    (words, chars, sentences, paragraphs, reading_time)
}
