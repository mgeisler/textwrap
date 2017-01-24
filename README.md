# Textwrap

[![](https://img.shields.io/crates/v/textwrap.svg)][crates-io]
[![](https://docs.rs/textwrap/badge.svg)][api-docs]
[![](https://travis-ci.org/mgeisler/textwrap.svg?branch=master)][travis-ci]

Textwrap is a small Rust crate for word wrapping text. You can use it
to format strings for display in commandline applications.

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
textwrap = "0.4"
```

and this to your crate root:
```rust
extern crate textwrap;
```

## Documentation

**[API documentation][api-docs]**

## Getting Started

Word wrapping single strings is easy using the `fill` function:
```rust
extern crate textwrap;
use textwrap::fill;

fn main() {
    let text = "textwrap: a small library for wrapping text.";
    println!("{}", fill(text, 18));
}
```
The output is
```
textwrap: a small
library for
wrapping text.
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
    let text = "textwrap: a small library for wrapping text.";
    println!("{}", wrapper.fill(text))
}
```

The output now looks like this:
```
textwrap: a small
library for wrap-
ping text.
```

## Examples

The library comes with a small example program that shows how a fixed
example string is wrapped at different widths. Run the example with:

```shell
$ cargo run --example layout
```

The program will use the following string:

> Memory safety without garbage collection. Concurrency without data
> races. Zero-cost abstractions.

The string is wrapped at all widths between 15 and 60 columns. With
narrow columns the output looks like this:

```
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
```

Later, longer lines are used and the output now looks like this:

```
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

### Version 0.4.0 — January 24th, 2017

Documented complexities and tested these via `cargo bench`.

* Fixed [#13][issue-13]: Immediatedly add word if it fits
* Fixed [#14][issue-14]: Avoid splitting on initial hyphens in `--foo-bar`

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
[issue-13]: ../../issues/13
[issue-14]: ../../issues/14
[mit]: LICENSE
