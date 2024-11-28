# Trext-rs

This is a Rust implementation of a Text Buffer. It contains all the basic implementations to be connected to a UI whether its a TUI or a GUI.

Why did I make this? To understand the VS Code implementation of a text buffer and text buffers in general.

## Text Buffers

There are three main types of text buffers:

- **Gap Buffer**: Two Buffers with a temporary buffer in the middle for insertion.
- **Piece Table**: Buffers with pieces pointing to sections of the buffers.
- **Piece Tree**: Like a `Piece Table` but the pieces are in a binary tree.

There are several benifits to `PieceTree` and `PieceTable` over a `GapBuffer`.

- `PieceTree` and `PieceTable` can split text into seperate loadable buffers. This allows for large files to be streamed.
- `PieceTree` and `PieceTable` easily allow for storing edits that can be undone. This is because the buffers themselves are append only, meaning no text is actually removed on deletion. Only if the buffers are refreshed will the text be deleted.
- `PieceTable`s have quicker times for jumps, to line or character position.

### Implementation

**Progress**:

- [ ] PieceTable
- [ ] PieceTree

| Feature           | `PieceTable` | `PieceTree` |
| :---------------- | :----------: | :---------: |
| Load String       |      X       |             |
| Load File         |      X       |             |
| Read Buffer       |      X       |             |
| Piece Count       |      X       |             |
| Buffer Count      |      X       |             |
| Total Buffer Size |      X       |             |
| Insert Piece      |      X       |             |
| Delete Piece      |              |             |
| Read Piece        |      X       |             |

## Layout

### Hierarchy

Each item in the list is used to compose the next one.

1. Characters
1. Words
1. Lines
1. Pages
1. Documents

## Crates

### Trext-rs

The interactive front-end for the text buffers.

```toml
[dependenies]
common = { version = "0.1.0", path = "../libs/common" }
raylib = "5.0.2"
```

### Lib/Common

The common types needed to run the application.

```toml
[dependencies]
regex = "1.11.1"
```

## Sources

- [VS-Code-TextBuffer](https://github.com/microsoft/vscode-textbuffer/tree/main)
- [Text Buffer Implementation](https://code.visualstudio.com/blogs/2018/03/23/text-buffer-reimplementation)
- [Piece Table](https://en.wikipedia.org/wiki/Piece_table)
- [Gap Buffer](https://en.wikipedia.org/wiki/Gap_buffer)
- [Craft Text Editing](https://web.mit.edu/~yandros/doc/craft-text-editing/index.html)
