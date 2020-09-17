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
//!     let options = Options::new(18).splitter(Box::new(dictionary));
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
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::redundant_field_names)]
#![feature(split_inclusive)]

use std::borrow::Cow;
use std::str::CharIndices;

use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

/// A non-breaking space.
const NBSP: char = '\u{a0}';

/// The CSI or "Control Sequence Introducer" introduces an ANSI escape
/// sequence. This is typically used for colored text and will be
/// ignored when computing the text width.
const CSI: (char, char) = ('\u{1b}', '[');
/// The final bytes of an ANSI escape sequence must be in this range.
const ANSI_FINAL_BYTE: std::ops::RangeInclusive<char> = '\x40'..='\x7e';

mod indentation;
pub use crate::indentation::dedent;
pub use crate::indentation::indent;

mod splitting;
pub use crate::splitting::{HyphenSplitter, NoHyphenation, WordSplitter};

/// Options for wrapping and filling text. Used with the [`wrap`] and
/// [`fill`] functions.
///
/// [`wrap`]: fn.wrap.html
/// [`fill`]: fn.fill.html
pub trait WrapOptions {
    /// The width in columns at which the text will be wrapped.
    fn width(&self) -> usize;
    /// Indentation used for the first line of output.
    fn initial_indent(&self) -> &str;
    /// Indentation used for subsequent lines of output.
    fn subsequent_indent(&self) -> &str;
    /// Allow long words to be broken if they cannot fit on a line.
    /// When set to `false`, some lines may be longer than `width`.
    fn break_words(&self) -> bool;
    /// Split word as with `WordSplitter::split`.
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)>;
}

/// Holds settings for wrapping and filling text.
#[derive(Debug)]
pub struct Options<'a> {
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
    pub splitter: Box<dyn WordSplitter>,
}

/// Allows using an `Options` with [`wrap`] and [`fill`]:
///
/// ```
/// use textwrap::{fill, Options};
///
/// let options = Options::new(15).initial_indent("> ");
/// assert_eq!(fill("Wrapping with options!", &options),
///            "> Wrapping with\noptions!");
/// ```
///
/// The integer specifes the wrapping width. This is equivalent to
/// passing `Options::new(15)`.
///
/// [`wrap`]: fn.wrap.html
/// [`fill`]: fn.fill.html
impl WrapOptions for &Options<'_> {
    #[inline]
    fn width(&self) -> usize {
        self.width
    }
    #[inline]
    fn initial_indent(&self) -> &str {
        self.initial_indent
    }
    #[inline]
    fn subsequent_indent(&self) -> &str {
        self.subsequent_indent
    }
    #[inline]
    fn break_words(&self) -> bool {
        self.break_words
    }
    #[inline]
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)> {
        self.splitter.split(word)
    }
}

/// Allows using an `usize` directly as options for [`wrap`] and
/// [`fill`]:
///
/// ```
/// use textwrap::fill;
///
/// assert_eq!(fill("Quick and easy wrapping!", 15),
///            "Quick and easy\nwrapping!");
/// ```
///
/// The integer specifes the wrapping width. This is equivalent to
/// passing `Options::new(15)`.
///
/// [`wrap`]: fn.wrap.html
/// [`fill`]: fn.fill.html
impl WrapOptions for usize {
    #[inline]
    fn width(&self) -> usize {
        *self
    }
    #[inline]
    fn initial_indent(&self) -> &str {
        ""
    }
    #[inline]
    fn subsequent_indent(&self) -> &str {
        ""
    }
    #[inline]
    fn break_words(&self) -> bool {
        true
    }
    #[inline]
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)> {
        HyphenSplitter.split(word)
    }
}

