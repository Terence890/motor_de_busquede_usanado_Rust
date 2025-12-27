# fast-search-engine-rust

Minimal in-memory inverted-index search engine demo written in Rust.

## Overview

- Implements a tiny inverted index with documents stored in memory.
- Query tokens are normalized to lowercase and combined with AND semantics (a document must contain every token to match).
- The crate includes a small example dataset (4 documents) used by the demo binary.

## Install

Requires Rust toolchain (rustup + cargo).

To install the demo binary locally:

```bash
cd fast-search-engine-rust
cargo install --path .
```

The installed binary will be available at `~/.cargo/bin/fast-search-engine-rust`.

## Usage

Run the demo (build+run) from the crate directory:

```bash
cd fast-search-engine-rust
cargo run -- 'search terms'
```

Examples:

- `cargo run -- 'rust'` — matches the document containing `Rust`.
- `cargo run -- 'rust search'` — returns no results because the demo uses AND semantics.

If no query is provided the program runs the interactive/demo mode and prints the indexed document count.

## Development

- Build: `cargo build` (inside `fast-search-engine-rust`)
- Run tests: `cargo test`
- Benchmarks: `cargo bench` (there is a `benches/throughput.rs` file)

## Notes

- I added a top-level `.gitignore` to ignore all `target/` directories (`**/target/`).
- To change search semantics from AND to OR, update `src/indexer.rs` (the `search` method).

## Contributing

Feel free to open issues or pull requests. Small improvements:

- support OR queries
- add tokenization and stemming
- persist index to disk
