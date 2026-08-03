# Multi-Buffer Development Guide

A reference for creating actions and working with multi-buffers in Zed.

## Core Types

### `MultiBuffer` (crate: `multi_buffer`)
A view over one or more physical `Buffer`s. Contains excerpts (slices) from buffers.

- `Entity<MultiBuffer>` - The entity handle used throughout the editor
- `MultiBufferSnapshot` - Immutable snapshot for reading state
- `ExcerptId` - Unique identifier for an excerpt within a multi-buffer
- `ExcerptRange<T>` - A range within a buffer, with `context` (full range) and `primary` (highlighted) fields

### `Buffer` (crate: `language`)
A physical text buffer. Identified by `BufferId` (a `u64` internally).

### `Editor` (crate: `editor`)
The UI component. Has a `buffer: Entity<MultiBuffer>` field.

## Key Multi-Buffer Methods

### Adding Excerpts

```rust
// Add excerpts at the end
multi_buffer.push_excerpts(
    buffer: Entity<Buffer>,
    ranges: impl IntoIterator<Item = ExcerptRange<O>>,
    cx: &mut Context<Self>,
) -> Vec<ExcerptId>

// Insert after a specific excerpt
multi_buffer.insert_excerpts_after(
    prev_excerpt_id: ExcerptId,
    buffer: Entity<Buffer>,
    ranges: impl IntoIterator<Item = ExcerptRange<O>>,
    cx: &mut Context<Self>,
) -> Vec<ExcerptId>

// Set excerpts for a specific path (replaces existing)
multi_buffer.set_excerpts_for_path(
    path: PathKey,
    buffer: Entity<Buffer>,
    ranges: impl IntoIterator<Item = Range<Point>>,
    context_line_count: u32,
    cx: &mut Context<Self>,
) -> (Vec<Range<Anchor>>, bool)
```

### Removing Excerpts

```rust
multi_buffer.remove_excerpts(
    excerpt_ids: impl IntoIterator<Item = ExcerptId>,
    cx: &mut Context<Self>,
)

multi_buffer.remove_excerpts_for_path(path: PathKey, cx: &mut Context<Self>)
multi_buffer.remove_excerpts_for_buffer(buffer_id: BufferId, cx: &mut Context<Self>)
multi_buffer.clear(cx: &mut Context<Self>)
```

### Querying

```rust
// Check if singleton (single buffer, single excerpt)
multi_buffer.is_singleton() -> bool
multi_buffer.as_singleton() -> Option<Entity<Buffer>>

// Get buffer by ID
multi_buffer.buffer(buffer_id: BufferId) -> Option<Entity<Buffer>>

// Get snapshot
multi_buffer.snapshot(cx: &App) -> MultiBufferSnapshot

// Iterate buffers
multi_buffer.for_each_buffer(f: impl FnMut(&Entity<Buffer>))
```

## Creating Editor Actions

### 1. Define the Action (`crates/editor/src/actions.rs`)

Add to the `actions!` macro in alphabetical order:

```rust
actions!(
    editor,
    [
        /// Documentation for your action.
        YourActionName,
    ]
);
```

### 2. Implement the Handler (`crates/editor/src/editor.rs`)

Add a method on `Editor`:

```rust
pub fn your_action_name(
    &mut self,
    _: &YourActionName,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    // Access the multi-buffer
    self.buffer.update(cx, |multi_buffer, cx| {
        // Modify multi-buffer here
    });
}
```

### 3. Register the Action (`crates/editor/src/element.rs`)

In `EditorElement::register_actions()`:

```rust
register_action(editor, window, Editor::your_action_name);
```

### 4. Add Keybinding (optional)

In `assets/keymaps/default-{platform}.json`:

```json
"your-keybinding": "editor::YourActionName"
```

## Accessing External State from Editor

```rust
// Get project (for buffer lookups, LSP, etc.)
self.project() -> Option<&Entity<Project>>

// Get workspace (for notifications, panes, etc.)
self.workspace() -> Option<Entity<Workspace>>

// Get buffer by ID through project
project.read(cx).buffer_for_id(buffer_id, cx) -> Option<Entity<Buffer>>
```

## Clipboard Metadata for Buffer References

The copy system stores buffer references in clipboard metadata:

```rust
#[derive(Serialize, Deserialize)]
pub struct CopyMetadata {
    pub selections: Vec<ClipboardSelection>,
    pub buffer_id: Option<u64>,    // BufferId as u64
    pub range: Option<((u32, u32), (u32, u32))>,  // (start, end) as Points
}
```

### Reading Clipboard with Metadata

```rust
if let Some(item) = cx.read_from_clipboard() {
    if let Some(ClipboardEntry::String(clipboard_string)) = item.entries().first() {
        if let Some(metadata) = clipboard_string.metadata_json::<CopyMetadata>() {
            // Use metadata.buffer_id and metadata.range
        }
    }
}
```

### Writing Clipboard with Metadata

```rust
cx.write_to_clipboard(ClipboardItem::new_string_with_json_metadata(text, metadata));
```

## Common Patterns

### Converting Points Between Multi-Buffer and Buffer

```rust
// Multi-buffer point to buffer point
let snapshot = multi_buffer.snapshot(cx);
snapshot.point_to_buffer_point(point) -> Option<(&BufferSnapshot, Point, ExcerptId)>

// Range to buffer ranges
snapshot.range_to_buffer_ranges(range) -> Vec<(&BufferSnapshot, Range<BufferOffset>, ExcerptId)>
```

### Creating an ExcerptRange

```rust
// Simple range (context = primary)
ExcerptRange::new(start..end)

// With separate primary highlight
ExcerptRange {
    context: full_start..full_end,
    primary: highlight_start..highlight_end,
}
```

## Events

`MultiBuffer` emits events via `EventEmitter<Event>`:

- `ExcerptsAdded { buffer, predecessor, excerpts }`
- `ExcerptsRemoved { ids, removed_buffer_ids }`
- `ExcerptsExpanded { ids }`
- `Edited { edited_buffer }`
- `Reloaded`, `Saved`, `DirtyChanged`, etc.

Subscribe to events in editor via `cx.subscribe(&multi_buffer, handler)`.

