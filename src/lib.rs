//! `textwrap` provides functions for word wrapping and filling text.
//!
//! Wrapping text can be very useful in commandline programs where you
//! want to format dynamic output nicely so it looks good in a
//! terminal. A quick example:
//!
//! ```no_run
//! fn main() {
//!     let text = "textwrap: a small library for wrapping text.";
//!     println!("{}", textwrap::fill(text, 18));
//! }
//! ```
//!
//! When you run this program, it will display the following output:
//!
//! ```text
//! textwrap: a small
//! library for
//! wrapping text.
//! ```
//!
//! If you enable the `hyphenation` feature, you can get automatic
//! hyphenation for a number of languages:
//!
//! ```no_run
//! # #[cfg(feature = "hyphenation")]
//! use hyphenation::{Language, Load, Standard};
//! use textwrap::{Options, fill};
//!
//! # #[cfg(feature = "hyphenation")]
//! fn main() {
//!     let text = "textwrap: a small library for wrapping text.";
//!     let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
//!     let options = Options::new(18).splitter(dictionary);
//!     println!("{}", fill(text, &options));
//! }
//!
//! # #[cfg(not(feature = "hyphenation"))]
//! # fn main() { }
//! ```
//!
//! The program will now output:
//!
//! ```text
//! textwrap: a small
//! library for wrap-
//! ping text.
//! ```
//!
//! # Wrapping Strings at Compile Time
//!
//! If your strings are known at compile time, please take a look at
//! the procedural macros from the [`textwrap-macros` crate].
//!
//! # Displayed Width vs Byte Size
//!
//! To word wrap text, one must know the width of each word so one can
//! know when to break lines. This library measures the width of text
//! using the [displayed width][unicode-width], not the size in bytes.
//!
//! This is important for non-ASCII text. ASCII characters such as `a`
//! and `!` are simple and take up one column each. This means that
//! the displayed width is equal to the string length in bytes.
//! However, non-ASCII characters and symbols take up more than one
//! byte when UTF-8 encoded: `é` is `0xc3 0xa9` (two bytes) and `⚙` is
//! `0xe2 0x9a 0x99` (three bytes) in UTF-8, respectively.
//!
//! This is why we take care to use the displayed width instead of the
//! byte count when computing line lengths. All functions in this
//! library handle Unicode characters like this.
//!
//! # Cargo Features
//!
//! The library has two optional features:
//!
//! * `terminal_size`: enables automatic detection of the terminal
//!   width via the [terminal_size][] crate. See the
//!   [`Options::with_termwidth`] constructor for details.
//!
//! * `hyphenation`: enables language-sentive hyphenation via the
//!   [hyphenation][] crate. See the [`WordSplitter`] trait for
//!   details.
//!
//! [`textwrap-macros` crate]: https://crates.io/crates/textwrap-macros
//! [unicode-width]: https://docs.rs/unicode-width/
//! [terminal_size]: https://crates.io/crates/terminal_size
//! [hyphenation]: https://crates.io/crates/hyphenation
//! [`Options::with_termwidth`]: struct.Options.html#method.with_termwidth
//! [`WordSplitter`]: trait.WordSplitter.html

#![doc(html_root_url = "https://docs.rs/textwrap/0.12.1")]
#![forbid(unsafe_code)] // See https://github.com/mgeisler/textwrap/issues/210
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::redundant_field_names)]

use std::borrow::Cow;
use unicode_width::UnicodeWidthStr;

mod indentation;
pub use crate::indentation::dedent;
pub use crate::indentation::indent;

mod splitting;
pub use crate::splitting::{HyphenSplitter, NoHyphenation, WordSplitter};

pub mod core;

/// Holds settings for wrapping and filling text.
#[derive(Debug, Clone)]
pub struct Options<'a, S = Box<dyn WordSplitter>> {
    /// The width in columns at which the text will be wrapped.
    pub width: usize,
    /// Indentation used for the first line of output.
    pub initial_indent: &'a str,
    /// Indentation used for subsequent lines of output.
    pub subsequent_indent: &'a str,
    /// Allow long words to be broken if they cannot fit on a line.
    /// When set to `false`, some lines may be longer than
    /// `self.width`.
    pub break_words: bool,
    /// The method for splitting words. If the `hyphenation` feature
    /// is enabled, you can use a `hyphenation::Standard` dictionary
    /// here to get language-aware hyphenation.
    pub splitter: S,
}

impl<'a, S> From<&'a Options<'a, S>> for Options<'a, &'a S> {
    fn from(options: &'a Options<'a, S>) -> Self {
        Self {
            width: options.width,
            initial_indent: options.initial_indent,
            subsequent_indent: options.subsequent_indent,
            break_words: options.break_words,
            splitter: &options.splitter,
        }
    }
}

impl<'a> From<usize> for Options<'a, HyphenSplitter> {
    fn from(width: usize) -> Self {
        Options::new(width)
    }
}

