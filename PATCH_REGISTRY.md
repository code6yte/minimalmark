# Patch Registry — MinimalMark

Check this before any push to avoid re-introducing old bugs.

## 1. Compilation Errors (pulldown-cmark 0.12 / gtk4-rs 0.9 API)

| Error | File | Cause | Fix |
|---|---|---|---|
| `EventControllerRightClick` not found | `editor.rs` | Removed in gtk4-rs 0.9 | `GestureClick::new()` + `set_button(3)` |
| `add_tag` missing (1-arg call) | `editor.rs`, `preview.rs` | `add_tag` requires 3 args (tag, start, end) | `buffer.tag_table().add(&t)` to register, `buffer.apply_tag(&t, &s, &e)` to apply |
| `Tag::Bold` / `Tag::Italic` undefined | `preview.rs` | pulldown-cmark 0.12 renamed | `Tag::Strong` / `Tag::Emphasis` |
| `Tag::BlockQuote` unit variant | `preview.rs` | 0.12 changed to tuple | `Tag::BlockQuote(_kind)` |
| `TagEnd::Bold` / `TagEnd::Italic` undefined | `preview.rs` | Same pulldown rename | `TagEnd::Strong` / `TagEnd::Emphasis` |
| `TagEnd::BlockQuote` unit variant | `preview.rs` | Same change as above | `TagEnd::BlockQuote(_)` |
| `Event::InlineHtml` not covered | `preview.rs` | 0.12 still has this variant | Keep `Event::Html(_) \| Event::InlineHtml(_) => {}` |
| `.weight(Weight::Bold)` expects `i32` | `editor.rs`, `preview.rs` | `TextTagBuilder::weight()` takes `i32` not `pango::Weight` | `.weight(700)` |
| `.activatable_widget("str")` takes Widget | `settingsdialog.rs` | Builder method expects `&impl IsA<Widget>` | Remove from builder; use `set_activatable_widget()` |
| `.margin_start(u32)` expects `i32` | `sidebar.rs` | Builder expects `i32` | Cast: `(level - 1) as i32 * 12` |
| `mark.name()` returns `Option<GString>` | `window.rs` | GString != `&str` | `.name().as_deref() == Some("insert")` |
| `set_title(&String)` expects `Option<&str>` | `window.rs` | Signature mismatch | `set_title(Some(&format!(...)))` |
| Unused import `self` | `window.rs` | `use crate::ctxmenu::{self, ...}` | Just `use crate::ctxmenu::...` |
| Unused params in handler | `window.rs` | Stubs not using all args | Prefix with `_` |

## 2. Runtime Crashes

| Symptom | Root Cause | File | Fix |
|---|---|---|---|
| Segfault on right-click | `Popover.popup()` called with no parent widget | `editor.rs` | `popover.set_parent(self.view)` before gesture |
| Segfault on typing/Enter | `remove_live_preview_tags()` reused same iterators across `remove_tag` calls | `editor.rs` | Create fresh `start_iter()`/`end_iter()` per tag in loop |
| `apply_tag` assertion on empty text | `apply_inline_preview()` returned early w/o resetting `is_applying` | `editor.rs` | Add `self.is_applying.set(false)` before each early return |
| Live preview re-triggered in Source mode | No guard when `is_source_view = true` | `editor.rs` | `if self.is_applying.get() \|\| self.is_source_view.get()` |
| Iterator invalidated after `buffer.insert()` | `start` iterator captured *before* insert, used *after* (invalid) | `preview.rs` | Save `end_iter().offset()`, recreate via `iter_at_offset()` after insert |

## 3. GTK CSS Warnings

| Warning | Location | Fix |
|---|---|---|
| `text-align` not a property | `data/style.css:237` | Removed from `.ctx-button` |
| `vh` is not a valid unit | `data/style.css:299` | `50vh` → `200px` in `.typewriter textview` |

## 4. View Mode Refactor

| Old (3 buttons, Split default) | New (3 buttons, Live default) |
|---|---|
| `Editor` / `Preview` / `Split` | `Source` / `Live` / `Split` |
| Sidebar always visible | Toggleable via headerbar button |
| `toggle_source_view()` only | Added `set_source_view(bool)` |
| paned started with both children | paned starts with only editor child |

## Prevent-Regression Checklist

Before any push:
- [ ] `.weight()` uses `700` (not `Weight::Bold`)
- [ ] No `add_tag()` with single arg — use `tag_table().add()` to register, `apply_tag()` to apply
- [ ] `Tag::Bold/Italic/BlockQuote` → `Tag::Strong/Emphasis/BlockQuote(_kind)`
- [ ] `TagEnd::Bold/Italic` → `TagEnd::Strong/Emphasis`
- [ ] `Event::InlineHtml` still covered
- [ ] Iterators never captured before `buffer.insert()` or `*_tag()` — use offset + `iter_at_offset()`
- [ ] `Popover` has `set_parent()` before `popup()`
- [ ] No `gtk::EventControllerRightClick` — use `GestureClick::new().set_button(3)`
- [ ] `is_applying` guard reset EVERY early return
- [ ] `set_title()` wraps in `Some()`
- [ ] `mark.name()` uses `.as_deref()`
- [ ] `GtkBox` builder uses `.orientation(Orientation::Horizontal)` correctly
- [ ] Paned `set_start_child` / `set_end_child` called in correct order (child must not already be parented elsewhere)
- [ ] `sidebar.container()` appended to correct container (not double-parented)
