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
textwrap = "0.3"
```

and this to your crate root:
```rust
extern crate textwrap;
```

## Examples

Word wrapping single strings is easy using the `fill` function:
```rust
extern crate textwrap;
use textwrap::fill;

fn main() {
    let output = "textwrap: a small library for wrapping output.";
    println!("{}", fill(output, 18));
}
```
The output is
```
textwrap: a small
library for
wrapping output.
```

You can use automatic hyphenation using TeX hyphenation patterns (with
support for [about 70 languages][patterns]) and get:
```rust
extern crate hyphenation;
extern crate textwrap;

use hyphenation::Language;
use textwrap::Wrapper;

fn main() {
    let corpus = hyphenation::load(Language::English_US).unwrap();
    let mut wrapper = Wrapper::new(18);
    wrapper.corpus = Some(&corpus);
    let output = "textwrap: a small library for wrapping output.";
    println!("{}", wrapper.fill(output))
}
```

The output now looks like this:
```
textwrap: a small
library for wrap-
ping output.
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
example string is wrapped at different widths. The string is

> Memory safety without garbage collection. Concurrency without data
> races. Zero-cost abstractions.

When run, the string is wrapped at all widths between 15 and 60
columns:

```shell
$ cargo run --example layout
.--- Width: 15 ---.
| Memory safety   |
| without garbage |
| collection.     |
| Concurrency     |
| without data    |
| races. Zero-    |
| cost abstrac-   |
| tions.          |
.--- Width: 16 ----.
| Memory safety    |
| without garbage  |
| collection. Con- |
| currency without |
| data races. Ze-  |
| ro-cost abstrac- |
| tions.           |
# ...
.-------------------- Width: 49 --------------------.
| Memory safety without garbage collection. Concur- |
| rency without data races. Zero-cost abstractions. |
.---------------------- Width: 53 ----------------------.
| Memory safety without garbage collection. Concurrency |
| without data races. Zero-cost abstractions.           |
.------------------------- Width: 59 -------------------------.
| Memory safety without garbage collection. Concurrency with- |
| out data races. Zero-cost abstractions.                     |
```

Notice how words are split at hyphens (such a s "zero-cost") but also
how words are hyphenated using automatic/machine hyphenation.

## Release History

This section lists the largest changes per release.

### Version 0.3.0 — January 7th, 2017

Added support for automatic hyphenation.

### Version 0.2.0 — December 28th, 2016

Introduced `Wrapper` struct. Added support for wrapping on hyphens.

### Version 0.1.0 — December 17th, 2016

First public release with support for wrapping strings on whitespace.

## License

Textwrap can be distributed according to the [MIT license][mit].
Contributions will be accepted under the same license.

[crates-io]: https://crates.io/crates/textwrap
[travis-ci]: https://travis-ci.org/mgeisler/textwrap
[patterns]: https://github.com/tapeinosyne/hyphenation/tree/master/patterns-tex
[api-docs]: https://docs.rs/textwrap/
[unicode-width]: https://unicode-rs.github.io/unicode-width/unicode_width/index.html
[mit]: LICENSE
