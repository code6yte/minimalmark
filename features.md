# Features — Van (Minimal Markdown Editor)

> A truly minimal, cross‑platform markdown editor built in Rust.  
> Crafted for deep focus, plain‑file longevity, and perfect harmony with AI agents.

---

## 🧘 Distraction‑Free Writing
- **Focus Mode** – dim everything except the current sentence or paragraph.
- **Typewriter Mode** – keep the active line vertically centered at all times.
- **Clean, Chromeless UI** – no toolbars, no status clutter; just you and your words.
- **Hemingway Mode** – disable backspace/delete to force forward‑drafting (first drafts without self‑editing).

---

## 📝 Core Editing — Obsidian‑Style Inline Rendering
- **Live Inline Rendering (Live Preview)** – markdown syntax hides the moment you type it, exactly like Obsidian’s Live Preview. Headings, bold, italics, and links render as formatted text while you stay in editing mode.
- **Instant Source View Toggle** – switch between the clean rendered view and raw markdown with a single keystroke (`Ctrl/Cmd + /`).
- **GitHub‑Flavored Markdown (GFM)** – tables, task lists, strikethrough, autolinks.
- **Syntax‑Highlighted Code Blocks** – hundreds of languages supported.
- **Advanced Diagrams** – native rendering of Mermaid charts and LaTeX math blocks.

---

## ⚡ Performance & Rust‑Native Footprint
- **Sub‑10 MB Install Size** – no Electron, no Chromium; a pure Rust + Tauri/GTK/Qt binary.
- **Instant Cold Start** – ready to write in under 1 second.
- **Smooth on Large Documents** – comfortable editing of 200k+ character files without lag.
- **Low Memory Footprint** – typically under 100 MB RAM, even with multiple tabs.

---

## 🖥️ True Cross‑Platform
- **macOS** – native feel, follows system appearance, Force Touch support.
- **Windows** – fluent design integration, native file dialogs.
- **Linux** – GTK4/Qt builds, delivered as Flatpak and AppImage for all major distributions.
- **Terminal Integration** – open files directly via `van mynote.md` from any CLI.

---

## 📂 Local‑First, Plain‑File Philosophy
- **Plain `.md` Files** – your notes are yours, forever; no proprietary databases.
- **Works with Any Folder** – point the editor at a directory of markdown files and start editing.
- **Git‑Friendly** – every save is a clean diff, perfect for version control.
- **Relative Image Paths** – images stay alongside your markdown, no broken links.

---

## 🗂️ File & Tab Management
- **Multi‑Tab Native Interface** – drag to reorder, tear off, and split panes.
- **Synchronised Scrolling** – linked panes for side‑by‑side editing and preview.
- **Outline Sidebar** – auto‑generated table of contents for quick navigation.

---

## ✍️ Margin Format — File‑Based Annotations
> An unobtrusive annotation system that lives directly in your `.md` files, never breaking portability.

- **Comment Prefix Syntax** – uses standard `[//]: # (Your note here)` inline comments.
- **Right‑Sidebar Cards** – annotations render as beautiful, Medium‑style margin cards on wide screens.
- **Floating Popups** – on narrow windows, annotations appear as discrete, anchored popups.
- **Anchor Persistence** – notes survive content edits; they stay attached to their parent paragraph.
- **Agent‑Readable** – AI tools can parse, write, and update these structured annotations without touching the main content.

---

## 🤖 AI Agent & LLM Ready
- **`AGENTS.md` Convention** – project‑wide context and instructions in a plain markdown file at the root.
- **MCP Server Mode** – optional Model Context Protocol server so Claude, Cursor, or other agents can directly read/write your workspace.
- **Machine‑Readable Output** – document stats, outlines, and annotations exposed as JSON for agents.
- **Agent‑Safe Comments** – all margin annotations use a machine‑parseable format, keeping human prose and agent metadata cleanly separated.

---

