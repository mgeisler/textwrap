//! `textwrap` provides functions for word wrapping and filling text.
//!
//! Wrapping text can be very useful in commandline programs where you
//! want to format dynamic output nicely so it looks good in a
//! terminal. A quick example:
//!
//! ```
//! extern crate textwrap;
//! use textwrap::fill;
//!
//! fn main() {
//!    let text = "textwrap: a small library for wrapping text.";
//!    println!("{}", fill(text, 18));
//! }
//! ```
//!
//! This will display the following output:
//!
//! ```text
//! textwrap: a small
//! library for
//! wrapping text.
//! ```
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
//! [unicode-width]: https://docs.rs/unicode-width/

#![deny(missing_docs)]

extern crate unicode_width;
extern crate term_size;
#[cfg(feature = "hyphenation")]
extern crate hyphenation;

use std::borrow::Cow;
use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;
#[cfg(feature = "hyphenation")]
use hyphenation::{Hyphenation, Corpus};

/// A non-breaking space.
const NBSP: char = '\u{a0}';

/// An interface for splitting words.
///
/// When the [`wrap`] method will try to fit text into a line, it will
/// eventually find a word that it too large the current text width.
/// It will then call the currently configured `WordSplitter` to have
/// it attempt to split the word into smaller parts. This trait
/// describes that functionality via the [`split`] method.
///
/// If the `textwrap` crate has been compiled with the `hyphenation`
/// feature enabled, you will find an implementation of `WordSplitter`
/// by the `hyphenation::language::Corpus` struct. Use this struct for
/// language-aware hyphenation. See the [`hyphenation` documentation]
/// for details.
///
/// [`wrap`]: struct.Wrapper.html#method.wrap
/// [`split`]: #tymethod.split
/// [`hyphenation` documentation]: https://docs.rs/hyphenation/
pub trait WordSplitter {
    /// Return all possible splits of word. Each split is a triple
    /// with a head, a hyphen, and a tail where `head + &hyphen +
    /// &tail == word`. The hyphen can be empty if there is already a
    /// hyphen in the head.
    ///
    /// The splits should go from smallest to longest and should
    /// include no split at all. So the word "technology" could be
    /// split into
    ///
    /// ```
    /// vec![("tech", "-", "nology"),
    ///      ("technol", "-", "ogy"),
    ///      ("technolo", "-", "gy"),
    ///      ("technology", "", "")];
    /// ```
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)>;
}

/// Use this as a [`Wrapper.splitter`] to avoid any kind of
/// hyphenation:
///
/// ```
/// use textwrap::{Wrapper, NoHyphenation};
///
/// let wrapper = Wrapper::new(8).word_splitter(Box::new(NoHyphenation));
/// assert_eq!(wrapper.wrap("foo bar-baz"), vec!["foo", "bar-baz"]);
/// ```
///
/// [`Wrapper.splitter`]: struct.Wrapper.html#structfield.splitter
pub struct NoHyphenation;

/// `NoHyphenation` implements `WordSplitter` by not splitting the
/// word at all.
impl WordSplitter for NoHyphenation {
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)> {
        vec![(word, "", "")]
    }
}

/// Simple and default way to split words: splitting on existing
/// hyphens only.
///
/// You probably don't need to use this type since it's already used
/// by default by `Wrapper::new`.
pub struct HyphenSplitter;

/// HyphenSplitter is the default WordSplitter used by `Wrapper::new`.
/// It will split words on any existing hyphens in the word.
///
/// It will only use hyphens that are surrounded by alphanumeric
/// characters, which prevents a word like "--foo-bar" from being
/// split on the first or second hyphen.
impl WordSplitter for HyphenSplitter {
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)> {
        let mut triples = Vec::new();
        // Split on hyphens, smallest split first. We only use hyphens
        // that are surrounded by alphanumeric characters. This is to
        // avoid splitting on repeated hyphens, such as those found in
        // --foo-bar.
        let mut char_indices = word.char_indices();
        // Early return if the word is empty.
        let mut prev = match char_indices.next() {
            None => return vec![(word, "", "")],
            Some((_, ch)) => ch,
        };

        // Find current word, or return early if the word only has a
        // single character.
        let (mut idx, mut cur) = match char_indices.next() {
            None => return vec![(word, "", "")],
            Some((idx, cur)) => (idx, cur),
        };

        while let Some((i, next)) = char_indices.next() {
            if prev.is_alphanumeric() && cur == '-' && next.is_alphanumeric() {
                let (head, tail) = word.split_at(idx + 1);
                triples.push((head, "", tail));
            }
            prev = cur;
            idx = i;
            cur = next;
        }

        // Finally option is no split at all.
        triples.push((word, "", ""));

        triples
    }
}

