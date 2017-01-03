# Textwrap

[![](https://img.shields.io/crates/v/textwrap.svg)][crates-io]
[![](https://docs.rs/textwrap/badge.svg)][api-docs]
[![](https://travis-ci.org/mgeisler/textwrap.svg?branch=master)][travis-ci]

Textwrap is a small Rust crate for word wrapping strings. You can use
it to format strings for display in commandline applications.

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
textwrap = "0.2"
```

and this to your crate root:
```rust
extern crate textwrap;
```

You can now easily word wrap strings:
```rust
use textwrap::fill;

fn main() {
    println!("{}", fill("textwrap: a small text wrapping library.", 20));
}
```
The output is
```
textwrap: a small
text wrapping
library.
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

## Examples

The library comes with a small example program that shows how a fixed
example string is wrapped at different widths:
```shell
$ cargo run --example layout
.--- Width: 15 ---.
| Memory safety   |
| without garbage |
| collection.     |
| Concurrency     |
| without data    |
| races. Zero-    |
| cost            |
| abstractions.   |
.--- Width: 16 ----.
| Memory safety    |
| without garbage  |
| collection.      |
| Concurrency      |
| without data     |
| races. Zero-cost |
| abstractions.    |
# ...
.---------------- Width: 41 ----------------.
| Memory safety without garbage collection. |
| Concurrency without data races. Zero-cost |
| abstractions.                             |
.---------------------- Width: 53 ----------------------.
| Memory safety without garbage collection. Concurrency |
| without data races. Zero-cost abstractions.           |
```

## Changelog

* **Version 0.2.0, December 28 2016:** Introduced `Wrapper` struct.
  Added support for wrapping on hyphens.

* **Version 0.1.0, December 17 2016:** First public release with
  support for wrapping strings on whitespace.

## License

Textwrap can be distributed according to the [MIT license][mit].
Contributions will be accepted under the same license.

[crates-io]: https://crates.io/crates/textwrap
[travis-ci]: https://travis-ci.org/mgeisler/textwrap
[api-docs]: https://docs.rs/textwrap/
[unicode-width]: https://unicode-rs.github.io/unicode-width/unicode_width/index.html
[mit]: LICENSE