/// Constructors for boxed Options, specifically.
impl<'a> Options<'a, HyphenSplitter> {
    /// Creates a new `Options` with the specified width and static dispatch
    /// using the `HyphenSplitter`. Equivalent to
    ///
    /// ```
    /// # use textwrap::{Options, HyphenSplitter, WordSplitter};
    /// # let width = 80;
    /// # let actual = Options::new(width);
    /// # let expected =
    /// Options {
    ///     width: width,
    ///     initial_indent: "",
    ///     subsequent_indent: "",
    ///     break_words: true,
    ///     splitter: HyphenSplitter,
    /// }
    /// # ;
    /// # assert_eq!(actual.width, expected.width);
    /// # assert_eq!(actual.initial_indent, expected.initial_indent);
    /// # assert_eq!(actual.subsequent_indent, expected.subsequent_indent);
    /// # assert_eq!(actual.break_words, expected.break_words);
    /// # let expected_coerced: Options<'static, HyphenSplitter> = expected;
    /// ```
    ///
    /// Static dispatch mean here, that the splitter is stored as-is and the
    /// type is known at compile-time. Thus the returned value is actually a
    /// `Options<HyphenSplitter>`.
    ///
    /// Dynamic dispatch on the other hand, mean that the splitter is stored as a trait object
    /// for instance in a `Box<dyn WordSplitter>`. This way the splitter's inner type
    /// can be changed without changing the type of this struct, which then would be just
    /// `Options` as a short cut for `Options<Box<dyn WordSplitter>>`.
    ///
    /// The value and type of the splitter can be choose from the start using the `with_splitter`
    /// constructor or changed afterwards using the `splitter` method. Whether static or
    /// dynamic dispatch is used, depends on whether these functions are given a boxed `WordSplitter`
    /// or not. Take for example:
    ///
    /// ```
    /// use textwrap::{Options, NoHyphenation, HyphenSplitter};
    /// # use textwrap::{WordSplitter};
    /// # let width = 80;
    ///
    /// // uses HyphenSplitter with static dispatch
    /// // the actual type: Options<HyphenSplitter>
    /// let opt = Options::new(width);
    /// # let opt_coerce: Options<HyphenSplitter> = opt;
    ///
    /// // uses NoHyphenation with static dispatch
    /// // the actual type: Options<NoHyphenation>
    /// let opt = Options::new(width).splitter(NoHyphenation);
    /// # let opt_coerce: Options<NoHyphenation> = opt;
    ///
    /// // uses HyphenSplitter with dynamic dispatch
    /// // the actual type: Options<Box<dyn WordSplitter>>
    /// let opt: Options = Options::new(width).splitter(Box::new(HyphenSplitter));
    /// # let opt_coerce: Options<Box<dyn WordSplitter>> = opt;
    ///
    /// // uses NoHyphenation with dynamic dispatch
    /// // the actual type: Options<Box<dyn WordSplitter>>
    /// let opt: Options = Options::new(width).splitter(Box::new(NoHyphenation));
    /// # let opt_coerce: Options<Box<dyn WordSplitter>> = opt;
    /// ```
    ///
    /// Notice that the last two variables have the same type, despite the different `WordSplitter`
    /// in use. Thus dynamic dispatch allows to change the splitter at run-time without changing
    /// the variables type.
    ///
    pub const fn new(width: usize) -> Self {
        Options::with_splitter(width, HyphenSplitter)
    }

    /// Creates a new `Options` with `width` set to the current
    /// terminal width. If the terminal width cannot be determined
    /// (typically because the standard input and output is not
    /// connected to a terminal), a width of 80 characters will be
    /// used. Other settings use the same defaults as `Options::new`.
    ///
    /// Equivalent to:
    ///
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// use textwrap::{Options, termwidth};
    ///
    /// let options = Options::new(termwidth());
    /// ```
    ///
    /// **Note:** Only available when the `terminal_size` feature is
    /// enabled.
    #[cfg(feature = "terminal_size")]
    pub fn with_termwidth() -> Self {
        Self::new(termwidth())
    }
}