/// A hyphenation Corpus can be used to do language-specific
/// hyphenation using patterns from the hyphenation crate.
#[cfg(feature = "hyphenation")]
impl WordSplitter for Corpus {
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)> {
        // Find splits based on language corpus.
        let mut triples = Vec::new();
        for n in word.opportunities(&self) {
            let (head, tail) = word.split_at(n);
            let hyphen = if head.ends_with('-') { "" } else { "-" };
            triples.push((head, hyphen, tail));
        }
        // Finally option is no split at all.
        triples.push((word, "", ""));

        triples
    }
}

/// Backport of the AddAssign trait implementation from Rust 1.14.
fn cow_add_assign<'a>(lhs: &mut Cow<'a, str>, rhs: &'a str) {
    if lhs.is_empty() {
        *lhs = Cow::Borrowed(rhs)
    } else if rhs.is_empty() {
        return;
    } else {
        if let Cow::Borrowed(inner) = *lhs {
            let mut s = String::with_capacity(lhs.len() + rhs.len());
            s.push_str(inner);
            *lhs = Cow::Owned(s);
        }
        lhs.to_mut().push_str(rhs);
    }
}


/// A Wrapper holds settings for wrapping and filling text. Use it
/// when the convenience [`wrap`] and [`fill`] functions are not
/// flexible enough.
///
/// [`wrap`]: fn.wrap.html
/// [`fill`]: fn.fill.html
///
/// The algorithm used by the `wrap` method works by doing a single
/// scan over words in the input string and splitting them into one or
/// more lines. The time and memory complexity is O(*n*) where *n* is
/// the length of the input string.
pub struct Wrapper<'a> {
    /// The width in columns at which the text will be wrapped.
    pub width: usize,
    /// Indentation used for the first line of output.
    pub initial_indent: &'a str,
    /// Indentation used for subsequent lines of output.
    pub subsequent_indent: &'a str,
    /// Allow long words to be broken if they cannot fit on a line.
    /// When set to `false`, some lines be being longer than
    /// `self.width`.
    pub break_words: bool,
    /// The method for splitting words. If the `hyphenation` feature
    /// is enabled, you can use a `hyphenation::language::Corpus` here
    /// to get language-aware hyphenation.
    pub splitter: Box<WordSplitter>,
}

