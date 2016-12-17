# Textwrap

Textwrap is a small Rust crate for word wrapping strings. You can use
it to format strings for display in commandline applications.

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
textwrap = "0.1"
```

and this to your crate root:
```rust
extern crate textwrap;
```

## Documentation

**[API documentation][api-docs]**

Strings are wrapped based on their [displayed width][unicode-width],
not their size in bytes. For ASCII characters such as `a` and `!`, the
displayed with is the same as the number of bytes used to UTF-8 encode
the character (one character takes up one byte). However, non-ASCII
characters and symbols take up more than one byte: `é` is `0xc3 0xa9`
and `⚙` is `0xe2 0x9a 0x99` in UTF-8, respectively. This means that
relying solely on the string length in bytes would give incorrect
results.

[api-docs]: https://docs.rs/textwrap/
[unicode-width]: https://unicode-rs.github.io/unicode-width/unicode_width/index.html
