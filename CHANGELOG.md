# Changelog

This file lists the most important changes made in each release of
`textwrap`.

## Version 0.12.1 — July 3rd, 2020

This is a bugfix release.

* Fixed [#176][issue-176]: Mention compile-time wrapping by linking to
  the [`textwrap-macros` crate].
* Fixed [#193][issue-193]: Wrapping with `break_words(false)` was
  broken and would cause extra whitespace to be inserted when words
  were longer than the line width.

## Version 0.12.0 — June 26th, 2020

The code has been updated to the [Rust 2018 edition][rust-2018] and
each new release of `textwrap` will only support the latest stable
version of Rust. Trying to support older Rust versions is a fool's
errand: our dependencies keep releasing new patch versions that
require newer and newer versions of Rust.

The `term_size` feature has been replaced by `terminal_size`. The API
is unchanged, it is just the name of the Cargo feature that changed.

The `hyphenation` feature now only embeds the hyphenation patterns for
US-English. This slims down the dependency.

* Fixed [#140][issue-140]: Ignore ANSI escape sequences.
* Fixed [#158][issue-158]: Unintended wrapping when using external splitter.
* Fixed [#177][issue-177]: Update examples to the 2018 edition.

## Version 0.11.0 — December 9th, 2018

Due to our dependencies bumping their minimum supported version of
Rust, the minimum version of Rust we test against is now 1.22.0.

* Merged [#141][issue-141]: Fix `dedent` handling of empty lines and
  trailing newlines. Thanks @bbqsrc!
* Fixed [#151][issue-151]: Release of version with hyphenation 0.7.

## Version 0.10.0 — April 28th, 2018

Due to our dependencies bumping their minimum supported version of
Rust, the minimum version of Rust we test against is now 1.17.0.

* Fixed [#99][issue-99]: Word broken even though it would fit on line.
* Fixed [#107][issue-107]: Automatic hyphenation is off by one.
* Fixed [#122][issue-122]: Take newlines into account when wrapping.
* Fixed [#129][issue-129]: Panic on string with em-dash.

## Version 0.9.0 — October 5th, 2017

The dependency on `term_size` is now optional, and by default this
feature is not enabled. This is a *breaking change* for users of
`Wrapper::with_termwidth`. Enable the `term_size` feature to restore
the old functionality.

Added a regression test for the case where `width` is set to
`usize::MAX`, thanks @Fraser999! All public structs now implement
`Debug`, thanks @hcpl!

* Fixed [#101][issue-101]: Make `term_size` an optional dependency.

## Version 0.8.0 — September 4th, 2017

The `Wrapper` stuct is now generic over the type of word splitter
being used. This means less boxing and a nicer API. The
`Wrapper::word_splitter` method has been removed. This is a *breaking
API change* if you used the method to change the word splitter.

The `Wrapper` struct has two new methods that will wrap the input text
lazily: `Wrapper::wrap_iter` and `Wrapper::into_wrap_iter`. Use those
if you will be iterating over the wrapped lines one by one.

* Fixed [#59][issue-59]: `wrap` could return an iterator. Thanks
  @hcpl!
* Fixed [#81][issue-81]: Set `html_root_url`.

## Version 0.7.0 — July 20th, 2017

Version 0.7.0 changes the return type of `Wrapper::wrap` from
`Vec<String>` to `Vec<Cow<'a, str>>`. This means that the output lines
borrow data from the input string. This is a *breaking API change* if
you relied on the exact return type of `Wrapper::wrap`. Callers of the
`textwrap::fill` convenience function will see no breakage.

The above change and other optimizations makes version 0.7.0 roughly
15-30% faster than version 0.6.0.

The `squeeze_whitespace` option has been removed since it was
complicating the above optimization. Let us know if this option is
important for you so we can provide a work around.

* Fixed [#58][issue-58]: Add a "fast_wrap" function.
* Fixed [#61][issue-61]: Documentation errors.

## Version 0.6.0 — May 22nd, 2017

Version 0.6.0 adds builder methods to `Wrapper` for easy one-line
initialization and configuration:

```rust
let wrapper = Wrapper::new(60).break_words(false);
```

It also add a new `NoHyphenation` word splitter that will never split
words, not even at existing hyphens.

* Fixed [#28][issue-28]: Support not squeezing whitespace.

## Version 0.5.0 — May 15th, 2017

Version 0.5.0 has *breaking API changes*. However, this only affects
code using the hyphenation feature. The feature is now optional, so
you will first need to enable the `hyphenation` feature as described
above. Afterwards, please change your code from
```rust
wrapper.corpus = Some(&corpus);
```
to
```rust
wrapper.splitter = Box::new(corpus);
```

Other changes include optimizations, so version 0.5.0 is roughly
10-15% faster than version 0.4.0.

* Fixed [#19][issue-19]: Add support for finding terminal size.
* Fixed [#25][issue-25]: Handle words longer than `self.width`.
* Fixed [#26][issue-26]: Support custom indentation.
* Fixed [#36][issue-36]: Support building without `hyphenation`.
* Fixed [#39][issue-39]: Respect non-breaking spaces.

## Version 0.4.0 — January 24th, 2017

Documented complexities and tested these via `cargo bench`.

* Fixed [#13][issue-13]: Immediatedly add word if it fits.
* Fixed [#14][issue-14]: Avoid splitting on initial hyphens.

## Version 0.3.0 — January 7th, 2017

Added support for automatic hyphenation.

## Version 0.2.0 — December 28th, 2016

Introduced `Wrapper` struct. Added support for wrapping on hyphens.

## Version 0.1.0 — December 17th, 2016

First public release with support for wrapping strings on whitespace.

[rust-2018]: https://doc.rust-lang.org/edition-guide/rust-2018/
[`textwrap-macros` crate]: https://crates.io/crates/textwrap-macros

[issue-13]: https://github.com/mgeisler/textwrap/issues/13
[issue-14]: https://github.com/mgeisler/textwrap/issues/14
[issue-19]: https://github.com/mgeisler/textwrap/issues/19
[issue-25]: https://github.com/mgeisler/textwrap/issues/25
[issue-26]: https://github.com/mgeisler/textwrap/issues/26
[issue-28]: https://github.com/mgeisler/textwrap/issues/28
[issue-36]: https://github.com/mgeisler/textwrap/issues/36
[issue-39]: https://github.com/mgeisler/textwrap/issues/39
[issue-58]: https://github.com/mgeisler/textwrap/issues/58
[issue-59]: https://github.com/mgeisler/textwrap/issues/59
[issue-61]: https://github.com/mgeisler/textwrap/issues/61
[issue-81]: https://github.com/mgeisler/textwrap/issues/81
[issue-99]: https://github.com/mgeisler/textwrap/issues/99
[issue-101]: https://github.com/mgeisler/textwrap/issues/101
[issue-107]: https://github.com/mgeisler/textwrap/issues/107
[issue-122]: https://github.com/mgeisler/textwrap/issues/122
[issue-129]: https://github.com/mgeisler/textwrap/issues/129
[issue-140]: https://github.com/mgeisler/textwrap/issues/140
[issue-141]: https://github.com/mgeisler/textwrap/issues/141
[issue-151]: https://github.com/mgeisler/textwrap/issues/151
[issue-158]: https://github.com/mgeisler/textwrap/issues/158
[issue-177]: https://github.com/mgeisler/textwrap/issues/177
[issue-193]: https://github.com/mgeisler/textwrap/issues/193