impl<'a> Wrapper<'a> {
    /// Create a new Wrapper for wrapping at the specified width. By
    /// default, we allow words longer than `width` to be broken. A
    /// [`HyphenSplitter`] will be used by default for splitting
    /// words. See the [`WordSplitter`] trait for other options.
    ///
    /// [`HyphenSplitter`]: struct.HyphenSplitter.html
    /// [`WordSplitter`]: trait.WordSplitter.html
    pub fn new(width: usize) -> Wrapper<'a> {
        Wrapper {
            width: width,
            initial_indent: "",
            subsequent_indent: "",
            break_words: true,
            splitter: Box::new(HyphenSplitter),
        }
    }

    /// Crate a new Wrapper for wrapping text at the current terminal
    /// width. If the terminal width cannot be determined (typically
    /// because the standard input and output is not connected to a
    /// terminal), a width of 80 characters will be used. Other
    /// settings use the same defaults as `Wrapper::new`.
    pub fn with_termwidth() -> Wrapper<'a> {
        Wrapper::new(term_size::dimensions_stdout().map_or(80, |(w, _)| w))
    }

    /// Change [`self.initial_indent`]. The initial indentation is
    /// used on the very first line of output. Setting it to something
    /// like `"* "` can be useful if you are formatting an item in a
    /// bulleted list. You will then probably want to set
    /// `self.subsequent_indent` to `"  "`.
    ///
    /// [`self.initial_indent`]: #structfield.initial_indent
    pub fn initial_indent(self, indent: &'a str) -> Wrapper<'a> {
        Wrapper { initial_indent: indent, ..self }
    }

    /// Change [`self.subsequent_indent`]. The subsequent indentation
    /// is used on lines following the first line of output. Setting
    /// it to something like `"  "` can be useful if you are
    /// formatting an item in a bulleted list.
    ///
    /// [`self.subsequent_indent`]: #structfield.subsequent_indent
    pub fn subsequent_indent(self, indent: &'a str) -> Wrapper<'a> {
        Wrapper { subsequent_indent: indent, ..self }
    }

    /// Change [`self.break_words`]. This controls if words longer
    /// than `self.width` can be broken, or if they will be left
    /// sticking out into the right margin.
    ///
    /// [`self.break_words`]: #structfield.break_words
    pub fn break_words(self, setting: bool) -> Wrapper<'a> {
        Wrapper { break_words: setting, ..self }
    }

    /// Change [`self.splitter`]. The word splitter is consulted when
    /// a word is too wide to fit the current line. By changing this,
    /// you can decide if such words should be hyphenated or left
    /// alone. Hyphenation can be done using existing hyphens (see
    /// [`HyphenSplitter`]) or it can be based on TeX hyphenation
    /// patterns, if the `hyphenation` feature is enabled.
    ///
    /// [`self.splitter`]: #structfield.splitter
    pub fn word_splitter(self, splitter: Box<WordSplitter>) -> Wrapper<'a> {
        Wrapper { splitter: splitter, ..self }
    }

    /// Fill a line of text at `self.width` characters. Strings are
    /// wrapped based on their displayed width, not their size in
    /// bytes.
    ///
    /// The result is a string with newlines between each line. Use
    /// the `wrap` method if you need access to the individual lines.
    ///
    /// ```
    /// use textwrap::Wrapper;
    ///
    /// let wrapper = Wrapper::new(15);
    /// assert_eq!(wrapper.fill("Memory safety without garbage collection."),
    ///            "Memory safety\nwithout garbage\ncollection.");
    /// ```
    ///
    /// This method simply joins the lines produced by `wrap`. As
    /// such, it inherits the O(*n*) time and memory complexity where
    /// *n* is the input string length.
    pub fn fill(&self, s: &str) -> String {
        self.wrap(s).join("\n")
    }

    /// Wrap a line of text at `self.width` characters. Strings are
    /// wrapped based on their displayed width, not their size in
    /// bytes.
    ///
    /// ```
    /// use textwrap::Wrapper;
    ///
    /// let wrap15 = Wrapper::new(15);
    /// assert_eq!(wrap15.wrap("Concurrency without data races."),
    ///            vec!["Concurrency",
    ///                 "without data",
    ///                 "races."]);
    ///
    /// let wrap20 = Wrapper::new(20);
    /// assert_eq!(wrap20.wrap("Concurrency without data races."),
    ///            vec!["Concurrency without",
    ///                 "data races."]);
    /// ```
    ///
    /// The [`WordSplitter`] stored in [`self.splitter`] is used
    /// whenever when a word is too large to fit on the current line.
    /// By changing the field, different hyphenation strategies can be
    /// implemented.
    ///
    /// This method does a single scan over the input string, it has
    /// an O(*n*) time and memory complexity where *n* is the input
    /// string length.
    ///
    /// [`self.splitter`]: #structfield.splitter
    /// [`WordSplitter`]: trait.WordSplitter.html
    ///
    pub fn wrap(&self, s: &'a str) -> Vec<Cow<'a, str>> {
        let mut lines = Vec::with_capacity(s.len() / (self.width + 1));
        // Byte index where the current line starts.
        let mut start = 0;
        // Byte index of the last place where the string can be split.
        let mut split = 0;
        // Size in bytes of the character at s[split].
        let mut split_len = 0;
        // Width of s[start..idx].
        let mut line_width = self.initial_indent.width();
        // Width of s[start..split].
        let mut line_width_at_split = line_width;
        // Tracking runs of whitespace characters.
        let mut in_whitespace = false;

        let mut line = Cow::from(self.initial_indent);

        for (idx, ch) in s.char_indices() {
            let char_width = ch.width().unwrap_or(0);
            let char_len = ch.len_utf8();
            if ch.is_whitespace() && ch != NBSP {
                // Extend the previous split or create a new one.
                if in_whitespace {
                    split_len += char_len;
                } else {
                    split = idx;
                    split_len = char_len;
                }
                line_width_at_split = line_width + char_width;
                in_whitespace = true;
            } else if line_width + char_width > self.width {
                // There is no room for this character on the current
                // line. Try to split the final word.
                let remaining_text = &s[split + split_len..];
                let final_word = match remaining_text
                          .find(|ch: char| ch.is_whitespace() && ch != NBSP) {
                    Some(i) => &remaining_text[..i],
                    None => remaining_text,
                };

                let mut hyphen = "";
                let splits = self.splitter.split(final_word);
                for &(head, hyp, _) in splits.iter().rev() {
                    if line_width_at_split + head.width() + hyp.width() <= self.width {
                        split += head.len();
                        split_len = 0;
                        hyphen = hyp;
                        break;
                    }
                }

                if start >= split {
                    // The word is too big to fit on a single line, so we
                    // need to split it at the current index.
                    if self.break_words {
                        // Break work at current index.
                        split = idx;
                        split_len = 0;
                        line_width_at_split = line_width;
                    } else {
                        // Add smallest split.
                        split = start + splits[0].0.len();
                        split_len = 0;
                        line_width_at_split = line_width;
                    }
                }

                if start < split {
                    cow_add_assign(&mut line, &s[start..split]);
                    cow_add_assign(&mut line, hyphen);
                    lines.push(line);
                    line = Cow::from(self.subsequent_indent);
                    start = split + split_len;
                    line_width += self.subsequent_indent.width();
                    line_width -= line_width_at_split;
                }
            } else {
                in_whitespace = false;
            }
            line_width += char_width;
        }

        // Add final line.
        if start < s.len() {
            cow_add_assign(&mut line, &s[start..]);
            lines.push(line);
        }

        lines
    }
}