impl<'a> Options<'a> {
    /// Creates a new `Options` with the specified width. Equivalent
    /// to
    ///
    /// ```
    /// # use textwrap::{Options, HyphenSplitter};
    /// # let width = 80;
    /// # let actual = Options::new(width);
    /// # let expected =
    /// Options {
    ///     width: width,
    ///     initial_indent: "",
    ///     subsequent_indent: "",
    ///     break_words: true,
    ///     splitter: Box::new(HyphenSplitter),
    /// }
    /// # ;
    /// # assert_eq!(actual.width, expected.width);
    /// # assert_eq!(actual.initial_indent, expected.initial_indent);
    /// # assert_eq!(actual.subsequent_indent, expected.subsequent_indent);
    /// # assert_eq!(actual.break_words, expected.break_words);
    /// ```
    pub fn new(width: usize) -> Options<'static> {
        Options {
            width: width,
            initial_indent: "",
            subsequent_indent: "",
            break_words: true,
            splitter: Box::new(HyphenSplitter),
        }
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
    pub fn with_termwidth() -> Options<'static> {
        Options::new(termwidth())
    }

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
    pub fn initial_indent(self, indent: &'a str) -> Options<'a> {
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
    pub fn subsequent_indent(self, indent: &'a str) -> Options<'a> {
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
    pub fn break_words(self, setting: bool) -> Options<'a> {
        Options {
            break_words: setting,
            ..self
        }
    }

    /// Change [`self.splitter`]. The [`WordSplitter`] is used to fit
    /// part of a word into the current line when wrapping text.
    ///
    /// [`self.splitter`]: #structfield.splitter
    /// [`WordSplitter`]: trait.WordSplitter.html
    pub fn splitter(self, splitter: Box<dyn WordSplitter>) -> Options<'a> {
        Options {
            splitter: splitter,
            ..self
        }
    }
}

/// Like `char::is_whitespace`, but non-breaking spaces don't count.
#[inline]
fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() && ch != NBSP
}

#[derive(Debug)]
struct WrapIter<'input, T: WrapOptions> {
    options: T,

    // String to wrap.
    source: &'input str,
    // CharIndices iterator over self.source.
    char_indices: CharIndices<'input>,
    // Byte index where the current line starts.
    start: usize,
    // Byte index of the last place where the string can be split.
    split: usize,
    // Size in bytes of the character at self.source[self.split].
    split_len: usize,
    // Width of self.source[self.start..idx].
    line_width: usize,
    // Width of self.source[self.start..self.split].
    line_width_at_split: usize,
    // Tracking runs of whitespace characters.
    in_whitespace: bool,
    // Has iterator finished producing elements?
    finished: bool,
}

impl<T: WrapOptions> WrapIter<'_, T> {
    fn new(options: T, s: &str) -> WrapIter<'_, T> {
        let initial_indent_width = options.initial_indent().width();

        WrapIter {
            options: options,
            source: s,
            char_indices: s.char_indices(),
            start: 0,
            split: 0,
            split_len: 0,
            line_width: initial_indent_width,
            line_width_at_split: initial_indent_width,
            in_whitespace: false,
            finished: false,
        }
    }

    fn create_result_line(&self) -> Cow<'static, str> {
        let indent = if self.start == 0 {
            self.options.initial_indent()
        } else {
            self.options.subsequent_indent()
        };
        if indent.is_empty() {
            Cow::Borrowed("") // return Cow<'static, str>
        } else {
            // This removes the link between the lifetime of the
            // indentation and the input string. The non-empty
            // indentation will force us to create an owned `String`
            // in any case.
            Cow::Owned(String::from(indent))
        }
    }
}

impl<'input, T: WrapOptions> Iterator for WrapIter<'input, T> {
    type Item = Cow<'input, str>;

    fn next(&mut self) -> Option<Cow<'input, str>> {
        if self.finished {
            return None;
        }

