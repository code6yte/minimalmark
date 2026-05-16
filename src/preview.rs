use gtk::prelude::*;
use gtk::Box as GtkBox;
use webkit2gtk::{WebView, SettingsExt};

const MARKDOWN_CSS: &str = r#"
<style>
:root {
    color-scheme: light dark;
    --bg-primary: #FAFAFA;
    --bg-secondary: #FFFFFF;
    --text-primary: #1D1D1F;
    --text-secondary: #6E6E73;
    --accent: #007AFF;
    --border: #D2D2D7;
    --code-bg: #F5F5F7;
    --blockquote-bg: #F5F5F7;
    --blockquote-border: #D2D2D7;
    --table-header: #F5F5F7;
    --table-border: #D2D2D7;
    --hr-color: #D2D2D7;
    --shadow: rgba(0, 0, 0, 0.04);
}

@media (prefers-color-scheme: dark) {
    :root {
        --bg-primary: #242426;
        --bg-secondary: #1C1C1E;
        --text-primary: #F5F5F7;
        --text-secondary: #98989D;
        --accent: #0A84FF;
        --border: #38383A;
        --code-bg: #2C2C2E;
        --blockquote-bg: #2C2C2E;
        --blockquote-border: #48484A;
        --table-header: #2C2C2E;
        --table-border: #38383A;
        --hr-color: #38383A;
        --shadow: rgba(0, 0, 0, 0.2);
    }
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    line-height: 1.7;
    color: var(--text-primary);
    background: var(--bg-primary);
    max-width: 780px;
    margin: 0 auto;
    padding: 32px 24px;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

h1, h2, h3, h4, h5, h6 {
    margin-top: 1.8em;
    margin-bottom: 0.6em;
    font-weight: 600;
    letter-spacing: -0.01em;
    line-height: 1.3;
}

h1 { 
    font-size: 2.2em; 
    border-bottom: 2px solid var(--border); 
    padding-bottom: 0.4em; 
    margin-top: 0;
}

h2 { 
    font-size: 1.6em; 
    border-bottom: 1px solid var(--border); 
    padding-bottom: 0.3em; 
}

h3 { font-size: 1.3em; }
h4 { font-size: 1.1em; }
h5 { font-size: 1em; }
h6 { font-size: 0.9em; color: var(--text-secondary); }

p { 
    margin: 1em 0; 
    font-size: 1em;
}

a {
    color: var(--accent);
    text-decoration: none;
    border-bottom: 1px solid transparent;
    transition: border-color 0.15s ease;
}

a:hover {
    border-bottom-color: var(--accent);
}

strong { font-weight: 600; }
em { font-style: italic; }

code {
    background: var(--code-bg);
    padding: 0.2em 0.45em;
    border-radius: 5px;
    font-family: "SF Mono", "JetBrains Mono", "Fira Code", "Cascadia Code", "Consolas", monospace;
    font-size: 0.88em;
    border: 1px solid var(--border);
}

pre {
    background: var(--code-bg);
    padding: 18px 20px;
    border-radius: 10px;
    overflow-x: auto;
    margin: 1.2em 0;
    border: 1px solid var(--border);
}

pre code {
    background: none;
    padding: 0;
    border: none;
    font-size: 0.9em;
    line-height: 1.6;
}

blockquote {
    border-left: 4px solid var(--blockquote-border);
    margin: 1.2em 0;
    padding: 0.8em 1.2em;
    color: var(--text-secondary);
    background: var(--blockquote-bg);
    border-radius: 0 8px 8px 0;
}

blockquote p:last-child {
    margin-bottom: 0;
}

ul, ol {
    padding-left: 1.8em;
    margin: 1em 0;
}

li {
    margin: 0.4em 0;
}

li > ul, li > ol {
    margin: 0.2em 0;
}

input[type="checkbox"] {
    margin-right: 0.5em;
    accent-color: var(--accent);
}

table {
    border-collapse: collapse;
    width: 100%;
    margin: 1.2em 0;
    border-radius: 10px;
    overflow: hidden;
    border: 1px solid var(--table-border);
}

th, td {
    border: 1px solid var(--table-border);
    padding: 10px 14px;
    text-align: left;
}

th {
    font-weight: 600;
    background: var(--table-header);
}

tr:nth-child(even) {
    background: var(--code-bg);
}

hr {
    border: none;
    height: 1px;
    background: var(--hr-color);
    margin: 2em 0;
}

img {
    max-width: 100%;
    height: auto;
    border-radius: 10px;
    margin: 1em 0;
    box-shadow: 0 2px 8px var(--shadow);
}

del {
    opacity: 0.7;
}

mark {
    background: rgba(255, 214, 0, 0.3);
    padding: 0.1em 0.3em;
    border-radius: 3px;
}

.footnotes {
    margin-top: 2em;
    padding-top: 1em;
    border-top: 1px solid var(--border);
    font-size: 0.9em;
    color: var(--text-secondary);
}

.footnotes ol {
    padding-left: 1.5em;
}

.task-list-item {
    list-style: none;
    margin-left: -1.5em;
}

.task-list-item input {
    margin-right: 0.6em;
}

.mermaid {
    background: var(--bg-secondary);
    padding: 16px;
    border-radius: 10px;
    margin: 1em 0;
    text-align: center;
}

.katex-display {
    margin: 1.2em 0;
    overflow-x: auto;
}
</style>
"#;

const KATEX_CDN: &str = r#"
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js"></script>
"#;

const MERMAID_CDN: &str = r#"
<script type="module">
import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
mermaid.initialize({ startOnLoad: true, theme: 'default' });
</script>
"#;

#[derive(Clone)]
pub struct PreviewPane {
    webview: WebView,
    container: GtkBox,
}

impl PreviewPane {
    pub fn new() -> Self {
        let webview = WebView::new();
        
        if let Some(settings) = webview.settings() {
            settings.set_enable_developer_extras(false);
            settings.set_javascript_can_access_clipboard(false);
        }

        let container = GtkBox::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        container.append(&webview);

        Self { webview, container }
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    pub fn webview(&self) -> &WebView {
        &self.webview
    }

    pub fn update(&self, html_body: &str, has_math: bool, has_mermaid: bool) {
        let mut head = MARKDOWN_CSS.to_string();
        
        if has_math {
            head.push_str(KATEX_CDN);
        }
        
        if has_mermaid {
            head.push_str(MERMAID_CDN);
        }

        let full_html = format!(
            "<!DOCTYPE html><html><head><meta charset='utf-8'>{}</head><body>{}</body></html>",
            head, html_body
        );
        self.webview.load_html(&full_html, None);
    }
}