/// Fill a line of text at `width` characters. Strings are wrapped
/// based on their displayed width, not their size in bytes.
///
/// The result is a string with newlines between each line. Use `wrap`
/// if you need access to the individual lines.
///
/// ```
/// use textwrap::fill;
///
/// assert_eq!(fill("Memory safety without garbage collection.", 15),
///            "Memory safety\nwithout garbage\ncollection.");
/// ```
///
/// This function creates a Wrapper on the fly with default settings.
/// If you need to set a language corpus for automatic hyphenation, or
/// need to fill many strings, then it is suggested to create Wrapper
/// and call its [`fill` method](struct.Wrapper.html#method.fill).
pub fn fill(s: &str, width: usize) -> String {
    wrap(s, width).join("\n")
}

/// Wrap a line of text at `width` characters. Strings are wrapped
/// based on their displayed width, not their size in bytes.
///
/// ```
/// use textwrap::wrap;
///
/// assert_eq!(wrap("Concurrency without data races.", 15),
///            vec!["Concurrency",
///                 "without data",
///                 "races."]);
///
/// assert_eq!(wrap("Concurrency without data races.", 20),
///            vec!["Concurrency without",
///                 "data races."]);
/// ```
///
/// This function creates a Wrapper on the fly with default settings.
/// If you need to set a language corpus for automatic hyphenation, or
/// need to wrap many strings, then it is suggested to create Wrapper
/// and call its [`wrap` method](struct.Wrapper.html#method.wrap).
pub fn wrap<'a>(s: &'a str, width: usize) -> Vec<Cow<'a, str>> {
    Wrapper::new(width).wrap(s)
}

