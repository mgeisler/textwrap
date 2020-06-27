# Textwrap

[![](https://travis-ci.org/mgeisler/textwrap.svg?branch=master)][travis-ci]
[![](https://ci.appveyor.com/api/projects/status/github/mgeisler/textwrap?branch=master&svg=true)][appveyor]
[![](https://codecov.io/gh/mgeisler/textwrap/branch/master/graph/badge.svg)][codecov]
[![](https://img.shields.io/crates/v/textwrap.svg)][crates-io]
[![](https://docs.rs/textwrap/badge.svg)][api-docs]

Textwrap is a small Rust crate for word wrapping text. You can use it
to format strings for display in commandline applications. The crate
name and interface is inspired by
the [Python textwrap module][py-textwrap].

## Usage

To use `textwrap`, add this to your `Cargo.toml` file:
```toml
[dependencies]
textwrap = "0.12"
```

This gives you the text wrapping without of the optional features
listed next.

### `hyphenation`

If you would like to have automatic language-sensitive hyphenation,
enable the `hyphenation` feature:

```toml
[dependencies]
textwrap = { version = "0.12", features = ["hyphenation"] }
```

This gives you hyphenation support for US English. Please see the
[`hyphenation` example] for an executable demo. Read the Getting
Started section below to see how to load the hyphenation patterns for
other languages.

### `terminal_size`

To conveniently wrap text at the current terminal width, enable the
`terminal_size` feature:

```toml
[dependencies]
textwrap = { version = "0.12", features = ["terminal_size"] }
```

Please see the [`termwidth` example] for how to use this feature.

## Documentation

**[API documentation][api-docs]**

## Getting Started

Word wrapping single strings is easy using the `fill` function:
```rust
fn main() {
    let text = "textwrap: a small library for wrapping text.";
    println!("{}", textwrap::fill(text, 18));
}
```
The output is
```
textwrap: a small
library for
wrapping text.
```

If you enable the `hyphenation` feature, you get support for automatic
hyphenation for [about 70 languages][patterns] via high-quality TeX
hyphenation patterns.

Your program must load the hyphenation pattern and call
`Wrapper::with_splitter` to use it:

```rust
use hyphenation::{Language, Load, Standard};
use textwrap::Wrapper;

fn main() {
    let hyphenator = Standard::from_embedded(Language::EnglishUS).unwrap();
    let wrapper = Wrapper::with_splitter(18, hyphenator);
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

The US-English hyphenation patterns are embedded when you enable the
`hyphenation` feature. They are licensed under a [permissive
license][en-us license] and take up about 88 KB of space in your
application. If you need hyphenation for other languages, you need to
download a [precompiled `.bincode` file][bincode] and load it
yourself. Please see the [`hyphenation` documentation] for details.

## Wrapping Strings at Compile Time

If your strings are known at compile time, please take a look at the
procedural macros from the [`textwrap-macros` crate].


## Examples

The library comes with some small example programs that shows various
features.

### Layout Example

The `layout` example shows how a fixed example string is wrapped at
different widths. Run the example with:

```shell
$ cargo run --features hyphenation --example layout
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

Notice how words are split at hyphens (such as "zero-cost") but also
how words are hyphenated using automatic/machine hyphenation.

### Terminal Width Example

The `termwidth` example simply shows how the width can be set
automatically to the current terminal width. Run it with this command:

```
$ cargo run --example termwidth
```

If you run it in a narrow terminal, you'll see output like this:
```
Formatted in within 60 columns:
----
Memory safety without garbage collection. Concurrency
without data races. Zero-cost abstractions.
----
```

If `stdout` is not connected to the terminal, the program will use a
default of 80 columns for the width:

```
$ cargo run --example termwidth | cat
Formatted in within 80 columns:
----
Memory safety without garbage collection. Concurrency without data races. Zero-
cost abstractions.
----
```

## Release History

Please see the [CHANGELOG file] for details on the changes made in
each release.

## License

Textwrap can be distributed according to the [MIT license][mit].
Contributions will be accepted under the same license.

[crates-io]: https://crates.io/crates/textwrap
[travis-ci]: https://travis-ci.org/mgeisler/textwrap
[appveyor]: https://ci.appveyor.com/project/mgeisler/textwrap
[codecov]: https://codecov.io/gh/mgeisler/textwrap
[py-textwrap]: https://docs.python.org/library/textwrap
[`textwrap-macros` crate]: https://crates.io/crates/textwrap-macros
[`hyphenation` example]: https://github.com/mgeisler/textwrap/blob/master/examples/hyphenation.rs
[`termwidth` example]: https://github.com/mgeisler/textwrap/blob/master/examples/termwidth.rs
[patterns]: https://github.com/tapeinosyne/hyphenation/tree/master/patterns-tex
[en-us license]: https://github.com/hyphenation/tex-hyphen/blob/master/hyph-utf8/tex/generic/hyph-utf8/patterns/tex/hyph-en-us.tex
[bincode]: https://github.com/tapeinosyne/hyphenation/tree/master/dictionaries
[`hyphenation` documentation]: http://docs.rs/hyphenation
[api-docs]: https://docs.rs/textwrap/
[CHANGELOG file]: https://github.com/mgeisler/textwrap/blob/master/CHANGELOG.md
[mit]: LICENSE
