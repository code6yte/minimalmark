# MinimalMark - Feature Checklist

## Design & UI/UX
- [x] macOS-inspired color palette (refined gradients, no transparency)
- [x] Rounded corners on all UI elements
- [x] Light theme with warm, premium feel
- [x] Dark theme with deep, rich colors
- [x] System theme detection (follows GNOME dark/light)
- [x] Clean, minimal headerbar
- [x] Proper spacing and typography
- [x] Smooth transitions and hover states
- [x] Custom CSS for preview with macOS-like styling
- [x] libadwaita integration for GNOME native feel

## Core Editing
- [x] Full CommonMark support
- [x] GitHub Flavored Markdown (tables, task lists, strikethrough)
- [x] Footnotes support
- [x] Syntax highlighting in editor
- [x] Auto-indentation
- [x] Line/word wrap
- [x] Monospace font for editor
- [x] Spell checking integration

## Preview
- [x] Split-pane live preview
- [x] Synchronized scrolling
- [x] Custom CSS for rendered markdown
- [x] Code block syntax highlighting in preview
- [x] Math formula rendering (KaTeX)
- [x] Mermaid diagram support
- [x] Table rendering
- [x] Task list checkboxes

## File Management
- [x] Open file dialog
- [x] Save / Save As
- [x] Auto-save
- [x] Recent files list
- [x] Command line file opening
- [x] File change detection
- [x] Crash recovery

## Formatting Toolbar
- [x] Bold, Italic, Strikethrough buttons
- [x] Heading levels (H1-H6)
- [x] Lists (ordered, unordered, task)
- [x] Link insertion
- [x] Image insertion
- [x] Code block insertion
- [x] Blockquote insertion
- [x] Table insertion

## Keyboard Shortcuts
- [x] Ctrl+B (Bold)
- [x] Ctrl+I (Italic)
- [x] Ctrl+K (Strikethrough)
- [x] Ctrl+L (Link)
- [x] Ctrl+Shift+K (Inline code)
- [x] Ctrl+Shift+I (Image)
- [x] Ctrl+Shift+T (Table)
- [x] Ctrl+S (Save)
- [x] Ctrl+Shift+S (Save As)
- [x] Ctrl+O (Open)
- [x] Ctrl+N (New)
- [x] Ctrl+F (Find)
- [x] Ctrl+Z (Undo)
- [x] Ctrl+Shift+Z (Redo)
- [x] F11 (Fullscreen)
- [x] Ctrl+? (Shortcuts help)

## Document Features
- [x] Live word count
- [x] Character count
- [x] Reading time estimate
- [x] Document outline (heading tree)
- [x] Focus mode (highlight current paragraph)

## Export
- [x] Export to HTML
- [x] Export to PDF (via print)
- [x] Copy as HTML
- [x] Copy rendered markdown

## Settings
- [x] Preferences window (libadwaita)
- [x] Theme selection (Light/Dark/System)
- [x] Font selection for editor
- [x] Font size adjustment
- [x] Preview width control
- [x] Auto-save toggle
- [x] Spell check toggle

## Performance
- [x] Fast startup
- [x] Low memory footprint
- [x] Incremental preview updates
- [x] Optimized for large documents

## GNOME Integration
- [x] GTK4 + libadwaita
- [x] Desktop file with MIME types
- [x] AppStream metadata
- [x] File portal support
- [x] Session state persistence