/// Add prefix to each non-empty line.
///
/// ```
/// use textwrap::indent;
///
/// assert_eq!(indent("Foo\nBar\n", "  "), "  Foo\n  Bar\n");
/// ```
///
/// Empty lines (lines consisting only of whitespace) are not indented
/// and the whitespace is replaced by a single newline (`\n`):
///
/// ```
/// use textwrap::indent;
///
/// assert_eq!(indent("Foo\n\nBar\n  \t  \nBaz\n", "  "),
///            "  Foo\n\n  Bar\n\n  Baz\n");
/// ```
///
/// Leading and trailing whitespace on non-empty lines is kept
/// unchanged:
///
/// ```
/// use textwrap::indent;
///
/// assert_eq!(indent(" \t  Foo   ", "  "), "   \t  Foo   \n");
/// ```
pub fn indent(s: &str, prefix: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        if line.chars().any(|c| !c.is_whitespace()) {
            result.push_str(prefix);
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Removes common leading whitespace from each line.
///
/// This will look at each non-empty line and determine the maximum
/// amount of whitespace that can be removed from the line.
///
/// ```
/// use textwrap::dedent;
///
/// assert_eq!(dedent("  1st line\n  2nd line\n"),
///            "1st line\n2nd line\n");
/// ```
pub fn dedent(s: &str) -> String {
    let mut prefix = String::new();
    let mut lines = s.lines();

    // We first search for a non-empty line to find a prefix.
    for line in &mut lines {
        let whitespace = line.chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        // Check if the line had anything but whitespace
        if whitespace.len() < line.len() {
            prefix = whitespace;
            break;
        }
    }

    // We then continue looking through the remaining lines to
    // possibly shorten the prefix.
    for line in &mut lines {
        let whitespace = line.chars()
            .zip(prefix.chars())
            .take_while(|&(a, b)| a == b)
            .map(|(_, b)| b)
            .collect::<String>();
        // Check if we have found a shorter prefix
        if whitespace.len() < prefix.len() {
            prefix = whitespace;
        }
    }

    // We now go over the lines a second time to build the result.
    let mut result = String::new();
    for line in s.lines() {
        if line.starts_with(&prefix) && line.chars().any(|c| !c.is_whitespace()) {
            let (_, tail) = line.split_at(prefix.len());
            result.push_str(tail);
        }
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "hyphenation")]
    extern crate hyphenation;

    #[cfg(feature = "hyphenation")]
    use hyphenation::Language;
    use super::*;
    use std::borrow::Cow;

    /// Add newlines. Ensures that the final line in the vector also
    /// has a newline.
    fn add_nl(lines: &Vec<&str>) -> String {
        lines.join("\n") + "\n"
    }

    #[test]
    fn no_wrap() {
        assert_eq!(wrap("foo", 10), vec!["foo"]);
    }

    #[test]
    fn simple() {
        assert_eq!(wrap("foo bar baz", 5), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn multi_word_on_line() {
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
    fn leading_whitespace() {
        assert_eq!(wrap("  foo bar", 6), vec!["  foo", "bar"]);
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(wrap("foo bar  ", 6), vec!["foo", "bar  "]);
    }

    #[test]
    fn interior_whitespace() {
        assert_eq!(wrap("foo:   bar baz", 10), vec!["foo:   bar", "baz"]);
    }

    #[test]
    fn extra_whitespace_start_of_line() {
        // Whitespace is only significant inside a line. After a line
        // gets too long and is broken, the first word starts in
        // column zero and is not indented. The line before might end
        // up with trailing whitespace.
        assert_eq!(wrap("foo               bar", 5), vec!["foo", "bar"]);
    }

    #[test]
    fn wide_character_handling() {
        assert_eq!(wrap("Hello, World!", 15), vec!["Hello, World!"]);
        assert_eq!(wrap("Ｈｅｌｌｏ, Ｗｏｒｌｄ!", 15),
                   vec!["Ｈｅｌｌｏ,", "Ｗｏｒｌｄ!"]);
    }

    #[test]
    fn indent_empty() {
        let wrapper = Wrapper::new(10).initial_indent("!!!");
        assert_eq!(wrapper.fill(""), "");
    }

    #[test]
    fn indent_single_line() {
        let wrapper = Wrapper::new(10).initial_indent(">>>"); // No trailing space
        assert_eq!(wrapper.fill("foo"), ">>>foo");
    }

    #[test]
    fn indent_multiple_lines() {
        let wrapper = Wrapper::new(6).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(wrapper.wrap("foo bar baz"), vec!["* foo", "  bar", "  baz"]);
    }

    #[test]
    fn indent_break_words() {
        let wrapper = Wrapper::new(5).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(wrapper.wrap("foobarbaz"), vec!["* foo", "  bar", "  baz"]);
    }

    #[test]
    fn hyphens() {
        assert_eq!(wrap("foo-bar", 5), vec!["foo-", "bar"]);
    }

    #[test]
    fn trailing_hyphen() {
        let wrapper = Wrapper::new(5).break_words(false);
        assert_eq!(wrapper.wrap("foobar-"), vec!["foobar-"]);
    }

    #[test]
    fn multiple_hyphens() {
        assert_eq!(wrap("foo-bar-baz", 5), vec!["foo-", "bar-", "baz"]);
    }

    #[test]
    fn hyphens_flag() {
        let wrapper = Wrapper::new(5).break_words(false);
        assert_eq!(wrapper.wrap("The --foo-bar flag."),
                   vec!["The", "--foo-", "bar", "flag."]);
    }

    #[test]
    fn repeated_hyphens() {
        let wrapper = Wrapper::new(4).break_words(false);
        assert_eq!(wrapper.wrap("foo--bar"), vec!["foo--bar"]);
    }

    #[test]
    fn hyphens_alphanumeric() {
        assert_eq!(wrap("Na2-CH4", 5), vec!["Na2-", "CH4"]);
    }

    #[test]
    fn hyphens_non_alphanumeric() {
        let wrapper = Wrapper::new(5).break_words(false);
        assert_eq!(wrapper.wrap("foo(-)bar"), vec!["foo(-)bar"]);
    }

    #[test]
    fn multiple_splits() {
        assert_eq!(wrap("foo-bar-baz", 9), vec!["foo-bar-", "baz"]);
    }

    #[test]
    fn forced_split() {
        let wrapper = Wrapper::new(5).break_words(false);
        assert_eq!(wrapper.wrap("foobar-baz"), vec!["foobar-", "baz"]);
    }

    #[test]
    fn no_hyphenation() {
        let wrapper = Wrapper::new(8).word_splitter(Box::new(NoHyphenation));
        assert_eq!(wrapper.wrap("foo bar-baz"), vec!["foo", "bar-baz"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation() {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let wrapper = Wrapper::new(10);
        assert_eq!(wrapper.wrap("Internationalization"),
                   vec!["Internatio", "nalization"]);

        let wrapper = wrapper.word_splitter(Box::new(corpus));
        assert_eq!(wrapper.wrap("Internationalization"),
                   vec!["Interna-", "tionaliza-", "tion"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn borrowed_lines() {
        // Lines that end with an extra hyphen are owned, the final
        // line is borrowed.
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let wrapper = Wrapper::new(10).word_splitter(Box::new(corpus));
        let lines = wrapper.wrap("Internationalization");
        if let Cow::Borrowed(s) = lines[0] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Cow::Borrowed(s) = lines[1] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Cow::Owned(ref s) = lines[2] {
            assert!(false, "should not have been owned: {:?}", s);
        }
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_with_hyphen() {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let wrapper = Wrapper::new(8).break_words(false);
        assert_eq!(wrapper.wrap("over-caffinated"), vec!["over-", "caffinated"]);

        let wrapper = wrapper.word_splitter(Box::new(corpus));
        assert_eq!(wrapper.wrap("over-caffinated"),
                   vec!["over-", "caffi-", "nated"]);
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
    fn test_non_breaking_space() {
        let wrapper = Wrapper::new(5).break_words(false);
        assert_eq!(wrapper.fill("foo bar baz"), "foo bar baz");
    }

    #[test]
    fn test_non_breaking_hyphen() {
        let wrapper = Wrapper::new(5).break_words(false);
        assert_eq!(wrapper.fill("foo‑bar‑baz"), "foo‑bar‑baz");
    }

    #[test]
    fn test_fill() {
        assert_eq!(fill("foo bar baz", 10), "foo bar\nbaz");
    }

    #[test]
    fn test_indent_empty() {
        assert_eq!(indent("\n", "  "), "\n");
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_indent_nonempty() {
        let x = vec!["  foo",
                     "bar",
                     "  baz"];
        let y = vec!["//  foo",
                     "//bar",
                     "//  baz"];
        assert_eq!(indent(&add_nl(&x), "//"), add_nl(&y));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_indent_empty_line() {
        let x = vec!["  foo",
                     "bar",
                     "",
                     "  baz"];
        let y = vec!["//  foo",
                     "//bar",
                     "",
                     "//  baz"];
        assert_eq!(indent(&add_nl(&x), "//"), add_nl(&y));
    }

    #[test]
    fn test_dedent_empty() {
        assert_eq!(dedent(""), "");
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dedent_multi_line() {
        let x = vec!["    foo",
                     "  bar",
                     "    baz"];
        let y = vec!["  foo",
                     "bar",
                     "  baz"];
        assert_eq!(dedent(&add_nl(&x)), add_nl(&y));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dedent_empty_line() {
        let x = vec!["    foo",
                     "  bar",
                     "   ",
                     "    baz"];
        let y = vec!["  foo",
                     "bar",
                     "",
                     "  baz"];
        assert_eq!(dedent(&add_nl(&x)), add_nl(&y));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dedent_mixed_whitespace() {
        let x = vec!["\tfoo",
                     "  bar"];
        let y = vec!["\tfoo",
                     "  bar"];
        assert_eq!(dedent(&add_nl(&x)), add_nl(&y));
    }
}