## 📄 Preview & Export
- **Instant Split‑Screen Preview** – live rendered output with synchronised scroll.
- **Export to Polished PDF** – beautiful typography out of the box.
- **Export to Clean HTML** – no inline styles, ready for the web.
- **Copy as Rich Text** – paste directly into Gmail, Word, or Google Docs with formatting intact.

---

## 🖼️ Media & Linking
- **Drag‑and‑Drop Images** – auto‑saves a copy to a relative `assets/` folder and inserts a correct link.
- **Wiki‑Style Linking** – type `[[` to quickly link other notes, creating a personal knowledge graph.
- **Automatic Link Title Resolution** – previews of internal links on hover.

---

## ⚙️ Settings & Preferences — Toggle Everything
A clean, minimal settings panel (toggle on/off) so every writer can shape their ideal environment.

| Preference | Default | Description |
|------------|---------|-------------|
| **Typewriter Mode** | Off | Keeps the active line vertically centered while you type. |
| **Focus Mode** | Off | Dims all but the current sentence/paragraph. |
| **Inline Rendering (Live Preview)** | On | Hides markdown syntax instantly (Obsidian‑style). |
| **Hemingway Mode** | Off | Disables backspace/delete to encourage forward drafting. |
| **Show Line Numbers** | Off | Toggles line numbers in the editor gutter. |
| **Spell Check** | On | Enables system‑level spell checking. |
| **Auto‑Pair Brackets & Quotes** | On | Automatically closes `[]`, `()`, `""`, etc. |
| **Default Theme** | System | Light, Dark, Sepia, or follow OS setting. |

All preferences are saved per workspace and sync‑friendly (stored as a simple JSON config file).

---

## ⌨️ Customizable Keyboard Shortcuts
Every formatting action gets a **default shortcut** – and every shortcut can be changed in settings.

| Action | Default Shortcut (Win/Linux) | Default Shortcut (macOS) |
|--------|------------------------------|---------------------------|
| **Bold** | `Ctrl + B` | `Cmd + B` |
| **Italic** | `Ctrl + I` | `Cmd + I` |
| **Strikethrough** | `Ctrl + Shift + X` | `Cmd + Shift + X` |
| **Heading 1** | `Ctrl + 1` | `Cmd + 1` |
| **Heading 2** | `Ctrl + 2` | `Cmd + 2` |
| **Heading 3** | `Ctrl + 3` | `Cmd + 3` |
| **Bulleted List** | `Ctrl + Shift + 8` | `Cmd + Shift + 8` |
| **Numbered List** | `Ctrl + Shift + 7` | `Cmd + Shift + 7` |
| **Blockquote** | `Ctrl + Shift + .` | `Cmd + Shift + .` |
| **Code Inline** | `Ctrl + Shift + C` | `Cmd + Shift + C` |
| **Toggle Source View** | `Ctrl + /` | `Cmd + /` |
| **Toggle Focus Mode** | `Ctrl + Shift + F` | `Cmd + Shift + F` |
| **Toggle Typewriter Mode** | `Ctrl + Shift + T` | `Cmd + Shift + T` |

> A built‑in **command palette** (`Ctrl/Cmd + K`) lets you search and execute any action without memorising shortcuts.

---

## 🎨 Themes & Customisation
- **Beautiful Light, Dark, & Sepia Themes** – easy on the eyes for long sessions.
- **Instant Theme Switching** – change themes on the fly via shortcut or command palette.
- **Custom CSS Snippets** – tweak the rendered output to your exact taste.
- **High‑Contrast & Accessibility Modes** – designed for inclusive use.

---

## 📊 Writing Stats & Tools
- **Live Word & Character Count** – subtle, always visible.
- **Reading Time Estimate** – updated in real time.
- **Readability Score** – optional Flesch‑Kincaid or similar metric for clarity.

---

*Built with Rust for speed, simplicity, and a lasting relationship with plain text.*