impl<'a, S> Options<'a, S> {
    /// Creates a new `Options` with the specified width and splitter. Equivalent
    /// to
    ///
    /// ```
    /// # use textwrap::{Options, NoHyphenation, HyphenSplitter};
    /// # const splitter: NoHyphenation = NoHyphenation;
    /// # const width: usize = 80;
    /// # const actual: Options<'static, NoHyphenation> = Options::with_splitter(width, splitter);
    /// # let expected =
    /// Options {
    ///     width: width,
    ///     initial_indent: "",
    ///     subsequent_indent: "",
    ///     break_words: true,
    ///     splitter: splitter,
    /// }
    /// # ;
    /// # assert_eq!(actual.width, expected.width);
    /// # assert_eq!(actual.initial_indent, expected.initial_indent);
    /// # assert_eq!(actual.subsequent_indent, expected.subsequent_indent);
    /// # assert_eq!(actual.break_words, expected.break_words);
    /// # let expected_coerced: Options<'static, NoHyphenation> = expected;
    /// ```
    ///
    /// This constructor allows to specify the splitter to be used. It is like a short-cut for
    /// `Options::new(w).splitter(s)`, but this function is a `const fn`. The given splitter may
    /// be in a `Box`, which then can be coerced into a trait object for dynamic dispatch:
    ///
    /// ```
    /// use textwrap::{Options, NoHyphenation, HyphenSplitter};
    /// # use textwrap::{WordSplitter};
    /// # const width: usize = 80;
    /// # let expected: Options =
    /// # Options {
    /// #    width: width,
    /// #    initial_indent: "",
    /// #    subsequent_indent: "",
    /// #    break_words: true,
    /// #    splitter: Box::new(NoHyphenation),
    /// # }
    /// # ;
    ///
    /// // This opt contains a boxed trait object as splitter.
    /// // Its type is actually: `Options<Box<dyn WordSplitter>>`
    /// // The type annotation is important, otherwise it will be not a trait object
    /// let mut opt: Options = Options::with_splitter(width, Box::new(NoHyphenation));
    /// #
    /// # let actual = opt;
    /// # assert_eq!(actual.width, expected.width);
    /// # assert_eq!(actual.initial_indent, expected.initial_indent);
    /// # assert_eq!(actual.subsequent_indent, expected.subsequent_indent);
    /// # assert_eq!(actual.break_words, expected.break_words);
    /// # let expected_coerced: Options<Box<dyn WordSplitter>> = expected;
    ///
    /// // Thus, it can be overridden with a different splitter.
    /// opt = Options::with_splitter(width, Box::new(HyphenSplitter));
    /// // Now, containing a `HyphenSplitter` instead.
    /// #
    /// # let expected: Options =
    /// # Options {
    /// #    width: width,
    /// #    initial_indent: "",
    /// #    subsequent_indent: "",
    /// #    break_words: true,
    /// #    splitter: Box::new(HyphenSplitter),
    /// # }
    /// # ;
    /// # let actual = opt;
    /// # assert_eq!(actual.width, expected.width);
    /// # assert_eq!(actual.initial_indent, expected.initial_indent);
    /// # assert_eq!(actual.subsequent_indent, expected.subsequent_indent);
    /// # assert_eq!(actual.break_words, expected.break_words);
    /// # let expected_coerced: Options<Box<dyn WordSplitter>> = expected;
    /// ```
    ///
    /// Since the splitter is given by value, which determines the generic
    /// type parameter, it can be used to produce both an `Options` with static
    /// and dynamic dispatch, respectively.
    /// While dynamic dispatch allows to change the type of the inner splitter
    /// at run time as seen above, static dispatch especially can store the splitter
    /// directly, without the need for a box. This in turn allows it to be used
    /// in constant and static context:
    ///
    /// ```
    /// use textwrap::{Options, HyphenSplitter};
    /// # use textwrap::{WordSplitter};
    /// # const width: usize = 80;
    /// # let expected =
    /// # Options {
    /// #    width: width,
    /// #    initial_indent: "",
    /// #    subsequent_indent: "",
    /// #    break_words: true,
    /// #    splitter: HyphenSplitter,
    /// # }
    /// # ;
    ///
    /// const FOO: Options<HyphenSplitter> = Options::with_splitter(width, HyphenSplitter);
    /// static BAR: Options<HyphenSplitter> = FOO;
    /// #
    /// # let actual = &BAR;
    /// # assert_eq!(actual.width, expected.width);
    /// # assert_eq!(actual.initial_indent, expected.initial_indent);
    /// # assert_eq!(actual.subsequent_indent, expected.subsequent_indent);
    /// # assert_eq!(actual.break_words, expected.break_words);
    /// # let expected_coerced: &Options<HyphenSplitter> = actual;
    /// ```
    ///
    pub const fn with_splitter(width: usize, splitter: S) -> Self {
        Options {
            width,
            initial_indent: "",
            subsequent_indent: "",
            break_words: true,
            splitter: splitter,
        }
    }
}

impl<'a, S: WordSplitter> Options<'a, S> {
    /// Change [`self.initial_indent`]. The initial indentation is
    /// used on the very first line of output.
    ///
    /// # Examples
    ///
    /// Classic paragraph indentation can be achieved by specifying an
    /// initial indentation and wrapping each paragraph by itself:
    ///
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// use textwrap::Options;
    ///
    /// let options = Options::new(15).initial_indent("    ");
    /// ```
    ///
    /// [`self.initial_indent`]: #structfield.initial_indent
    pub fn initial_indent(self, indent: &'a str) -> Self {
        Options {
            initial_indent: indent,
            ..self
        }
    }

    /// Change [`self.subsequent_indent`]. The subsequent indentation
    /// is used on lines following the first line of output.
    ///
    /// # Examples
    ///
    /// Combining initial and subsequent indentation lets you format a
    /// single paragraph as a bullet list:
    ///
    /// ```no_run
    /// # #![allow(unused_variables)]
    /// use textwrap::Options;
    ///
    /// let options = Options::new(15)
    ///     .initial_indent("* ")
    ///     .subsequent_indent("  ");
    /// ```
    ///
    /// [`self.subsequent_indent`]: #structfield.subsequent_indent
    pub fn subsequent_indent(self, indent: &'a str) -> Self {
        Options {
            subsequent_indent: indent,
            ..self
        }
    }