        while let Some((idx, ch)) = self.char_indices.next() {
            if ch == CSI.0 && self.char_indices.next().map(|(_, ch)| ch) == Some(CSI.1) {
                // We have found the start of an ANSI escape code,
                // typically used for colored text. We ignore all
                // characters until we find a "final byte" in the
                // range 0x40–0x7E.
                while let Some((_, ch)) = self.char_indices.next() {
                    if ANSI_FINAL_BYTE.contains(&ch) {
                        break;
                    }
                }
                // Done with the escape sequence, we continue with
                // next character in the outer loop.
                continue;
            }

            let char_width = ch.width().unwrap_or(0);
            let char_len = ch.len_utf8();
            if ch == '\n' {
                self.split = idx;
                self.split_len = char_len;
                self.line_width_at_split = self.line_width;
                self.in_whitespace = false;

                // If this is not the final line, return the current line. Otherwise,
                // we will return the line with its line break after exiting the loop
                if self.split + self.split_len < self.source.len() {
                    let mut line = self.create_result_line();
                    line += &self.source[self.start..self.split];

                    self.start = self.split + self.split_len;
                    self.line_width = self.options.subsequent_indent().width();

                    return Some(line);
                }
            } else if is_whitespace(ch) {
                // Extend the previous split or create a new one.
                if self.in_whitespace {
                    self.split_len += char_len;
                } else {
                    self.split = idx;
                    self.split_len = char_len;
                }
                self.line_width_at_split = self.line_width + char_width;
                self.in_whitespace = true;
            } else if self.line_width + char_width > self.options.width() {
                // There is no room for this character on the current
                // line. Try to split the final word.
                self.in_whitespace = false;
                let remaining_text = &self.source[self.split + self.split_len..];
                let final_word = match remaining_text.find(is_whitespace) {
                    Some(i) => &remaining_text[..i],
                    None => remaining_text,
                };

                let mut hyphen = "";
                let splits = self.options.split(final_word);
                for &(head, hyp, _) in splits.iter().rev() {
                    if self.line_width_at_split + head.width() + hyp.width() <= self.options.width()
                    {
                        // We can fit head into the current line.
                        // Advance the split point by the width of the
                        // whitespace and the head length.
                        self.split += self.split_len + head.len();
                        // The new `split_len` is equal to the stretch
                        // of whitespace following the split.
                        self.split_len = remaining_text[head.len()..]
                            .char_indices()
                            .skip_while(|(_, ch)| is_whitespace(*ch))
                            .next()
                            .map_or(0, |(idx, _)| idx);
                        self.line_width_at_split += head.width() + hyp.width();
                        hyphen = hyp;
                        break;
                    }
                }

                if self.start >= self.split {
                    // The word is too big to fit on a single line.
                    if self.options.break_words() {
                        // Break work at current index.
                        self.split = idx;
                        self.split_len = 0;
                        self.line_width_at_split = self.line_width;
                    } else {
                        // Add smallest split.
                        self.split += self.split_len + splits[0].0.len();
                        // The new `split_len` is equal to the stretch
                        // of whitespace following the smallest split.
                        self.split_len = remaining_text[splits[0].0.len()..]
                            .char_indices()
                            .skip_while(|(_, ch)| is_whitespace(*ch))
                            .next()
                            .map_or(0, |(idx, _)| idx);
                        self.line_width_at_split = self.line_width;
                    }
                }

                if self.start < self.split {
                    let mut line = self.create_result_line();
                    line += &self.source[self.start..self.split];
                    line += hyphen;

                    self.start = self.split + self.split_len;
                    self.line_width += self.options.subsequent_indent().width();
                    self.line_width -= self.line_width_at_split;
                    self.line_width += char_width;
                    self.line_width_at_split = self.options.subsequent_indent().width();

                    return Some(line);
                }
            } else {
                self.in_whitespace = false;
            }
            self.line_width += char_width;
        }

        self.finished = true;

        // Add final line.
        if self.start < self.source.len() {
            let mut line = self.create_result_line();
            line += &self.source[self.start..];
            return Some(line);
        }

