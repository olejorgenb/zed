GPUI Max Width and Overflow Handling Summary

## Key Discoveries

### 1. **Setting Max Width**

**Basic syntax:**
- `.max_w(px(400))` - Custom pixel value
- `.max_w(rems(2.0))` - Using rems (as shown in geometry.rs)
- `.max_w_64()`, `.max_w_96()`, `.max_w_full()` - Predefined values

**Relevant files:**
- `zed/crates/gpui/src/style.rs#L198` - `max_size` field in Style struct
- `zed/crates/gpui/src/geometry.rs#L3329-3367` - Length conversion (px, rems, fractions)

### 2. **Text Overflow with Ellipsis**

**Core ellipsis constant:**
```rust
const ELLIPSIS: SharedString = SharedString::new_static("…");
```

**Key methods:**
- `.text_ellipsis()` - Adds "…" for overflowing text
- `.truncate()` - Combines `overflow_hidden()` + `whitespace_nowrap()` + `text_ellipsis()`
- `.line_clamp(n)` - Multi-line truncation

**Relevant files:**
- `zed/crates/gpui/src/styled.rs#L12` - ELLIPSIS constant
- `zed/crates/gpui/src/styled.rs#L82-87` - `text_ellipsis()` method
- `zed/crates/gpui/src/styled.rs#L122-124` - `truncate()` method
- `zed/crates/gpui/src/style.rs#L331-335` - TextOverflow enum

### 3. **General Overflow Control**

**Methods available:**
- `.overflow_hidden()` - Clips all overflow
- `.overflow_scroll()` - Adds scrollbars
- `.overflow_x_hidden()` / `.overflow_y_hidden()` - Axis-specific clipping

**Relevant files:**
- `zed/crates/gpui_macros/src/styles.rs#L132-142` - overflow_hidden() implementation
- `zed/crates/gpui/src/elements/div.rs#L1061-1065` - overflow_scroll() method

### 4. **Label Components**

**HighlightedLabel** (used in pickers):
- Supports `.truncate()` method
- Used in command palette, file finder, branch picker

**LabelLike base implementation:**
```rust
.when(self.truncate, |this| {
    this.overflow_x_hidden().text_ellipsis()
})
```

**Relevant files:**
- `zed/crates/ui/src/components/label/highlighted_label.rs` - HighlightedLabel component
- `zed/crates/ui/src/components/label/label_like.rs#L235-239` - Truncate rendering logic

### 5. **Real-World Usage Examples**

**Command Palette:**
- `zed/crates/command_palette/src/command_palette.rs#L481-488`

**File Finder:**
- `zed/crates/file_finder/src/file_finder.rs#L1148-1153`

**Branch Picker (where you saw ellipsis):**
- `zed/crates/git_ui/src/branch_picker.rs#L492-495`

**Context Pills:**
- `zed/crates/agent_ui/src/ui/context_pill.rs#L147-153`

### 6. **Style Architecture**

The overflow behavior is controlled through the Style struct's fields:
- `max_size: Size<Length>` - Controls maximum dimensions
- `overflow: Point<Overflow>` - Controls overflow behavior per axis
- Text-specific overflow handled via `TextStyle.text_overflow`

**Relevant files:**
- `zed/crates/gpui/src/style.rs#L144-281` - Main Style struct
- `zed/crates/gpui/src/style.rs#L354-399` - TextStyle struct

## Quick Reference

For a div with max width and ellipsis:
```rust
div()
    .max_w(px(400))
    .truncate()
    .child("Your text content")
```

For labels specifically:
```rust
Label::new("Long text content").truncate()
// or
HighlightedLabel::new("Text", positions).truncate()