    /// Change [`self.break_words`]. This controls if words longer
    /// than `self.width` can be broken, or if they will be left
    /// sticking out into the right margin.
    ///
    /// [`self.break_words`]: #structfield.break_words
    pub fn break_words(self, setting: bool) -> Self {
        Options {
            break_words: setting,
            ..self
        }
    }

    /// Change [`self.splitter`]. The [`WordSplitter`] is used to fit
    /// part of a word into the current line when wrapping text.
    ///
    /// This function may return a different type than `Self`. That is the case when the given
    /// `splitter` is of a different type the the currently stored one in the `splitter` field.
    /// Take for example:
    ///
    /// ```
    /// use textwrap::{Options, HyphenSplitter, NoHyphenation};
    /// // The default type returned by `new` is `Options<HyphenSplitter>`
    /// let opt: Options<HyphenSplitter> = Options::new(80);
    /// // Setting a different splitter changes the type
    /// let opt: Options<NoHyphenation> = opt.splitter(NoHyphenation);
    /// ```
    ///
    /// [`self.splitter`]: #structfield.splitter
    /// [`WordSplitter`]: trait.WordSplitter.html
    pub fn splitter<T>(self, splitter: T) -> Options<'a, T> {
        Options {
            width: self.width,
            initial_indent: self.initial_indent,
            subsequent_indent: self.subsequent_indent,
            break_words: self.break_words,
            splitter: splitter,
        }
    }
}

/// Return the current terminal width. If the terminal width cannot be
/// determined (typically because the standard output is not connected
/// to a terminal), a default width of 80 characters will be used.
///
/// # Examples
///
/// Create an `Options` for wrapping at the current terminal width
/// with a two column margin to the left and the right:
///
/// ```no_run
/// # #![allow(unused_variables)]
/// use textwrap::{Options, NoHyphenation, termwidth};
///
/// let width = termwidth() - 4; // Two columns on each side.
/// let options = Options::new(width)
///     .splitter(NoHyphenation)
///     .initial_indent("  ")
///     .subsequent_indent("  ");
/// ```
///
/// **Note:** Only available when the `terminal_size` feature is
/// enabled.
#[cfg(feature = "terminal_size")]
pub fn termwidth() -> usize {
    terminal_size::terminal_size().map_or(80, |(terminal_size::Width(w), _)| w.into())
}

/// Fill a line of text at `width` characters.
///
/// The result is a `String`, complete with newlines between each
/// line. Use the [`wrap`] function if you need access to the
/// individual lines.
///
/// The easiest way to use this function is to pass an integer for
/// `options`:
///
/// ```
/// use textwrap::fill;
///
/// assert_eq!(fill("Memory safety without garbage collection.", 15),
///            "Memory safety\nwithout garbage\ncollection.");
/// ```
///
/// If you need to customize the wrapping, you can pass an [`Options`]
/// instead of an `usize`:
///
/// ```
/// use textwrap::{fill, Options};
///
/// let options = Options::new(15).initial_indent("- ").subsequent_indent("  ");
/// assert_eq!(fill("Memory safety without garbage collection.", &options),
///            "- Memory safety\n  without\n  garbage\n  collection.");
/// ```
///
/// [`wrap`]: fn.wrap.html
pub fn fill<'a, S: WordSplitter, T: Into<Options<'a, S>>>(text: &str, options: T) -> String {
    // This will avoid reallocation in simple cases (no
    // indentation, no hyphenation).
    let mut result = String::with_capacity(text.len());

    for (i, line) in wrap(text, options).iter().enumerate() {
        if i > 0 {
            result.push('\n');
        }
        result.push_str(&line);
    }

    result
}