        None
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
///     .splitter(Box::new(NoHyphenation))
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
pub fn fill<T: WrapOptions>(text: &str, options: T) -> String {
    // This will avoid reallocation in simple cases (no
    // indentation, no hyphenation).
    let mut result = String::with_capacity(text.len());

    for (i, line) in wrap(text, options).enumerate() {
        if i > 0 {
            result.push('\n');
        }
        result.push_str(&line);
    }

    result
}

/// The easiest way to use this function is to pass an integer for
/// `options`:
/// let lines = wrap("Memory safety without garbage collection.", 15);
/// assert_eq!(lines.collect::<Vec<_>>(), &[
///     "Memory safety",
///     "without garbage",
///     "collection.",
/// ]);
/// If you need to customize the wrapping, you can pass an [`Options`]
/// instead of an `usize`:
///
/// ```
/// use textwrap::{wrap, Options};
///
/// let options = Options::new(15).initial_indent("- ").subsequent_indent("  ");
/// let lines = wrap("Memory safety without garbage collection.", &options);
/// assert_eq!(lines.collect::<Vec<_>>(), &[
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
/// let annotated = lines.map(|line| match line {
///     Borrowed(text) => format!("[Borrowed] {}", text),
///     Owned(text)    => format!("[Owned]    {}", text),
/// }).collect::<Vec<_>>();
/// assert_eq!(annotated, &[
///     "[Borrowed] Wrapping text",
///     "[Owned]    ....all day",
///     "[Owned]    ....long.",
/// ]);
/// ```
pub fn wrap<T: WrapOptions>(text: &str, options: T) -> impl Iterator<Item = Cow<'_, str>> {
    WrapIter::new(options, text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "hyphenation")]
    use hyphenation::{Language, Load, Standard};

    macro_rules! assert_iter_eq {
        ($left:expr, $right:expr) => {
            assert_eq!($left.collect::<Vec<_>>(), $right);
        };
    }

    #[test]
    fn options_agree_with_usize() {
        let opt_usize: &dyn WrapOptions = &42;
        let opt_options: &dyn WrapOptions = &&Options::new(42);

        assert_eq!(opt_usize.width(), opt_options.width());
        assert_eq!(opt_usize.initial_indent(), opt_options.initial_indent());
        assert_eq!(
            opt_usize.subsequent_indent(),
            opt_options.subsequent_indent()
        );
        assert_eq!(opt_usize.break_words(), opt_options.break_words());
        assert_eq!(
            opt_usize.split("hello-world"),
            opt_options.split("hello-world")
        );
    }

    #[test]
    fn no_wrap() {
        assert_iter_eq!(wrap("foo", 10), vec!["foo"]);
    }

    #[test]
    fn simple() {
        assert_iter_eq!(wrap("foo bar baz", 5), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn multi_word_on_line() {
        assert_iter_eq!(wrap("foo bar baz", 10), vec!["foo bar", "baz"]);
    }

    #[test]
    fn long_word() {
        assert_iter_eq!(wrap("foo", 0), vec!["f", "o", "o"]);
    }

    #[test]
    fn long_words() {
        assert_iter_eq!(wrap("foo bar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }

    #[test]
    fn max_width() {
        assert_iter_eq!(wrap("foo bar", usize::max_value()), vec!["foo bar"]);
    }

    #[test]
    fn leading_whitespace() {
        assert_iter_eq!(wrap("  foo bar", 6), vec!["  foo", "bar"]);
    }

    #[test]
    fn trailing_whitespace() {
        assert_iter_eq!(wrap("foo bar  ", 6), vec!["foo", "bar  "]);
    }

    #[test]
    fn interior_whitespace() {
        assert_iter_eq!(wrap("foo:   bar baz", 10), vec!["foo:   bar", "baz"]);
    }

    #[test]
    fn extra_whitespace_start_of_line() {
        // Whitespace is only significant inside a line. After a line
        // gets too long and is broken, the first word starts in
        // column zero and is not indented. The line before might end
        // up with trailing whitespace.
        assert_iter_eq!(wrap("foo               bar", 5), vec!["foo", "bar"]);
    }

    #[test]
    fn issue_99() {
        // We did not reset the in_whitespace flag correctly and did
        // not handle single-character words after a line break.
        assert_iter_eq!(
            wrap("aaabbbccc x yyyzzzwww", 9),
            vec!["aaabbbccc", "x", "yyyzzzwww"]
        );
    }

    #[test]
    fn issue_129() {
        // The dash is an em-dash which takes up four bytes. We used
        // to panic since we tried to index into the character.
        assert_iter_eq!(wrap("x – x", 1), vec!["x", "–", "x"]);
    }

    #[test]
    fn wide_character_handling() {
        assert_iter_eq!(wrap("Hello, World!", 15), vec!["Hello, World!"]);
        assert_iter_eq!(
            wrap("Ｈｅｌｌｏ, Ｗｏｒｌｄ!", 15),
            vec!["Ｈｅｌｌｏ,", "Ｗｏｒｌｄ!"]
        );
    }

    #[test]
    fn empty_input_not_indented() {
        let options = Options::new(10).initial_indent("!!!");
        assert_eq!(fill("", &options), "");
    }

    #[test]
    fn indent_single_line() {
        let options = Options::new(10).initial_indent(">>>"); // No trailing space
        assert_eq!(fill("foo", &options), ">>>foo");
    }

    #[test]
    fn indent_multiple_lines() {
        let options = Options::new(6).initial_indent("* ").subsequent_indent("  ");
        assert_iter_eq!(
            wrap("foo bar baz", &options),
            vec!["* foo", "  bar", "  baz"]
        );
    }

    #[test]
    fn indent_break_words() {
        let options = Options::new(5).initial_indent("* ").subsequent_indent("  ");
        assert_iter_eq!(wrap("foobarbaz", &options), vec!["* foo", "  bar", "  baz"]);
    }

    #[test]
    fn hyphens() {
        assert_iter_eq!(wrap("foo-bar", 5), vec!["foo-", "bar"]);
    }

    #[test]
    fn trailing_hyphen() {
        let options = Options::new(5).break_words(false);
        assert_iter_eq!(wrap("foobar-", &options), vec!["foobar-"]);
    }

    #[test]
    fn multiple_hyphens() {
        assert_iter_eq!(wrap("foo-bar-baz", 5), vec!["foo-", "bar-", "baz"]);
    }

    #[test]
    fn hyphens_flag() {
        let options = Options::new(5).break_words(false);
        assert_iter_eq!(
            wrap("The --foo-bar flag.", &options),
            vec!["The", "--foo-", "bar", "flag."]
        );
    }

    #[test]
    fn repeated_hyphens() {
        let options = Options::new(4).break_words(false);
        assert_iter_eq!(wrap("foo--bar", &options), vec!["foo--bar"]);
    }

    #[test]
    fn hyphens_alphanumeric() {
        assert_iter_eq!(wrap("Na2-CH4", 5), vec!["Na2-", "CH4"]);
    }

    #[test]
    fn hyphens_non_alphanumeric() {
        let options = Options::new(5).break_words(false);
        assert_iter_eq!(wrap("foo(-)bar", &options), vec!["foo(-)bar"]);
    }

    #[test]
    fn multiple_splits() {
        assert_iter_eq!(wrap("foo-bar-baz", 9), vec!["foo-bar-", "baz"]);
    }

    #[test]
    fn forced_split() {
        let options = Options::new(5).break_words(false);
        assert_iter_eq!(wrap("foobar-baz", &options), vec!["foobar-", "baz"]);
    }

    #[test]
    fn multiple_unbroken_words_issue_193() {
        let options = Options::new(3).break_words(false);
        assert_iter_eq!(
            wrap("small large tiny", &options),
            vec!["small", "large", "tiny"]
        );
        assert_iter_eq!(
            wrap("small  large   tiny", &options),
            vec!["small", "large", "tiny"]
        );
    }

    #[test]
    fn very_narrow_lines_issue_193() {
        let options = Options::new(1).break_words(false);
        assert_iter_eq!(wrap("fooo x y", &options), vec!["fooo", "x", "y"]);
        assert_iter_eq!(wrap("fooo   x     y", &options), vec!["fooo", "x", "y"]);
    }

    #[test]
    fn no_hyphenation() {
        let options = Options::new(8).splitter(Box::new(NoHyphenation));
        assert_iter_eq!(wrap("foo bar-baz", &options), vec!["foo", "bar-baz"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_iter_eq!(
            wrap("Internationalization", &options),
            vec!["Internatio", "nalization"]
        );

        let options = Options::new(10).splitter(Box::new(dictionary));
        assert_iter_eq!(
            wrap("Internationalization", &options),
            vec!["Interna-", "tionaliza-", "tion"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_issue_158() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_iter_eq!(
            wrap("participation is the key to success", &options),
            vec!["participat", "ion is the", "key to", "success"]
        );

        let options = Options::new(10).splitter(Box::new(dictionary));
        assert_iter_eq!(
            wrap("participation is the key to success", &options),
            vec!["participa-", "tion is the", "key to", "success"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn split_len_hyphenation() {
        // Test that hyphenation takes the width of the wihtespace
        // into account.
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(15).splitter(Box::new(dictionary));
        assert_iter_eq!(
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
        let options = Options::new(10).splitter(Box::new(dictionary));
        let lines = wrap("Internationalization", &options).collect::<Vec<_>>();
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
        assert_iter_eq!(
            wrap("over-caffinated", &options),
            vec!["over-", "caffinated"]
        );

        let options = options.splitter(Box::new(dictionary));
        assert_iter_eq!(
            wrap("over-caffinated", &options),
            vec!["over-", "caffi-", "nated"]
        );
    }

    #[test]
    fn break_words() {
        assert_iter_eq!(wrap("foobarbaz", 3), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn break_words_wide_characters() {
        assert_iter_eq!(wrap("Ｈｅｌｌｏ", 5), vec!["Ｈｅ", "ｌｌ", "ｏ"]);
    }

    #[test]
    fn break_words_zero_width() {
        assert_iter_eq!(wrap("foobar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }

    #[test]
    fn break_words_line_breaks() {
        assert_eq!(fill("ab\ncdefghijkl", 5), "ab\ncdefg\nhijkl");
        assert_eq!(fill("abcdefgh\nijkl", 5), "abcde\nfgh\nijkl");
    }

    #[test]
    fn preserve_line_breaks() {
        assert_eq!(fill("test\n", 11), "test\n");
        assert_eq!(fill("test\n\na\n\n", 11), "test\n\na\n\n");
        assert_eq!(fill("1 3 5 7\n1 3 5 7", 7), "1 3 5 7\n1 3 5 7");
    }

    #[test]
    fn wrap_preserve_line_breaks() {
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
}
