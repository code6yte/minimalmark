# MinimalMark

A minimal, fast Markdown editor for GNOME desktop with macOS-inspired design.

![MinimalMark](data/icons/hicolor/scalable/apps/io.github.minimalmark.svg)

## Features

### Design
- macOS-inspired color palette with refined gradients
- Rounded corners on all UI elements
- Light and dark themes with system detection
- Clean, minimal interface with proper spacing
- Smooth transitions and hover states

### Editing
- Full CommonMark + GitHub Flavored Markdown support
- Tables, task lists, strikethrough, footnotes
- Syntax highlighting with gtksourceview5
- Auto-indentation and word wrap
- Spell checking integration
- Monospace font with customizable size

### Preview
- Split-pane live preview
- Custom CSS for rendered markdown
- KaTeX math formula rendering
- Mermaid diagram support
- Code block syntax highlighting

### Toolbar
- Bold, Italic, Strikethrough
- Heading levels (H1-H3)
- Lists (ordered, unordered, task)
- Link, Image, Code block, Blockquote, Table insertion
- Fullscreen toggle

### Keyboard Shortcuts
- `Ctrl+B` - Bold
- `Ctrl+I` - Italic
- `Ctrl+K` - Strikethrough
- `Ctrl+L` - Link
- `Ctrl+Shift+K` - Inline code
- `Ctrl+Shift+I` - Image
- `Ctrl+Shift+T` - Table
- `Ctrl+S` - Save
- `Ctrl+O` - Open
- `F11` - Fullscreen

### Document Stats
- Live word count
- Character count
- Reading time estimate
- Cursor position (line, column)

### File Management
- Auto-save
- Recent files list
- Command line file opening
- File change detection

### Export
- Export to HTML
- Export to PDF (via print)
- Copy as HTML

## Build Dependencies (Fedora)

```bash
sudo dnf install -y rust cargo gtk4-devel glib2-devel webkit2gtk4.1-devel libadwaita-devel gtksourceview5-devel
```

## Build

```bash
cargo build --release
```

## Install

```bash
sudo ./packaging/fedora/build.sh
```

Or manually:

```bash
sudo install -Dm755 target/release/minimalmark /usr/local/bin/minimalmark
sudo install -Dm644 data/io.github.minimalmark.desktop /usr/share/applications/io.github.minimalmark.desktop
sudo install -Dm644 data/io.github.minimalmark.metainfo.xml /usr/share/metainfo/io.github.minimalmark.metainfo.xml
sudo install -Dm644 data/style.css /usr/share/minimalmark/style.css
sudo install -Dm644 data/icons/hicolor/scalable/apps/io.github.minimalmark.svg /usr/share/icons/hicolor/scalable/apps/io.github.minimalmark.svg
```

## Run

```bash
minimalmark
```

Or open a markdown file directly:

```bash
minimalmark document.md
```

## Settings

Settings are stored in `~/.config/minimalmark/settings.json`:

```json
{
  "theme": "System",
  "editor_font": "SF Mono, JetBrains Mono, Fira Code, monospace",
  "editor_font_size": 14,
  "preview_width_percent": 50.0,
  "auto_save": true,
  "auto_save_interval": 30,
  "spell_check": true,
  "word_wrap": true,
  "show_line_numbers": false,
  "focus_mode": false,
  "sync_scrolling": true
}
```

## License

MIT
