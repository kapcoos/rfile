# rfile 📁

A fast, single-shot CLI file and directory inspector written in Rust.

`rfile` combines the functionality of `stat`, `file`, and `du` into a single, quick overview for your terminal.

## Features

- **Human-Readable Sizes**: Formats byte counts automatically (`B`, `KiB`, `MiB`, `GiB`).
- **Directory Inspection**: Calculates total folder size and counts sub-files/folders recursively.
- **Deep Type Detection**: Inspects magic bytes (via `infer`) to detect real MIME types, ignoring fake extensions.
- **Path Resolution**: Resolves full absolute paths on disk.
- **Timestamps**: Shows exact local last-modified dates.

## Installation

```bash
cargo install rfile