/// Wrap a line of text at `width` characters.
///
/// The result is a vector of lines, each line is of type `Cow<'_,
/// str>`, which means that the line will borrow from the input `&str`
/// if possible. The lines do not have a trailing `'\n'`. Use the
/// [`fill`] function if you need a `String` instead.
///
/// The easiest way to use this function is to pass an integer for
/// `options`:
///
/// ```
/// use textwrap::wrap;
///
/// let lines = wrap("Memory safety without garbage collection.", 15);
/// assert_eq!(lines, &[
///     "Memory safety",
///     "without garbage",
///     "collection.",
/// ]);
/// ```
///
/// If you need to customize the wrapping, you can pass an [`Options`]
/// instead of an `usize`:
///
/// ```
/// use textwrap::{wrap, Options};
///
/// let options = Options::new(15).initial_indent("- ").subsequent_indent("  ");
/// let lines = wrap("Memory safety without garbage collection.", &options);
/// assert_eq!(lines, &[
///     "- Memory safety",
///     "  without",
///     "  garbage",
///     "  collection.",
/// ]);
/// ```
///
/// # Examples
///
/// The returned iterator yields lines of type `Cow<'_, str>`. If
/// possible, the wrapped lines will borrow borrow from the input
/// string. As an example, a hanging indentation, the first line can
/// borrow from the input, but the subsequent lines become owned
/// strings:
///
/// ```
/// use std::borrow::Cow::{Borrowed, Owned};
/// use textwrap::{wrap, Options};
///
/// let options = Options::new(15).subsequent_indent("....");
/// let lines = wrap("Wrapping text all day long.", &options);
/// let annotated = lines.iter().map(|line| match line {
///     Borrowed(text) => format!("[Borrowed] {}", text),
///     Owned(text)    => format!("[Owned]    {}", text),
/// }).collect::<Vec<_>>();
/// assert_eq!(annotated, &[
///     "[Borrowed] Wrapping text",
///     "[Owned]    ....all day",
///     "[Owned]    ....long.",
/// ]);
/// ```
///
/// [`fill`]: fn.fill.html
pub fn wrap<'a, S: WordSplitter, T: Into<Options<'a, S>>>(
    text: &str,
    options: T,
) -> Vec<Cow<'_, str>> {
    let options = options.into();

    let initial_width = options.width.saturating_sub(options.initial_indent.width());
    let subsequent_width = options
        .width
        .saturating_sub(options.subsequent_indent.width());

    let mut lines = Vec::new();
    for line in text.split('\n') {
        let words = core::find_words(line);
        let split_words = core::split_words(words, &options);
        let broken_words = if options.break_words {
            let mut broken_words = core::break_words(split_words, subsequent_width);
            if !options.initial_indent.is_empty() {
                // Without this, the first word will always go into
                // the first line. However, since we break words based
                // on the _second_ line width, it can be wrong to
                // unconditionally put the first word onto the first
                // line. An empty zero-width word fixed this.
                broken_words.insert(0, core::Word::from(""));
            }
            broken_words
        } else {
            split_words.collect::<Vec<_>>()
        };

        #[rustfmt::skip]
        let line_lengths = |i| if i == 0 { initial_width } else { subsequent_width };
        let wrapped_words = core::wrap_fragments(&broken_words, line_lengths);

        let mut idx = 0;
        for words in wrapped_words {
            let last_word = match words.last() {
                None => {
                    lines.push(Cow::from(""));
                    continue;
                }
                Some(word) => word,
            };

            // We assume here that all words are contiguous in `line`.
            // That is, the sum of their lengths should add up to the
            // lenght of `line`.
            let len = words
                .iter()
                .map(|word| word.len() + word.whitespace.len())
                .sum::<usize>()
                - last_word.whitespace.len();

            // The result is owned if we have indentation, otherwise
            // we can simply borrow an empty string.
            let mut result = if lines.is_empty() && !options.initial_indent.is_empty() {
                Cow::Owned(options.initial_indent.to_owned())
            } else if !lines.is_empty() && !options.subsequent_indent.is_empty() {
                Cow::Owned(options.subsequent_indent.to_owned())
            } else {
                // We can use an empty string here since string
                // concatenation for `Cow` preserves a borrowed value
                // when either side is empty.
                Cow::from("")
            };

            result += &line[idx..idx + len];

            if !last_word.penalty.is_empty() {
                result.to_mut().push_str(&last_word.penalty);
            }

            lines.push(result);

            // Advance by the length of `result`, plus the length of
            // `last_word.whitespace` -- even if we had a penalty, we
            // need to skip over the whitespace.
            idx += len + last_word.whitespace.len();
        }
    }

    lines
}

