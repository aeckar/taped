# taped

A lightweight cursor for non-linear parsing of byte slices.

[![Rust](https://github.com/aeckar/taped/actions/workflows/rust.yml/badge.svg)](https://github.com/aeckar/taped/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/taped.svg)](https://crates.io/crates/taped)

## Documentation

[https://docs.rs/taped](https://docs.rs/taped)

## Overview

`Tape` wraps a byte slice with a position cursor, providing methods for
scanning, backtracking, and consuming bytes without allocating.

- Optimized character and string search via [`memchr`](https://github.com/BurntSushi/memchr)
    - Controlled by `intrinsics` feature flag
- Simple lookahead and lookbehind of whitespace characters
- Indentation counting and paragraph/line awareness

Originally developed as the byte reader for [`bincake`](https://github.com/aeckar/bincake),
extracted as a standalone primitive after the same pattern appeared
across multiple projects.

## Example

```rust
use taped::ToTape;

let data = b"hello world!";
let mut tape = data.to_tape();

tape.seek(|ch, _| ch == b' '); // advance to space
tape.adv(); // skip one character
let word = tape.consume(|ch, _| ch != b'!'); // consume "hello"
assert_eq!(word, b"hello");
assert_eq!(tape.rest(), b"!");
```

## When to use this

- Writing a parser for a binary or text format
- Scanning byte sequences without regex or [`nom`](https://github.com/rust-bakery/nom)
- Anywhere you need backtracking via cheap position snapshots (`tape.clone()`)

## When not to use this

- Full parser combinator framework → use [`nom`](https://github.com/rust-bakery/nom) or [`winnow`](https://github.com/winnow-rs/winnow)
- Lexer generation → use [`logos`](https://github.com/maciejhirsz/logos)
- Async byte streams → use [`tokio::io::AsyncBufRead`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufRead.html)