/// Fill `text` in-place without reallocating the input string.
///
/// This function works by modifying the input string: some `' '`
/// characters will be replaced by `'\n'` characters. The rest of the
/// text remains untouched.
///
/// Since we can only replace existing whitespace in the input with
/// `'\n'`, we cannot do hyphenation nor can we split words longer
/// than the line width. Indentation is also rules out. In other
/// words, `fill_inplace(width)` behaves as if you had called `fill`
/// with these options:
///
/// ```
/// # use textwrap::{Options, NoHyphenation};
/// # let width = 80;
/// Options {
///    width: width,
///    initial_indent: "",
///    subsequent_indent: "",
///    break_words: false,
///    splitter: NoHyphenation,
/// };
/// ```
///
/// Finally, unlike `fill`, `fill_inplace` can leave trailing
/// whitespace on lines. This is because we wrap by inserting a `'\n'`
/// at the final whitespace in the input string:
///
/// ```
/// let mut text = String::from("Hello   World!");
/// textwrap::fill_inplace(&mut text, 10);
/// assert_eq!(text, "Hello  \nWorld!");
/// ```
///
/// If we didn't do this, the word `World!` would end up being
/// indented. You can avoid this if you make sure that your input text
/// has no double spaces.
///
/// # Performance
///
/// In benchmarks, `fill_inplace` is about twice as fast as `fill`.
/// Please see the [`linear`
/// benchmark](https://github.com/mgeisler/textwrap/blob/master/benches/linear.rs)
/// for details.
pub fn fill_inplace(text: &mut String, width: usize) {
    let mut indices = Vec::new();

    let mut offset = 0;
    for line in text.split('\n') {
        let words = core::find_words(line).collect::<Vec<_>>();
        let wrapped_words = core::wrap_fragments(&words, |_| width);

        let mut line_offset = offset;
        for words in &wrapped_words[..wrapped_words.len() - 1] {
            let line_len = words
                .iter()
                .map(|word| word.len() + word.whitespace.len())
                .sum::<usize>();

            line_offset += line_len;
            // We've advanced past all ' ' characters -- want to move
            // one ' ' backwards and insert our '\n' there.
            indices.push(line_offset - 1);
        }

        // Advance past entire line, plus the '\n' which was removed
        // by the split call above.
        offset += line.len() + 1;
    }

    let mut bytes = std::mem::take(text).into_bytes();
    for idx in indices {
        bytes[idx] = b'\n';
    }
    *text = String::from_utf8(bytes).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "hyphenation")]
    use hyphenation::{Language, Load, Standard};

    #[test]
    fn options_agree_with_usize() {
        let opt_usize = Options::from(42_usize);
        let opt_options = Options::new(42);

        assert_eq!(opt_usize.width, opt_options.width);
        assert_eq!(opt_usize.initial_indent, opt_options.initial_indent);
        assert_eq!(opt_usize.subsequent_indent, opt_options.subsequent_indent);
        assert_eq!(opt_usize.break_words, opt_options.break_words);
        assert_eq!(
            opt_usize.splitter.split_points("hello-world"),
            opt_options.splitter.split_points("hello-world")
        );
    }

    #[test]
    fn no_wrap() {
        assert_eq!(wrap("foo", 10), vec!["foo"]);
    }

    #[test]
    fn wrap_simple() {
        assert_eq!(wrap("foo bar baz", 5), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn multiple_words_on_first_line() {
        assert_eq!(wrap("foo bar baz", 10), vec!["foo bar", "baz"]);
    }

    #[test]
    fn long_word() {
        assert_eq!(wrap("foo", 0), vec!["f", "o", "o"]);
    }

    #[test]
    fn long_words() {
        assert_eq!(wrap("foo bar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }

    #[test]
    fn max_width() {
        assert_eq!(wrap("foo bar", usize::max_value()), vec!["foo bar"]);
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(wrap("  foo bar", 6), vec!["  foo", "bar"]);
    }

    #[test]
    fn trailing_whitespace() {
        // Whitespace is only significant inside a line. After a line
        // gets too long and is broken, the first word starts in
        // column zero and is not indented.
        assert_eq!(wrap("foo     bar     baz", 5), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn issue_99() {
        // We did not reset the in_whitespace flag correctly and did
        // not handle single-character words after a line break.
        assert_eq!(
            wrap("aaabbbccc x yyyzzzwww", 9),
            vec!["aaabbbccc", "x", "yyyzzzwww"]
        );
    }

    #[test]
    fn issue_129() {
        // The dash is an em-dash which takes up four bytes. We used
        // to panic since we tried to index into the character.
        assert_eq!(wrap("x – x", 1), vec!["x", "–", "x"]);
    }

    #[test]
    fn wide_character_handling() {
        assert_eq!(wrap("Hello, World!", 15), vec!["Hello, World!"]);
        assert_eq!(
            wrap("Ｈｅｌｌｏ, Ｗｏｒｌｄ!", 15),
            vec!["Ｈｅｌｌｏ,", "Ｗｏｒｌｄ!"]
        );
    }

    #[test]
    fn empty_line_is_indented() {
        // Previously, indentation was not applied to empty lines.
        // However, this is somewhat inconsistent and undesirable if
        // the indentation is something like a border ("| ") which you
        // want to apply to all lines, empty or not.
        let options = Options::new(10).initial_indent("!!!");
        assert_eq!(fill("", &options), "!!!");
    }

    #[test]
    fn indent_single_line() {
        let options = Options::new(10).initial_indent(">>>"); // No trailing space
        assert_eq!(fill("foo", &options), ">>>foo");
    }

    #[test]
    fn indent_first() {
        let options = Options::new(10).initial_indent("👉👉");
        assert_eq!(
            wrap("x x x x x x x x x x x x x", &options),
            vec!["👉👉x x x", "x x x x x", "x x x x x"]
        );
    }

    #[test]
    fn indent_multiple_lines() {
        let options = Options::new(6).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(
            wrap("foo bar baz", &options),
            vec!["* foo", "  bar", "  baz"]
        );
    }

    #[test]
    fn indent_break_words() {
        let options = Options::new(5).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(wrap("foobarbaz", &options), vec!["* foo", "  bar", "  baz"]);
    }

    #[test]
    fn initial_indent_break_words() {
        // This is a corner-case showing how the long word is broken
        // according to the width of the subsequent lines. The first
        // fragment of the word no longer fits on the first line,
        // which ends up being pure indentation.
        let options = Options::new(5).initial_indent("-->");
        assert_eq!(wrap("foobarbaz", &options), vec!["-->", "fooba", "rbaz"]);
    }

    #[test]
    fn hyphens() {
        assert_eq!(wrap("foo-bar", 5), vec!["foo-", "bar"]);
    }

    #[test]
    fn trailing_hyphen() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foobar-", &options), vec!["foobar-"]);
    }

    #[test]
    fn multiple_hyphens() {
        assert_eq!(wrap("foo-bar-baz", 5), vec!["foo-", "bar-", "baz"]);
    }

    #[test]
    fn hyphens_flag() {
        let options = Options::new(5).break_words(false);
        assert_eq!(
            wrap("The --foo-bar flag.", &options),
            vec!["The", "--foo-", "bar", "flag."]
        );
    }

    #[test]
    fn repeated_hyphens() {
        let options = Options::new(4).break_words(false);
        assert_eq!(wrap("foo--bar", &options), vec!["foo--bar"]);
    }

    #[test]
    fn hyphens_alphanumeric() {
        assert_eq!(wrap("Na2-CH4", 5), vec!["Na2-", "CH4"]);
    }

    #[test]
    fn hyphens_non_alphanumeric() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foo(-)bar", &options), vec!["foo(-)bar"]);
    }

    #[test]
    fn multiple_splits() {
        assert_eq!(wrap("foo-bar-baz", 9), vec!["foo-bar-", "baz"]);
    }

    #[test]
    fn forced_split() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foobar-baz", &options), vec!["foobar-", "baz"]);
    }

    #[test]
    fn multiple_unbroken_words_issue_193() {
        let options = Options::new(3).break_words(false);
        assert_eq!(
            wrap("small large tiny", &options),
            vec!["small", "large", "tiny"]
        );
        assert_eq!(
            wrap("small  large   tiny", &options),
            vec!["small", "large", "tiny"]
        );
    }

    #[test]
    fn very_narrow_lines_issue_193() {
        let options = Options::new(1).break_words(false);
        assert_eq!(wrap("fooo x y", &options), vec!["fooo", "x", "y"]);
        assert_eq!(wrap("fooo   x     y", &options), vec!["fooo", "x", "y"]);
    }

    #[test]
    fn simple_hyphens_static() {
        let options = Options::new(8).splitter(HyphenSplitter);
        assert_eq!(wrap("foo bar-baz", &options), vec!["foo bar-", "baz"]);
    }

    #[test]
    fn simple_hyphens_dynamic() {
        let options: Options = Options::new(8).splitter(Box::new(HyphenSplitter));
        assert_eq!(wrap("foo bar-baz", &options), vec!["foo bar-", "baz"]);
    }

    #[test]
    fn no_hyphenation_static() {
        let options = Options::new(8).splitter(NoHyphenation);
        assert_eq!(wrap("foo bar-baz", &options), vec!["foo", "bar-baz"]);
    }

    #[test]
    fn no_hyphenation_dynamic() {
        let options: Options = Options::new(8).splitter(Box::new(NoHyphenation));
        assert_eq!(wrap("foo bar-baz", &options), vec!["foo", "bar-baz"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_double_hyphenation_static() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            wrap("Internationalization", &options),
            vec!["Internatio", "nalization"]
        );

        let options = Options::new(10).splitter(dictionary);
        assert_eq!(
            wrap("Internationalization", &options),
            vec!["Interna-", "tionaliza-", "tion"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_double_hyphenation_dynamic() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let mut options: Options = Options::new(10).splitter(Box::new(HyphenSplitter));
        assert_eq!(
            wrap("Internationalization", &options),
            vec!["Internatio", "nalization"]
        );

        options = Options::new(10).splitter(Box::new(dictionary));
        assert_eq!(
            wrap("Internationalization", &options),
            vec!["Interna-", "tionaliza-", "tion"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_issue_158() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            wrap("participation is the key to success", &options),
            vec!["participat", "ion is the", "key to", "success"]
        );

        let options = Options::new(10).splitter(dictionary);
        assert_eq!(
            wrap("participation is the key to success", &options),
            vec!["participa-", "tion is", "the key to", "success"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn split_len_hyphenation() {
        // Test that hyphenation takes the width of the wihtespace
        // into account.
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(15).splitter(dictionary);
        assert_eq!(
            wrap("garbage   collection", &options),
            vec!["garbage   col-", "lection"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn borrowed_lines() {
        // Lines that end with an extra hyphen are owned, the final
        // line is borrowed.
        use std::borrow::Cow::{Borrowed, Owned};
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10).splitter(dictionary);
        let lines = wrap("Internationalization", &options);
        if let Borrowed(s) = lines[0] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Borrowed(s) = lines[1] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Owned(ref s) = lines[2] {
            assert!(false, "should not have been owned: {:?}", s);
        }
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_with_hyphen() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(8).break_words(false);
        assert_eq!(
            wrap("over-caffinated", &options),
            vec!["over-", "caffinated"]
        );

        let options = options.splitter(dictionary);
        assert_eq!(
            wrap("over-caffinated", &options),
            vec!["over-", "caffi-", "nated"]
        );
    }

    #[test]
    fn break_words() {
        assert_eq!(wrap("foobarbaz", 3), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn break_words_wide_characters() {
        assert_eq!(wrap("Ｈｅｌｌｏ", 5), vec!["Ｈｅ", "ｌｌ", "ｏ"]);
    }

    #[test]
    fn break_words_zero_width() {
        assert_eq!(wrap("foobar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }

    #[test]
    fn break_long_first_word() {
        assert_eq!(wrap("testx y", 4), vec!["test", "x y"]);
    }

    #[test]
    fn break_words_line_breaks() {
        assert_eq!(fill("ab\ncdefghijkl", 5), "ab\ncdefg\nhijkl");
        assert_eq!(fill("abcdefgh\nijkl", 5), "abcde\nfgh\nijkl");
    }

    #[test]
    fn break_words_empty_lines() {
        assert_eq!(
            fill("foo\nbar", &Options::new(2).break_words(false)),
            "foo\nbar"
        );
    }

    #[test]
    fn preserve_line_breaks() {
        assert_eq!(fill("", 80), "");
        assert_eq!(fill("\n", 80), "\n");
        assert_eq!(fill("\n\n\n", 80), "\n\n\n");
        assert_eq!(fill("test\n", 80), "test\n");
        assert_eq!(fill("test\n\na\n\n", 80), "test\n\na\n\n");
        assert_eq!(fill("1 3 5 7\n1 3 5 7", 7), "1 3 5 7\n1 3 5 7");
        assert_eq!(fill("1 3 5 7\n1 3 5 7", 5), "1 3 5\n7\n1 3 5\n7");
    }

    #[test]
    fn non_breaking_space() {
        let options = Options::new(5).break_words(false);
        assert_eq!(fill("foo bar baz", &options), "foo bar baz");
    }

    #[test]
    fn non_breaking_hyphen() {
        let options = Options::new(5).break_words(false);
        assert_eq!(fill("foo‑bar‑baz", &options), "foo‑bar‑baz");
    }

    #[test]
    fn fill_simple() {
        assert_eq!(fill("foo bar baz", 10), "foo bar\nbaz");
    }

    #[test]
    fn fill_colored_text() {
        // The words are much longer than 6 bytes, but they remain
        // intact after filling the text.
        let green_hello = "\u{1b}[0m\u{1b}[32mHello\u{1b}[0m";
        let blue_world = "\u{1b}[0m\u{1b}[34mWorld!\u{1b}[0m";
        assert_eq!(
            fill(&(String::from(green_hello) + " " + &blue_world), 6),
            String::from(green_hello) + "\n" + &blue_world
        );
    }

    #[test]
    fn cloning_works() {
        static OPT: Options<HyphenSplitter> = Options::with_splitter(80, HyphenSplitter);
        #[allow(clippy::clone_on_copy)]
        let opt = OPT.clone();
        assert_eq!(opt.width, 80);
    }

    #[test]
    fn fill_inplace_empty() {
        let mut text = String::from("");
        fill_inplace(&mut text, 80);
        assert_eq!(text, "");
    }

    #[test]
    fn fill_inplace_simple() {
        let mut text = String::from("foo bar baz");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo bar\nbaz");
    }

    #[test]
    fn fill_inplace_multiple_lines() {
        let mut text = String::from("Some text to wrap over multiple lines");
        fill_inplace(&mut text, 12);
        assert_eq!(text, "Some text to\nwrap over\nmultiple\nlines");
    }

    #[test]
    fn fill_inplace_long_word() {
        let mut text = String::from("Internationalization is hard");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "Internationalization\nis hard");
    }

    #[test]
    fn fill_inplace_no_hyphen_splitting() {
        let mut text = String::from("A well-chosen example");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "A\nwell-chosen\nexample");
    }

    #[test]
    fn fill_inplace_newlines() {
        let mut text = String::from("foo bar\n\nbaz\n\n\n");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo bar\n\nbaz\n\n\n");
    }

    #[test]
    fn fill_inplace_newlines_reset_line_width() {
        let mut text = String::from("1 3 5\n1 3 5 7 9\n1 3 5 7 9 1 3");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "1 3 5\n1 3 5 7 9\n1 3 5 7 9\n1 3");
    }

    #[test]
    fn fill_inplace_leading_whitespace() {
        let mut text = String::from("  foo bar baz");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "  foo bar\nbaz");
    }

    #[test]
    fn fill_inplace_trailing_whitespace() {
        let mut text = String::from("foo bar baz  ");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo bar\nbaz  ");
    }

    #[test]
    fn fill_inplace_interior_whitespace() {
        // To avoid an unwanted indentation of "baz", it is important
        // to replace the final ' ' with '\n'.
        let mut text = String::from("foo  bar    baz");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo  bar   \nbaz");
    }
}
